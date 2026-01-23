//! Demo application types for the flow-demo worker.
//!
//! These types demonstrate how to define application-specific state, deltas,
//! events, and actions that work with the unified protocol.

use cardano_assets::AssetId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Counter/Chat Demo Types (Original Demo)
// =============================================================================

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

// =============================================================================
// Memory Game Types
// =============================================================================

/// Game mode determines how players interact with the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameMode {
    /// Classic turn-taking with shared board view - all players see all flips
    TurnTaking,
    /// Simultaneous race with independent views - only see own flips until match
    Race,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::TurnTaking
    }
}

/// Game configuration set by the host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Grid dimensions (cols, rows) - e.g., (8, 8) = 64 cards = 32 pairs
    pub grid_size: (u8, u8),
    /// Game mode
    pub mode: GameMode,
    /// Policy ID for fetching assets
    pub policy_id: String,
    /// Milliseconds before non-matching cards flip back
    pub flip_delay_ms: u64,
    /// Shuffle seed for deterministic card layout
    pub shuffle_seed: u64,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            grid_size: (6, 6),
            mode: GameMode::TurnTaking,
            policy_id: "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6".to_string(), // Black Flag
            flip_delay_ms: 1200,
            shuffle_seed: 0,
        }
    }
}

/// Current phase of the game
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case")]
pub enum GamePhase {
    /// Waiting for players to join
    Lobby { min_players: u8, max_players: u8 },
    /// Countdown before game starts
    Starting { countdown: u8 },
    /// Loading assets - waiting for players to signal ready
    Loading {
        /// Players who have signaled ready
        ready_players: Vec<String>,
        /// Total players expected to be ready
        total_players: usize,
    },
    /// Game in progress
    Playing,
    /// Game finished
    Finished {
        /// Winner (None if tie or solo)
        winner: Option<String>,
        /// Final rankings: (user_id, name, score)
        rankings: Vec<(String, String, u32)>,
    },
}

impl Default for GamePhase {
    fn default() -> Self {
        Self::Lobby {
            min_players: 1,
            max_players: 8,
        }
    }
}

// =============================================================================
// Turn State Machine (for turn-taking mode)
// =============================================================================

/// Finite state machine for a single turn's flip/match cycle.
///
/// This cleanly models the valid states and transitions during card flipping,
/// ensuring we don't start timers until images are loaded.
///
/// ```text
///                              ┌─────────────────┐
///                              │ AwaitingFirst   │ ◄── Turn starts here
///                              └────────┬────────┘
///                                       │ FlipCard(idx)
///                                       ▼
///                              ┌─────────────────┐
///                              │ FirstFlipped    │
///                              │ { idx, acked }  │
///                              └────────┬────────┘
///                                       │ AckCardLoaded (acked=true)
///                                       │ then FlipCard(idx2)
///                                       ▼
///                              ┌─────────────────┐
///                              │ SecondFlipped   │
///                              │ { first, second,│
///                              │   first_acked,  │
///                              │   second_acked }│
///                              └────────┬────────┘
///                                       │ Both acked
///                                       ▼
///                              ┌─────────────────┐
///                              │ BothReady       │
///                              │ { first, second,│
///                              │   is_match }    │ ◄── Timer starts HERE
///                              └────────┬────────┘
///                                       │ Timer expires OR match handled
///                                       ▼
///                              ┌─────────────────┐
///                              │ AwaitingFirst   │ (next turn)
///                              └─────────────────┘
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum TurnState {
    /// Waiting for the current player to flip their first card
    AwaitingFirst,

    /// First card has been flipped, waiting for ACK and/or second flip
    FirstFlipped { index: usize, acked: bool },

    /// Second card has been flipped, waiting for both ACKs
    SecondFlipped {
        first: usize,
        second: usize,
        first_acked: bool,
        second_acked: bool,
    },

    /// Both cards flipped and ACKed - timer is running
    BothReady {
        first: usize,
        second: usize,
        is_match: bool,
        /// Timestamp when both became ready (timer started)
        ready_at: u64,
    },
}

impl Default for TurnState {
    fn default() -> Self {
        Self::AwaitingFirst
    }
}

impl TurnState {
    /// Handle a card flip action. Returns the new state, or None if invalid.
    pub fn on_flip(&self, index: usize) -> Option<TurnState> {
        match self {
            TurnState::AwaitingFirst => Some(TurnState::FirstFlipped {
                index,
                acked: false,
            }),

            TurnState::FirstFlipped {
                index: first,
                acked,
            } => {
                if index == *first {
                    None // Can't flip same card twice
                } else {
                    Some(TurnState::SecondFlipped {
                        first: *first,
                        second: index,
                        first_acked: *acked,
                        second_acked: false,
                    })
                }
            }

            // Can't flip more cards until turn resolves
            TurnState::SecondFlipped { .. } | TurnState::BothReady { .. } => None,
        }
    }

    /// Handle a card ACK. Returns the new state.
    /// If both cards are now acked, transitions to BothReady with match info.
    pub fn on_ack(&self, index: usize, is_match: bool, now: u64) -> TurnState {
        match self {
            TurnState::FirstFlipped {
                index: first,
                acked: _,
            } if index == *first => TurnState::FirstFlipped {
                index: *first,
                acked: true,
            },

            TurnState::SecondFlipped {
                first,
                second,
                first_acked,
                second_acked,
            } => {
                let new_first_acked = *first_acked || index == *first;
                let new_second_acked = *second_acked || index == *second;

                if new_first_acked && new_second_acked {
                    TurnState::BothReady {
                        first: *first,
                        second: *second,
                        is_match,
                        ready_at: now,
                    }
                } else {
                    TurnState::SecondFlipped {
                        first: *first,
                        second: *second,
                        first_acked: new_first_acked,
                        second_acked: new_second_acked,
                    }
                }
            }

            // Ignore ACKs in other states
            other => other.clone(),
        }
    }

    /// Check if the timer has expired and we should resolve the turn.
    /// Returns true if in BothReady state and enough time has passed.
    pub fn should_resolve(&self, now: u64, delay_ms: u64) -> bool {
        matches!(
            self,
            TurnState::BothReady { ready_at, .. } if now >= ready_at + delay_ms
        )
    }

    /// Get the flipped card indices (if any) for rendering
    pub fn flipped_indices(&self) -> Vec<usize> {
        match self {
            TurnState::AwaitingFirst => vec![],
            TurnState::FirstFlipped { index, .. } => vec![*index],
            TurnState::SecondFlipped { first, second, .. } => vec![*first, *second],
            TurnState::BothReady { first, second, .. } => vec![*first, *second],
        }
    }

    /// Check if we're waiting for any ACKs
    pub fn awaiting_acks(&self) -> bool {
        match self {
            TurnState::FirstFlipped { acked, .. } => !acked,
            TurnState::SecondFlipped {
                first_acked,
                second_acked,
                ..
            } => !first_acked || !second_acked,
            _ => false,
        }
    }
}

/// A card on the game board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// Asset identifier from PFP City
    pub asset_id: String,
    /// Display name of the asset
    pub name: String,
    /// Image URL for the card face
    pub image_url: String,
    /// Whether this card has been matched (stays revealed)
    pub matched: bool,
    /// Who matched this card (user_id)
    pub matched_by: Option<String>,
    /// Pair ID - cards with same pair_id match each other
    pub pair_id: u8,
}

/// State for a single player in the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    /// User identifier
    pub user_id: String,
    /// Display name
    pub user_name: String,
    /// Current score (number of pairs found)
    pub score: u32,
    /// Cards this player has flipped (race mode - each player has own view)
    pub flipped: Vec<usize>,
    /// Whether player is spectating (joined after game started)
    pub spectating: bool,
    /// When the player joined
    pub joined_at: u64,
}

/// Complete authoritative game state (server-side)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryGameState {
    /// Game configuration
    pub config: GameConfig,
    /// All cards on the board
    pub cards: Vec<Card>,
    /// Current game phase
    pub phase: GamePhase,
    /// Player data keyed by user_id
    pub players: HashMap<String, PlayerState>,
    /// Turn order (list of user_ids) - used in turn-taking mode
    pub turn_order: Vec<String>,
    /// Index into turn_order for current player
    pub current_turn: usize,
    /// Turn state machine (turn-taking mode) - tracks flip/ack/match cycle
    pub turn_state: TurnState,
    /// Host user_id (can change settings)
    pub host: Option<String>,
}

impl MemoryGameState {
    /// Get currently flipped card indices (convenience method)
    pub fn flipped_indices(&self) -> Vec<usize> {
        self.turn_state.flipped_indices()
    }
}

/// Card face data sent to clients when a card is revealed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardFace {
    pub asset_id: String,
    pub name: String,
}

impl From<&Card> for CardFace {
    fn from(card: &Card) -> Self {
        Self {
            asset_id: card.asset_id.clone(),
            name: card.name.clone(),
        }
    }
}

/// Hidden card data - just the structure, no revealed info
/// Sent when cards are dealt so clients know board layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiddenCard {
    /// Whether this card has been matched
    pub matched: bool,
    /// Who matched this card (if matched)
    pub matched_by: Option<String>,
}

impl From<&Card> for HiddenCard {
    fn from(card: &Card) -> Self {
        Self {
            matched: card.matched,
            matched_by: card.matched_by.clone(),
        }
    }
}

/// Incremental state changes for the memory game
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MemoryDelta {
    // === Lobby Phase ===
    /// Player joined the game
    PlayerJoined {
        user_id: String,
        user_name: String,
        spectating: bool,
    },
    /// Player left the game
    PlayerLeft { user_id: String },
    /// Game configuration was changed by host
    ConfigChanged { config: GameConfig },
    /// Host changed (e.g., original host left)
    HostChanged { user_id: String },

    // === Starting Phase ===
    /// Countdown tick
    CountdownTick { seconds: u8 },
    /// Cards have been dealt - contains hidden card structure and asset IDs for preloading
    /// Game enters Loading phase after this
    CardsDealt {
        /// Hidden cards (no face data - just matched state)
        cards: Vec<HiddenCard>,
        /// Asset IDs for all unique cards (for preloading images)
        asset_ids: Vec<AssetId>,
        /// Total number of players expected to signal ready
        total_players: usize,
    },
    /// A player has signaled they are ready (assets loaded)
    PlayerReady {
        user_id: String,
        /// Number of players now ready
        ready_count: usize,
        /// Total players expected
        total_players: usize,
    },
    /// All players ready - game actually starts now
    GameStarted {
        /// Turn order (turn-taking mode)
        turn_order: Vec<String>,
    },

    // === Turn-Taking Mode ===
    /// A card was flipped (visible to all in turn-taking)
    CardFlipped {
        index: usize,
        by: String,
        face: CardFace,
    },
    /// Turn passed to next player
    TurnChanged { user_id: String },

    // === Race Mode ===
    /// Your own card flip result (only you see this in race mode)
    OwnCardFlipped { index: usize, face: CardFace },

    // === Both Modes ===
    /// A pair was matched and claimed
    PairMatched {
        indices: [usize; 2],
        by: String,
        by_name: String,
        new_score: u32,
        face: CardFace,
    },
    /// Cards didn't match - flip them back
    CardsReset {
        indices: [usize; 2],
        /// In race mode, only reset for specific player; None = all players
        for_player: Option<String>,
    },
    /// Score changed (redundant with PairMatched but useful for UI)
    ScoreChanged { user_id: String, score: u32 },

    // === Game End ===
    /// Game has ended
    GameEnded {
        winner: Option<String>,
        rankings: Vec<(String, String, u32)>,
    },
}

/// Events/notifications for the memory game (not state changes)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MemoryEvent {
    /// Countdown before game starts
    StartingIn { seconds: u8 },
    /// Player found a match (celebration moment)
    MatchFound {
        player_name: String,
        pair_name: String,
    },
    /// Player got a streak of consecutive matches (turn-taking)
    Streak { player_name: String, count: u8 },
    /// Someone is close to winning (race mode)
    NearVictory {
        player_name: String,
        pairs_remaining: u8,
    },
    /// Invalid action attempted
    InvalidAction { reason: String },
}

/// Actions players can send to the server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MemoryAction {
    /// Join the game (lobby or as spectator if in progress)
    JoinGame { user_name: String },
    /// Leave the game
    LeaveGame,
    /// Update game config (lobby phase only)
    SetConfig {
        mode: Option<GameMode>,
        grid_size: Option<(u8, u8)>,
    },
    /// Start the game (lobby phase only)
    StartGame,
    /// Flip a card at the given index
    FlipCard { index: usize },
    /// Acknowledge that a flipped card's image has loaded (starts flip-back timer)
    AckCardLoaded { index: usize },
    /// Signal that client has finished preloading assets and is ready to play
    Ready,
    /// Request a rematch after game ends
    RequestRematch,
    /// Reset entire game state (admin)
    ResetGame,
}

/// Type aliases for memory game protocol
pub type MemoryServerMsg =
    ui_flow_protocol::ServerMessage<MemoryGameState, MemoryDelta, MemoryEvent>;
pub type MemoryClientMsg = ui_flow_protocol::ClientMessage<MemoryAction>;
