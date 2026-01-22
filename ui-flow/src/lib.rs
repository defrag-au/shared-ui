//! Framework-agnostic real-time state synchronization
//!
//! `ui-flow` provides a snapshot + delta pattern for efficient real-time state
//! synchronization over WebSockets. It supports optimistic UI with operation
//! tracking and automatic reconnection.
//!
//! ## Core Concepts
//!
//! - **Snapshot**: Full state sent on connection
//! - **Delta**: Incremental updates sent after the snapshot
//! - **FlowState**: Trait for state types that can receive deltas
//! - **OpId**: Unique identifier for tracking client actions through their lifecycle
//!
//! ## Quick Start
//!
//! ```ignore
//! use ui_flow::{FlowConnection, FlowState, OpId};
//!
//! // Define your state and delta types
//! #[derive(Clone, Default, Deserialize)]
//! struct GameState {
//!     score: u32,
//!     entities: HashMap<String, Entity>,
//! }
//!
//! #[derive(Deserialize)]
//! enum GameDelta {
//!     ScoreChanged(u32),
//!     EntityUpdated { id: String, entity: Entity },
//! }
//!
//! impl FlowState for GameState {
//!     type Delta = GameDelta;
//!
//!     fn apply_delta(&mut self, delta: Self::Delta) {
//!         match delta {
//!             GameDelta::ScoreChanged(score) => self.score = score,
//!             GameDelta::EntityUpdated { id, entity } => {
//!                 self.entities.insert(id, entity);
//!             }
//!         }
//!     }
//! }
//!
//! // Define your action type
//! #[derive(Serialize)]
//! enum GameAction {
//!     Move { direction: String },
//!     Attack { target: String },
//! }
//!
//! // Create a connection
//! let connection = FlowConnection::<GameState, GameAction>::builder()
//!     .url("wss://example.com/game")
//!     .on_snapshot(|state, seq| {
//!         // Replace local state with server snapshot
//!         set_state(state);
//!     })
//!     .on_delta(|delta, seq| {
//!         // Apply incremental update
//!         update_state(|s| s.apply_delta(delta));
//!     })
//!     .on_status(|status| {
//!         // Update connection indicator
//!         set_status(status);
//!     })
//!     .connect()?;
//!
//! // Send an action with tracking
//! let op_id = OpId::new();
//! connection.send_action(op_id, GameAction::Move { direction: "north".into() })?;
//! ```
//!
//! ## Optimistic UI
//!
//! Use `OperationTracker` to manage pending operations:
//!
//! ```ignore
//! use ui_flow::{OperationTracker, OpId};
//!
//! let mut tracker = OperationTracker::new();
//!
//! // Start tracking when sending an action
//! let op_id = tracker.start(MyActionData { ... });
//! connection.send_action(op_id, action)?;
//!
//! // On progress update
//! tracker.update_progress(progress);
//!
//! // On completion
//! let data = tracker.complete(op_id);
//!
//! // On error
//! let data = tracker.fail(&error);
//! ```
//!
//! ## Framework Integration
//!
//! This crate is framework-agnostic and uses callbacks. Framework-specific
//! wrappers can integrate these callbacks with their reactive systems:
//!
//! - **Leptos**: Use signals in callbacks
//! - **Seed**: Bridge to Seed's message system
//! - **Yew**: Use component callbacks
//!
//! See the `examples` directory for framework-specific integration patterns.

mod connection;
mod messages;
mod operation;
mod state;
mod status;

pub use connection::{FlowConnection, FlowConnectionBuilder, FlowError, ReconnectConfig};
pub use messages::{ClientMessage, ServerMessage};
pub use operation::{ActionError, ActionProgress, OpId, OperationTracker, PendingOperation};
pub use state::FlowState;
pub use status::{CloseInfo, ConnectionStatus};
