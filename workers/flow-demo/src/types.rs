//! Demo application types for the flow-demo worker.
//!
//! These types demonstrate how to define application-specific state, deltas,
//! events, and actions that work with the unified protocol.

use serde::{Deserialize, Serialize};

/// The complete state of a demo room.
///
/// This is sent as a snapshot when a client connects.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DemoState {
    /// A simple counter that can be incremented/decremented
    pub counter: u64,
    /// Chat messages in the room
    pub messages: Vec<ChatMessage>,
}

/// A chat message in the room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Unique message ID
    pub id: u64,
    /// User who sent the message
    pub user_id: String,
    /// Display name of the user
    pub user_name: String,
    /// Message content
    pub text: String,
    /// Timestamp (unix ms)
    pub timestamp: u64,
}

/// Incremental state changes.
///
/// Instead of sending the full state, we send only what changed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DemoDelta {
    /// Counter value changed
    CounterChanged { value: u64 },
    /// A new message was added
    MessageAdded { message: ChatMessage },
    /// A user joined the room
    UserJoined { user_id: String, user_name: String },
    /// A user left the room
    UserLeft { user_id: String },
}

/// Application-specific events (notifications).
///
/// These are sent through the `Notify` message type for events that
/// don't directly modify state but are useful for the UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DemoEvent {
    /// A system announcement
    Announcement { text: String },
    /// A user is typing
    UserTyping { user_id: String, user_name: String },
}

/// Actions that clients can send to the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DemoAction {
    /// Increment the counter
    Increment,
    /// Decrement the counter
    Decrement,
    /// Send a chat message
    SendMessage { text: String },
    /// Indicate that the user is typing
    StartTyping,
}

/// Type aliases for the protocol types with our concrete types
pub type ServerMsg = ui_flow_protocol::ServerMessage<DemoState, DemoDelta, DemoEvent>;
pub type ClientMsg = ui_flow_protocol::ClientMessage<DemoAction>;
