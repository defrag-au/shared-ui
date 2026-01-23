//! Demo application types for the flow-demo worker.
//!
//! These types demonstrate how to define application-specific state, deltas,
//! events, and actions that work with the unified protocol.

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
            flip_delay_ms: 1500,
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
    /// Currently flipped cards (turn-taking mode - shared view)
    pub shared_flipped: Vec<usize>,
    /// Timestamp when cards were flipped (for auto-flip-back timer)
    pub flip_time: Option<u64>,
    /// Host user_id (can change settings)
    pub host: Option<String>,
}

/// Card face data sent to clients when a card is revealed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardFace {
    pub asset_id: String,
    pub name: String,
    pub image_url: String,
}

impl From<&Card> for CardFace {
    fn from(card: &Card) -> Self {
        Self {
            asset_id: card.asset_id.clone(),
            name: card.name.clone(),
            image_url: card.image_url.clone(),
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
    /// Game started - cards are dealt
    GameStarted {
        /// Turn order (turn-taking mode)
        turn_order: Vec<String>,
        /// Number of cards on the board
        card_count: usize,
        /// Shuffle seed for client-side determinism
        shuffle_seed: u64,
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
    /// Request a rematch after game ends
    RequestRematch,
    /// Reset entire game state (admin)
    ResetGame,
}

/// Type aliases for memory game protocol
pub type MemoryServerMsg =
    ui_flow_protocol::ServerMessage<MemoryGameState, MemoryDelta, MemoryEvent>;
pub type MemoryClientMsg = ui_flow_protocol::ClientMessage<MemoryAction>;
