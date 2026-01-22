//! WebSocket connection management with auto-reconnect
//!
//! Provides a framework-agnostic connection builder and manager that uses
//! callbacks for state updates. Framework-specific adapters can wrap this
//! to integrate with their reactive systems.

use std::cell::RefCell;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CloseEvent, MessageEvent, WebSocket};

use crate::messages::{ClientMessage, ServerMessage};
use crate::operation::{ActionError, ActionProgress, OpId};
use crate::status::{CloseInfo, ConnectionStatus};

/// Configuration for reconnection behavior
#[derive(Debug, Clone)]
pub struct ReconnectConfig {
    /// Base delay in ms (doubled each attempt)
    pub base_delay_ms: u32,
    /// Maximum delay between attempts
    pub max_delay_ms: u32,
    /// Maximum number of attempts (None = infinite)
    pub max_attempts: Option<u32>,
    /// Ping interval in ms (0 = disabled)
    pub ping_interval_ms: u32,
}

impl Default for ReconnectConfig {
    fn default() -> Self {
        Self {
            base_delay_ms: 1000,
            max_delay_ms: 30000,
            max_attempts: None,
            ping_interval_ms: 30000,
        }
    }
}

/// Builder for creating a Flow connection
///
/// # Example
///
/// ```ignore
/// let connection = FlowConnection::<GameState, GameAction>::builder()
///     .url(&ws_url)
///     .on_snapshot(|state| { /* update UI */ })
///     .on_delta(|delta| { /* apply change */ })
///     .on_status(|status| { /* update status indicator */ })
///     .connect();
/// ```
pub struct FlowConnectionBuilder<State, Delta, Action> {
    url: String,
    reconnect_config: ReconnectConfig,
    on_snapshot: Option<Rc<dyn Fn(State, u64)>>,
    on_delta: Option<Rc<dyn Fn(Delta, u64)>>,
    on_status: Option<Rc<dyn Fn(ConnectionStatus)>>,
    on_progress: Option<Rc<dyn Fn(ActionProgress)>>,
    on_action_complete: Option<Rc<dyn Fn(OpId)>>,
    on_action_error: Option<Rc<dyn Fn(ActionError)>>,
    on_error: Option<Rc<dyn Fn(String)>>,
    _action: std::marker::PhantomData<Action>,
}

impl<State, Delta, Action> FlowConnectionBuilder<State, Delta, Action>
where
    State: DeserializeOwned + 'static,
    Delta: DeserializeOwned + 'static,
    Action: Serialize + 'static,
{
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            url: String::new(),
            reconnect_config: ReconnectConfig::default(),
            on_snapshot: None,
            on_delta: None,
            on_status: None,
            on_progress: None,
            on_action_complete: None,
            on_action_error: None,
            on_error: None,
            _action: std::marker::PhantomData,
        }
    }

    /// Set the WebSocket URL
    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    /// Configure reconnection behavior
    pub fn reconnect_config(mut self, config: ReconnectConfig) -> Self {
        self.reconnect_config = config;
        self
    }

    /// Callback when a full snapshot is received
    pub fn on_snapshot<F>(mut self, f: F) -> Self
    where
        F: Fn(State, u64) + 'static,
    {
        self.on_snapshot = Some(Rc::new(f));
        self
    }

    /// Callback when a delta is received
    pub fn on_delta<F>(mut self, f: F) -> Self
    where
        F: Fn(Delta, u64) + 'static,
    {
        self.on_delta = Some(Rc::new(f));
        self
    }

    /// Callback when connection status changes
    pub fn on_status<F>(mut self, f: F) -> Self
    where
        F: Fn(ConnectionStatus) + 'static,
    {
        self.on_status = Some(Rc::new(f));
        self
    }

    /// Callback for action progress updates
    pub fn on_progress<F>(mut self, f: F) -> Self
    where
        F: Fn(ActionProgress) + 'static,
    {
        self.on_progress = Some(Rc::new(f));
        self
    }

    /// Callback when an action completes successfully
    pub fn on_action_complete<F>(mut self, f: F) -> Self
    where
        F: Fn(OpId) + 'static,
    {
        self.on_action_complete = Some(Rc::new(f));
        self
    }

    /// Callback when an action fails
    pub fn on_action_error<F>(mut self, f: F) -> Self
    where
        F: Fn(ActionError) + 'static,
    {
        self.on_action_error = Some(Rc::new(f));
        self
    }

    /// Callback for connection errors
    pub fn on_error<F>(mut self, f: F) -> Self
    where
        F: Fn(String) + 'static,
    {
        self.on_error = Some(Rc::new(f));
        self
    }

    /// Build and connect
    pub fn connect(self) -> Result<FlowConnection<Action>, FlowError> {
        if self.url.is_empty() {
            return Err(FlowError::Configuration("URL is required".into()));
        }

        FlowConnection::connect_internal(
            self.url,
            self.reconnect_config,
            self.on_snapshot,
            self.on_delta,
            self.on_status,
            self.on_progress,
            self.on_action_complete,
            self.on_action_error,
            self.on_error,
        )
    }
}

impl<State, Delta, Action> Default for FlowConnectionBuilder<State, Delta, Action>
where
    State: DeserializeOwned + 'static,
    Delta: DeserializeOwned + 'static,
    Action: Serialize + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Active WebSocket connection with Flow protocol
pub struct FlowConnection<Action> {
    inner: Rc<RefCell<ConnectionInner<Action>>>,
}

struct ConnectionInner<Action> {
    ws: Option<WebSocket>,
    #[allow(dead_code)] // Used for reconnection
    url: String,
    status: ConnectionStatus,
    reconnect_config: ReconnectConfig,
    reconnect_attempt: u32,
    current_seq: u64,
    // Store closures to prevent them from being dropped
    _closures: Vec<Closure<dyn FnMut(JsValue)>>,
    _action: std::marker::PhantomData<Action>,
}

impl<Action> FlowConnection<Action>
where
    Action: Serialize + 'static,
{
    /// Create a builder for configuring a connection
    pub fn builder<State, Delta>() -> FlowConnectionBuilder<State, Delta, Action>
    where
        State: DeserializeOwned + 'static,
        Delta: DeserializeOwned + 'static,
    {
        FlowConnectionBuilder::new()
    }

    fn connect_internal<State, Delta>(
        url: String,
        reconnect_config: ReconnectConfig,
        on_snapshot: Option<Rc<dyn Fn(State, u64)>>,
        on_delta: Option<Rc<dyn Fn(Delta, u64)>>,
        on_status: Option<Rc<dyn Fn(ConnectionStatus)>>,
        on_progress: Option<Rc<dyn Fn(ActionProgress)>>,
        on_action_complete: Option<Rc<dyn Fn(OpId)>>,
        on_action_error: Option<Rc<dyn Fn(ActionError)>>,
        on_error: Option<Rc<dyn Fn(String)>>,
    ) -> Result<Self, FlowError>
    where
        State: DeserializeOwned + 'static,
        Delta: DeserializeOwned + 'static,
    {
        let ws = WebSocket::new(&url)
            .map_err(|e| FlowError::Connection(format!("Failed to create WebSocket: {:?}", e)))?;

        // Use text mode for JSON messages
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let inner = Rc::new(RefCell::new(ConnectionInner {
            ws: Some(ws.clone()),
            url: url.clone(),
            status: ConnectionStatus::Connecting,
            reconnect_config,
            reconnect_attempt: 0,
            current_seq: 0,
            _closures: Vec::new(),
            _action: std::marker::PhantomData,
        }));

        // Notify initial status
        if let Some(ref cb) = on_status {
            cb(ConnectionStatus::Connecting);
        }

        // Set up event handlers
        let mut closures = Vec::new();

        // onopen
        {
            let on_status = on_status.clone();
            let inner = inner.clone();
            let ping_interval = inner.borrow().reconnect_config.ping_interval_ms;

            let onopen = Closure::wrap(Box::new(move |_: JsValue| {
                tracing::info!("WebSocket connected");
                {
                    let mut inner = inner.borrow_mut();
                    inner.status = ConnectionStatus::Connected;
                    inner.reconnect_attempt = 0;
                }

                if let Some(ref cb) = on_status {
                    cb(ConnectionStatus::Connected);
                }

                // Start ping timer if configured
                if ping_interval > 0 {
                    let inner_ping = inner.clone();
                    start_ping_timer(inner_ping, ping_interval);
                }
            }) as Box<dyn FnMut(JsValue)>);

            ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
            closures.push(onopen);
        }

        // onmessage
        {
            let on_snapshot = on_snapshot.clone();
            let on_delta = on_delta.clone();
            let on_progress = on_progress.clone();
            let on_action_complete = on_action_complete.clone();
            let on_action_error = on_action_error.clone();
            let on_error = on_error.clone();
            let inner = inner.clone();

            let onmessage = Closure::wrap(Box::new(move |event: JsValue| {
                let event: MessageEvent = event.unchecked_into();

                let text = match event.data().as_string() {
                    Some(t) => t,
                    None => {
                        tracing::warn!("Received non-text WebSocket message");
                        return;
                    }
                };

                match serde_json::from_str::<ServerMessage<State, Delta>>(&text) {
                    Ok(msg) => {
                        handle_server_message(
                            msg,
                            &inner,
                            &on_snapshot,
                            &on_delta,
                            &on_progress,
                            &on_action_complete,
                            &on_action_error,
                        );
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse server message: {} - {}", e, text);
                        if let Some(ref cb) = on_error {
                            cb(format!("Parse error: {e}"));
                        }
                    }
                }
            }) as Box<dyn FnMut(JsValue)>);

            ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            closures.push(onmessage);
        }

        // onclose
        {
            let on_status = on_status.clone();
            let on_error = on_error.clone();
            let inner = inner.clone();
            let url = url.clone();

            let onclose = Closure::wrap(Box::new(move |event: JsValue| {
                let event: CloseEvent = event.unchecked_into();
                let close_info = CloseInfo {
                    code: event.code(),
                    reason: event.reason(),
                };

                tracing::info!(
                    "WebSocket closed: code={}, reason={}",
                    close_info.code,
                    close_info.reason
                );

                let should_reconnect = {
                    let mut inner = inner.borrow_mut();
                    inner.ws = None;

                    if close_info.is_auth_failure() {
                        inner.status = ConnectionStatus::AuthFailed;
                        if let Some(ref cb) = on_status {
                            cb(ConnectionStatus::AuthFailed);
                        }
                        false
                    } else {
                        let attempt = inner.reconnect_attempt + 1;
                        let max_attempts = inner.reconnect_config.max_attempts;
                        let should = max_attempts.map(|m| attempt <= m).unwrap_or(true);

                        if should {
                            inner.status = ConnectionStatus::Reconnecting { attempt };
                            inner.reconnect_attempt = attempt;
                            if let Some(ref cb) = on_status {
                                cb(ConnectionStatus::Reconnecting { attempt });
                            }
                        } else {
                            inner.status = ConnectionStatus::Disconnected;
                            if let Some(ref cb) = on_status {
                                cb(ConnectionStatus::Disconnected);
                            }
                        }
                        should
                    }
                };

                if should_reconnect {
                    let delay = {
                        let inner = inner.borrow();
                        calculate_backoff(inner.reconnect_attempt, &inner.reconnect_config)
                    };

                    tracing::info!("Scheduling reconnection in {}ms", delay);

                    let inner = inner.clone();
                    let url = url.clone();
                    let on_status = on_status.clone();
                    let on_error = on_error.clone();

                    wasm_bindgen_futures::spawn_local(async move {
                        gloo_timers::future::TimeoutFuture::new(delay).await;
                        if let Err(e) = reconnect(&inner, &url, &on_status, &on_error) {
                            tracing::error!("Reconnection failed: {}", e);
                        }
                    });
                }
            }) as Box<dyn FnMut(JsValue)>);

            ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
            closures.push(onclose);
        }

        // onerror
        {
            let on_error = on_error.clone();

            let onerror = Closure::wrap(Box::new(move |_event: JsValue| {
                tracing::error!("WebSocket error");
                if let Some(ref cb) = on_error {
                    cb("WebSocket error".into());
                }
            }) as Box<dyn FnMut(JsValue)>);

            ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
            closures.push(onerror);
        }

        // Store closures to prevent drop
        inner.borrow_mut()._closures = closures;

        Ok(Self { inner })
    }

    /// Get current connection status
    pub fn status(&self) -> ConnectionStatus {
        self.inner.borrow().status
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.inner.borrow().status.is_connected()
    }

    /// Get current sequence number
    pub fn current_seq(&self) -> u64 {
        self.inner.borrow().current_seq
    }

    /// Send an action to the server
    pub fn send_action(&self, op_id: OpId, action: Action) -> Result<(), FlowError> {
        let msg = ClientMessage::Action { op_id, action };
        self.send_message(&msg)
    }

    /// Request state synchronization
    pub fn sync_state(&self, from_seq: Option<u64>) -> Result<(), FlowError> {
        let msg: ClientMessage<Action> = ClientMessage::SyncState { from_seq };
        self.send_message(&msg)
    }

    /// Subscribe to a topic
    pub fn subscribe(&self, topic: &str) -> Result<(), FlowError> {
        let msg: ClientMessage<Action> = ClientMessage::Subscribe {
            topic: topic.to_string(),
        };
        self.send_message(&msg)
    }

    /// Unsubscribe from a topic
    pub fn unsubscribe(&self, topic: &str) -> Result<(), FlowError> {
        let msg: ClientMessage<Action> = ClientMessage::Unsubscribe {
            topic: topic.to_string(),
        };
        self.send_message(&msg)
    }

    /// Send a ping
    pub fn send_ping(&self) -> Result<(), FlowError> {
        let msg: ClientMessage<Action> = ClientMessage::Ping;
        self.send_message(&msg)
    }

    /// Disconnect and clean up
    pub fn disconnect(&self) {
        let mut inner = self.inner.borrow_mut();
        if let Some(ref ws) = inner.ws {
            let _ = ws.close();
        }
        inner.ws = None;
        inner.status = ConnectionStatus::Disconnected;
        inner.reconnect_attempt = 0;
    }

    fn send_message<M: Serialize>(&self, msg: &M) -> Result<(), FlowError> {
        let inner = self.inner.borrow();
        let ws = inner.ws.as_ref().ok_or(FlowError::NotConnected)?;

        let json =
            serde_json::to_string(msg).map_err(|e| FlowError::Serialization(e.to_string()))?;

        ws.send_with_str(&json)
            .map_err(|e| FlowError::Send(format!("{:?}", e)))
    }
}

fn handle_server_message<State, Delta, Action>(
    msg: ServerMessage<State, Delta>,
    inner: &Rc<RefCell<ConnectionInner<Action>>>,
    on_snapshot: &Option<Rc<dyn Fn(State, u64)>>,
    on_delta: &Option<Rc<dyn Fn(Delta, u64)>>,
    on_progress: &Option<Rc<dyn Fn(ActionProgress)>>,
    on_action_complete: &Option<Rc<dyn Fn(OpId)>>,
    on_action_error: &Option<Rc<dyn Fn(ActionError)>>,
) {
    match msg {
        ServerMessage::Connected => {
            tracing::debug!("Server acknowledged connection");
        }
        ServerMessage::Pong => {
            tracing::trace!("Received pong");
        }
        ServerMessage::Snapshot { state, seq } => {
            inner.borrow_mut().current_seq = seq;
            if let Some(ref cb) = on_snapshot {
                cb(state, seq);
            }
        }
        ServerMessage::Delta { delta, seq } => {
            inner.borrow_mut().current_seq = seq;
            if let Some(ref cb) = on_delta {
                cb(delta, seq);
            }
        }
        ServerMessage::Progress(progress) => {
            if let Some(ref cb) = on_progress {
                cb(progress);
            }
        }
        ServerMessage::ActionComplete { op_id } => {
            if let Some(ref cb) = on_action_complete {
                cb(op_id);
            }
        }
        ServerMessage::ActionError(error) => {
            if let Some(ref cb) = on_action_error {
                cb(error);
            }
        }
        ServerMessage::Error { message } => {
            tracing::error!("Server error: {}", message);
        }
    }
}

fn calculate_backoff(attempt: u32, config: &ReconnectConfig) -> u32 {
    let multiplier = 2u32.saturating_pow(attempt.saturating_sub(1));
    let delay = config.base_delay_ms.saturating_mul(multiplier);
    delay.min(config.max_delay_ms)
}

fn reconnect<Action>(
    inner: &Rc<RefCell<ConnectionInner<Action>>>,
    url: &str,
    on_status: &Option<Rc<dyn Fn(ConnectionStatus)>>,
    _on_error: &Option<Rc<dyn Fn(String)>>,
) -> Result<(), FlowError> {
    tracing::info!("Attempting reconnection to {}", url);

    let ws = WebSocket::new(url)
        .map_err(|e| FlowError::Connection(format!("Failed to create WebSocket: {:?}", e)))?;

    // Note: In a full implementation, we'd need to re-attach all the handlers
    // For now, this is a simplified version
    inner.borrow_mut().ws = Some(ws);

    if let Some(ref cb) = on_status {
        cb(ConnectionStatus::Connecting);
    }

    Ok(())
}

fn start_ping_timer<Action: 'static>(
    inner: Rc<RefCell<ConnectionInner<Action>>>,
    interval_ms: u32,
) {
    wasm_bindgen_futures::spawn_local(async move {
        loop {
            gloo_timers::future::TimeoutFuture::new(interval_ms).await;

            let should_ping = {
                let inner = inner.borrow();
                inner.status.is_connected() && inner.ws.is_some()
            };

            if should_ping {
                let result = {
                    let inner = inner.borrow();
                    if let Some(ref ws) = inner.ws {
                        let msg: ClientMessage<()> = ClientMessage::Ping;
                        serde_json::to_string(&msg)
                            .ok()
                            .and_then(|json| ws.send_with_str(&json).ok())
                    } else {
                        None
                    }
                };

                if result.is_none() {
                    break;
                }
            } else {
                break;
            }
        }
    });
}

/// Errors that can occur with Flow connections
#[derive(Debug, thiserror::Error)]
pub enum FlowError {
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Not connected")]
    NotConnected,
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Send error: {0}")]
    Send(String),
}
