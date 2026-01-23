//! Durable Object for managing WebSocket sessions.
//!
//! This implements the server-side of the unified realtime protocol,
//! handling WebSocket connections, state management, and broadcasting.

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use ui_flow_protocol::{encode, OpId, PresenceInfo, PresenceStatus, ServerMessage};
use worker::*;

/// Connection information stored as WebSocket attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConnectionInfo {
    user_id: String,
    user_name: String,
    connected_at: u64,
}

/// The Durable Object that manages a single room's state and connections.
#[durable_object]
pub struct FlowDemoSessionDO {
    state: State,
    env: Env,
    /// Current room state
    room_state: RefCell<DemoState>,
    /// Current sequence number
    seq: RefCell<u64>,
    /// Message ID counter
    next_message_id: RefCell<u64>,
}

impl DurableObject for FlowDemoSessionDO {
    fn new(state: State, env: Env) -> Self {
        Self {
            state,
            env,
            room_state: RefCell::new(DemoState::default()),
            seq: RefCell::new(0),
            next_message_id: RefCell::new(1),
        }
    }

    async fn fetch(&self, req: Request) -> Result<Response> {
        // Handle WebSocket upgrade
        if req.headers().get("Upgrade")?.as_deref() == Some("websocket") {
            return self.handle_websocket_upgrade(req).await;
        }

        Response::error("Expected WebSocket upgrade", 400)
    }

    async fn websocket_message(
        &self,
        ws: WebSocket,
        message: WebSocketIncomingMessage,
    ) -> Result<()> {
        let bytes = match message {
            WebSocketIncomingMessage::Binary(bytes) => bytes,
            WebSocketIncomingMessage::String(text) => text.into_bytes(),
        };

        // Decode the client message
        let client_msg: ClientMsg = match ui_flow_protocol::decode(&bytes) {
            Ok(msg) => msg,
            Err(e) => {
                let error_msg: ServerMsg =
                    ServerMessage::error(format!("Failed to decode message: {e}"), false);
                if let Ok(bytes) = encode(&error_msg) {
                    let _ = ws.send_with_bytes(&bytes);
                }
                return Ok(());
            }
        };

        // Get connection info
        let conn_info: Option<ConnectionInfo> = ws.deserialize_attachment().ok().flatten();
        let conn = conn_info.unwrap_or_else(|| ConnectionInfo {
            user_id: "anonymous".to_string(),
            user_name: "Anonymous".to_string(),
            connected_at: 0,
        });

        // Handle the message
        self.handle_client_message(&ws, &conn, client_msg).await?;

        Ok(())
    }

    async fn websocket_close(
        &self,
        ws: WebSocket,
        _code: usize,
        _reason: String,
        _was_clean: bool,
    ) -> Result<()> {
        // Get connection info
        if let Ok(Some(conn)) = ws.deserialize_attachment::<ConnectionInfo>() {
            // Broadcast user left delta
            let delta = DemoDelta::UserLeft {
                user_id: conn.user_id.clone(),
            };
            self.broadcast_delta(delta).await;

            // Broadcast updated presence
            self.broadcast_presence().await;
        }

        Ok(())
    }
}

impl FlowDemoSessionDO {
    /// Handle WebSocket upgrade request
    async fn handle_websocket_upgrade(&self, req: Request) -> Result<Response> {
        // Extract user info from query params (in production, use JWT)
        let url = req.url()?;
        let user_id = url
            .query_pairs()
            .find(|(k, _)| k == "user_id")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| format!("user_{}", rand::random::<u32>()));
        let user_name = url
            .query_pairs()
            .find(|(k, _)| k == "user_name")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| format!("User {}", &user_id[..8.min(user_id.len())]));

        // Create WebSocket pair
        let WebSocketPair { client, server } = WebSocketPair::new()?;

        // Store connection info as attachment
        let conn_info = ConnectionInfo {
            user_id: user_id.clone(),
            user_name: user_name.clone(),
            connected_at: now(),
        };
        server.serialize_attachment(&conn_info)?;

        // Accept the WebSocket with hibernation API
        self.state.accept_web_socket(&server);

        // Send Connected message
        let connected_msg: ServerMsg = ServerMessage::connected(1, self.state.id().to_string());
        if let Ok(bytes) = encode(&connected_msg) {
            let _ = server.send_with_bytes(&bytes);
        }

        // Send current state snapshot
        let snapshot_msg: ServerMsg =
            ServerMessage::snapshot(self.room_state.borrow().clone(), *self.seq.borrow(), now());
        if let Ok(bytes) = encode(&snapshot_msg) {
            let _ = server.send_with_bytes(&bytes);
        }

        // Broadcast user joined delta to all clients
        let delta = DemoDelta::UserJoined { user_id, user_name };
        self.broadcast_delta(delta).await;

        // Broadcast updated presence
        self.broadcast_presence().await;

        Response::from_websocket(client)
    }

    /// Handle a client message
    async fn handle_client_message(
        &self,
        ws: &WebSocket,
        conn: &ConnectionInfo,
        msg: ClientMsg,
    ) -> Result<()> {
        use ui_flow_protocol::ClientMessage;

        match msg {
            ClientMessage::Ping { ts } => {
                let pong: ServerMsg = ServerMessage::pong(ts, now());
                if let Ok(bytes) = encode(&pong) {
                    let _ = ws.send_with_bytes(&bytes);
                }
            }

            ClientMessage::Resync { last_seq: _ } => {
                // Send full snapshot
                let snapshot_msg: ServerMsg = ServerMessage::snapshot(
                    self.room_state.borrow().clone(),
                    *self.seq.borrow(),
                    now(),
                );
                if let Ok(bytes) = encode(&snapshot_msg) {
                    let _ = ws.send_with_bytes(&bytes);
                }
            }

            ClientMessage::Action { op_id, action } => {
                self.handle_action(ws, conn, op_id, action).await?;
            }

            ClientMessage::Subscribe { domains: _ } => {
                // For this demo, we don't filter by domain
            }

            ClientMessage::Unsubscribe { domains: _ } => {
                // For this demo, we don't filter by domain
            }

            ClientMessage::Signal { .. } => {
                // WebRTC signalling not implemented in demo
            }
        }

        Ok(())
    }

    /// Handle a client action
    async fn handle_action(
        &self,
        ws: &WebSocket,
        conn: &ConnectionInfo,
        op_id: OpId,
        action: DemoAction,
    ) -> Result<()> {
        match action {
            DemoAction::Increment => {
                let new_value = {
                    let mut state = self.room_state.borrow_mut();
                    state.counter = state.counter.saturating_add(1);
                    state.counter
                };

                let delta = DemoDelta::CounterChanged { value: new_value };
                self.broadcast_delta(delta).await;

                // Send success
                let ok_msg: ServerMsg = ServerMessage::action_ok(op_id, None);
                if let Ok(bytes) = encode(&ok_msg) {
                    let _ = ws.send_with_bytes(&bytes);
                }
            }

            DemoAction::Decrement => {
                let new_value = {
                    let mut state = self.room_state.borrow_mut();
                    state.counter = state.counter.saturating_sub(1);
                    state.counter
                };

                let delta = DemoDelta::CounterChanged { value: new_value };
                self.broadcast_delta(delta).await;

                // Send success
                let ok_msg: ServerMsg = ServerMessage::action_ok(op_id, None);
                if let Ok(bytes) = encode(&ok_msg) {
                    let _ = ws.send_with_bytes(&bytes);
                }
            }

            DemoAction::SendMessage { text } => {
                let message = {
                    let mut state = self.room_state.borrow_mut();
                    let id = *self.next_message_id.borrow();
                    *self.next_message_id.borrow_mut() += 1;

                    let msg = ChatMessage {
                        id,
                        user_id: conn.user_id.clone(),
                        user_name: conn.user_name.clone(),
                        text,
                        timestamp: now(),
                    };

                    state.messages.push(msg.clone());

                    // Keep only last 100 messages
                    if state.messages.len() > 100 {
                        state.messages.remove(0);
                    }

                    msg
                };

                let delta = DemoDelta::MessageAdded { message };
                self.broadcast_delta(delta).await;

                // Send success
                let ok_msg: ServerMsg = ServerMessage::action_ok(op_id, None);
                if let Ok(bytes) = encode(&ok_msg) {
                    let _ = ws.send_with_bytes(&bytes);
                }
            }

            DemoAction::StartTyping => {
                // Broadcast typing event (not a state change)
                let event = DemoEvent::UserTyping {
                    user_id: conn.user_id.clone(),
                    user_name: conn.user_name.clone(),
                };
                self.broadcast_event("typing", event).await;

                // Send success
                let ok_msg: ServerMsg = ServerMessage::action_ok(op_id, None);
                if let Ok(bytes) = encode(&ok_msg) {
                    let _ = ws.send_with_bytes(&bytes);
                }
            }
        }

        Ok(())
    }

    /// Broadcast a delta to all connected clients
    async fn broadcast_delta(&self, delta: DemoDelta) {
        *self.seq.borrow_mut() += 1;
        let seq = *self.seq.borrow();

        let msg: ServerMsg = ServerMessage::delta(delta, seq, now());

        if let Ok(bytes) = encode(&msg) {
            for ws in self.state.get_websockets() {
                let _ = ws.send_with_bytes(&bytes);
            }
        }
    }

    /// Broadcast an event to all connected clients
    async fn broadcast_event(&self, domain: &str, event: DemoEvent) {
        let msg: ServerMsg = ServerMessage::notify(domain, event, None);

        if let Ok(bytes) = encode(&msg) {
            for ws in self.state.get_websockets() {
                let _ = ws.send_with_bytes(&bytes);
            }
        }
    }

    /// Broadcast presence information to all connected clients
    async fn broadcast_presence(&self) {
        let websockets = self.state.get_websockets();
        let mut users = Vec::new();

        for ws in &websockets {
            if let Ok(Some(conn)) = ws.deserialize_attachment::<ConnectionInfo>() {
                users.push(PresenceInfo {
                    user_id: conn.user_id,
                    name: Some(conn.user_name),
                    status: PresenceStatus::Active,
                    connected_at: conn.connected_at,
                });
            }
        }

        let msg: ServerMsg = ServerMessage::presence(users);

        if let Ok(bytes) = encode(&msg) {
            for ws in websockets {
                let _ = ws.send_with_bytes(&bytes);
            }
        }
    }
}

/// Get current timestamp in milliseconds
fn now() -> u64 {
    js_sys::Date::now() as u64
}

/// Simple random number generation for user IDs
mod rand {
    pub fn random<T>() -> T
    where
        T: From<u32>,
    {
        let r = js_sys::Math::random() * (u32::MAX as f64);
        T::from(r as u32)
    }
}
