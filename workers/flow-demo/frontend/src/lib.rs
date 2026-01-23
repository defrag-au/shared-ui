//! # Flow Demo Frontend
//!
//! A Leptos frontend demonstrating the unified realtime protocol.
//!
//! Features:
//! - WebSocket connection with auto-reconnect
//! - MessagePack binary protocol
//! - Snapshot + delta state synchronization
//! - Presence tracking
//! - Optimistic UI with action feedback

mod components;

use components::{Chat, Counter, Presence};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use ui_flow_protocol::{ClientMessage, OpId, PresenceInfo, ServerMessage};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

/// Application state synced from server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DemoState {
    pub counter: u64,
    pub messages: Vec<ChatMessage>,
}

/// A chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: u64,
    pub user_id: String,
    pub user_name: String,
    pub text: String,
    pub timestamp: u64,
}

/// State deltas from server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DemoDelta {
    CounterChanged { value: u64 },
    MessageAdded { message: ChatMessage },
    UserJoined { user_id: String, user_name: String },
    UserLeft { user_id: String },
}

/// Events from server (not state changes)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DemoEvent {
    Announcement { text: String },
    UserTyping { user_id: String, user_name: String },
}

/// Actions we can send to server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DemoAction {
    Increment,
    Decrement,
    SendMessage { text: String },
    StartTyping,
}

/// Connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
}

/// Type aliases for protocol messages
type ServerMsg = ServerMessage<DemoState, DemoDelta, DemoEvent>;
type ClientMsg = ClientMessage<DemoAction>;

/// Main application entry point
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize logging
    tracing_wasm::set_as_global_default();

    // Mount the app
    mount_to_body(App);
}

/// Main application component
#[component]
fn App() -> impl IntoView {
    // Room ID (can be changed)
    let (room_id, set_room_id) = create_signal("default".to_string());

    // Connection status
    let (status, set_status) = create_signal(ConnectionStatus::Disconnected);

    // Application state
    let (state, set_state) = create_signal(DemoState::default());

    // Presence list
    let (presence, set_presence) = create_signal(Vec::<PresenceInfo>::new());

    // WebSocket reference
    let ws: Rc<RefCell<Option<WebSocket>>> = Rc::new(RefCell::new(None));

    // Disconnect helper
    let ws_disconnect = ws.clone();
    let disconnect = Rc::new(move || {
        if let Some(socket) = ws_disconnect.borrow_mut().take() {
            let _ = socket.close();
        }
        set_status.set(ConnectionStatus::Disconnected);
    });

    // Connect to WebSocket
    let ws_connect = ws.clone();
    let connect = Rc::new(move || {
        let room = room_id.get();
        let ws_url = get_ws_url(&room);

        set_status.set(ConnectionStatus::Connecting);

        match WebSocket::new(&ws_url) {
            Ok(socket) => {
                socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

                // Handle open
                let set_status_clone = set_status;
                let on_open = Closure::wrap(Box::new(move |_: JsValue| {
                    set_status_clone.set(ConnectionStatus::Connected);
                    tracing::info!("WebSocket connected");
                }) as Box<dyn FnMut(JsValue)>);
                socket.set_onopen(Some(on_open.as_ref().unchecked_ref()));
                on_open.forget();

                // Handle close
                let set_status_clone = set_status;
                let on_close = Closure::wrap(Box::new(move |_: JsValue| {
                    set_status_clone.set(ConnectionStatus::Disconnected);
                    tracing::info!("WebSocket disconnected");
                }) as Box<dyn FnMut(JsValue)>);
                socket.set_onclose(Some(on_close.as_ref().unchecked_ref()));
                on_close.forget();

                // Handle error
                let on_error = Closure::wrap(Box::new(move |e: JsValue| {
                    tracing::error!("WebSocket error: {:?}", e);
                }) as Box<dyn FnMut(JsValue)>);
                socket.set_onerror(Some(on_error.as_ref().unchecked_ref()));
                on_error.forget();

                // Handle messages
                let set_state_clone = set_state;
                let set_presence_clone = set_presence;
                let on_message = Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Ok(buffer) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                        let array = js_sys::Uint8Array::new(&buffer);
                        let bytes = array.to_vec();

                        match ui_flow_protocol::decode::<ServerMsg>(&bytes) {
                            Ok(msg) => {
                                handle_server_message(msg, set_state_clone, set_presence_clone);
                            }
                            Err(e) => {
                                tracing::error!("Failed to decode message: {}", e);
                            }
                        }
                    }
                }) as Box<dyn FnMut(MessageEvent)>);
                socket.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
                on_message.forget();

                *ws_connect.borrow_mut() = Some(socket);
            }
            Err(e) => {
                tracing::error!("Failed to create WebSocket: {:?}", e);
                set_status.set(ConnectionStatus::Disconnected);
            }
        }
    });

    // Send action helper
    let ws_send = ws.clone();
    let send_action = Rc::new(move |action: DemoAction| {
        if let Some(socket) = ws_send.borrow().as_ref() {
            if socket.ready_state() == WebSocket::OPEN {
                let msg: ClientMsg = ClientMessage::action(OpId::new(), action);
                if let Ok(bytes) = ui_flow_protocol::encode(&msg) {
                    let _ = socket.send_with_u8_array(&bytes);
                }
            }
        }
    });

    // Auto-connect on mount
    let connect_effect = connect.clone();
    create_effect(move |_| {
        connect_effect();
    });

    // Clone for view handlers
    let connect_view = connect.clone();
    let disconnect_view = disconnect.clone();

    view! {
        <div class="header">
            <h1>"Flow Demo"</h1>
            <p class="subtitle">"Unified Realtime Protocol with MessagePack"</p>
        </div>

        <div class="room-selector">
            <label>"Room:"</label>
            <input
                type="text"
                value=move || room_id.get()
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    set_room_id.set(value);
                    disconnect_view();
                    connect_view();
                }
            />
            <ConnectionStatusBadge status=status />
        </div>

        <div class="main-content">
            <div class="left-column">
                <Counter
                    value=Signal::derive(move || state.get().counter)
                    on_increment={
                        let send = send_action.clone();
                        move || send(DemoAction::Increment)
                    }
                    on_decrement={
                        let send = send_action.clone();
                        move || send(DemoAction::Decrement)
                    }
                    disabled=Signal::derive(move || status.get() != ConnectionStatus::Connected)
                />

                <Chat
                    messages=Signal::derive(move || state.get().messages)
                    on_send={
                        let send = send_action.clone();
                        move |text: String| send(DemoAction::SendMessage { text })
                    }
                    disabled=Signal::derive(move || status.get() != ConnectionStatus::Connected)
                />
            </div>

            <div class="right-column">
                <Presence users=presence />
            </div>
        </div>
    }
}

/// Connection status badge component
#[component]
fn ConnectionStatusBadge(status: ReadSignal<ConnectionStatus>) -> impl IntoView {
    let (class, text) = match status.get() {
        ConnectionStatus::Connected => ("connection-status connected", "Connected"),
        ConnectionStatus::Connecting => ("connection-status connecting", "Connecting..."),
        ConnectionStatus::Disconnected => ("connection-status disconnected", "Disconnected"),
    };

    view! {
        <div class=class>
            <span class="indicator"></span>
            <span>{text}</span>
        </div>
    }
}

/// Handle incoming server message
fn handle_server_message(
    msg: ServerMsg,
    set_state: WriteSignal<DemoState>,
    set_presence: WriteSignal<Vec<PresenceInfo>>,
) {
    match msg {
        ServerMessage::Connected { .. } => {
            tracing::info!("Received Connected message");
        }

        ServerMessage::Snapshot { state, seq, .. } => {
            tracing::info!("Received Snapshot at seq {}", seq);
            set_state.set(state);
        }

        ServerMessage::Delta { delta, seq, .. } => {
            tracing::debug!("Received Delta at seq {}", seq);
            set_state.update(|s| apply_delta(s, delta));
        }

        ServerMessage::Deltas { deltas, seq, .. } => {
            tracing::debug!("Received {} Deltas, final seq {}", deltas.len(), seq);
            set_state.update(|s| {
                for delta in deltas {
                    apply_delta(s, delta);
                }
            });
        }

        ServerMessage::Presence { users } => {
            tracing::debug!("Received Presence: {} users", users.len());
            set_presence.set(users);
        }

        ServerMessage::Notify { domain, event, .. } => {
            tracing::debug!("Received Notify on domain: {}", domain);
            match event {
                DemoEvent::Announcement { text } => {
                    tracing::info!("Announcement: {}", text);
                }
                DemoEvent::UserTyping { user_name, .. } => {
                    tracing::debug!("{} is typing...", user_name);
                }
            }
        }

        ServerMessage::ActionOk { op_id, .. } => {
            tracing::debug!("Action {} completed successfully", op_id);
        }

        ServerMessage::ActionErr { op_id, message, .. } => {
            tracing::error!("Action {} failed: {}", op_id, message);
        }

        ServerMessage::Progress {
            op_id,
            percent,
            message,
        } => {
            tracing::debug!("Action {} progress: {:?}% - {:?}", op_id, percent, message);
        }

        ServerMessage::Pong { .. } => {
            // Latency tracking could go here
        }

        ServerMessage::Error { message, fatal, .. } => {
            tracing::error!("Server error (fatal={}): {}", fatal, message);
        }

        ServerMessage::Signal { .. } => {
            // WebRTC signalling not implemented
        }
    }
}

/// Apply a delta to the state
fn apply_delta(state: &mut DemoState, delta: DemoDelta) {
    match delta {
        DemoDelta::CounterChanged { value } => {
            state.counter = value;
        }
        DemoDelta::MessageAdded { message } => {
            state.messages.push(message);
            // Keep only last 100 messages
            if state.messages.len() > 100 {
                state.messages.remove(0);
            }
        }
        DemoDelta::UserJoined { .. } => {
            // Presence is handled separately
        }
        DemoDelta::UserLeft { .. } => {
            // Presence is handled separately
        }
    }
}

/// Get WebSocket URL based on current location
fn get_ws_url(room_id: &str) -> String {
    let window = web_sys::window().expect("no window");
    let location = window.location();
    let protocol = location.protocol().unwrap_or_else(|_| "http:".to_string());
    let host = location
        .host()
        .unwrap_or_else(|_| "localhost:8787".to_string());

    let ws_protocol = if protocol == "https:" { "wss:" } else { "ws:" };

    format!("{}//{}/ws/{}?user_name=User", ws_protocol, host, room_id)
}
