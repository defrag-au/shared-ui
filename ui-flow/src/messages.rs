//! Wire protocol message types
//!
//! Defines the client-to-server and server-to-client message formats.

use serde::{Deserialize, Serialize};

use crate::operation::{ActionError, ActionProgress, OpId};

/// Messages sent from client to server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage<Action> {
    /// Keepalive ping
    Ping,
    /// Request state synchronization from a specific point
    SyncState {
        /// Last known sequence number (for delta recovery)
        #[serde(skip_serializing_if = "Option::is_none")]
        from_seq: Option<u64>,
    },
    /// Client action with operation tracking
    Action {
        /// Operation ID for correlation
        op_id: OpId,
        /// The action payload
        action: Action,
    },
    /// Subscribe to additional topics/channels
    Subscribe {
        /// Topic to subscribe to
        topic: String,
    },
    /// Unsubscribe from a topic
    Unsubscribe {
        /// Topic to unsubscribe from
        topic: String,
    },
}

/// Messages sent from server to client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage<State, Delta> {
    /// Connection acknowledged
    Connected,
    /// Keepalive pong
    Pong,
    /// Full state snapshot
    Snapshot {
        /// The complete state
        state: State,
        /// Current sequence number
        seq: u64,
    },
    /// Incremental state update
    Delta {
        /// The change to apply
        delta: Delta,
        /// New sequence number after this delta
        seq: u64,
    },
    /// Progress update for a pending operation
    Progress(ActionProgress),
    /// Operation completed successfully
    ActionComplete {
        /// The completed operation
        op_id: OpId,
    },
    /// Operation failed
    ActionError(ActionError),
    /// Server-initiated error message
    Error {
        /// Error message
        message: String,
    },
}
