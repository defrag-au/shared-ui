# Black Flag Memory Game - Design Document

## Overview

A competitive multiplayer memory/matching game using Black Flag NFT assets. Players flip cards to find matching pairs, competing for the best score either solo or against other players in real-time.

## Goals

1. **Demonstrate flow-demo capabilities** - Real-time multiplayer, state sync, presence
2. **Migrate components to shared-ui** - Move AssetCard from augminted-bots
3. **Create engaging proof-of-concept** - Fun game that showcases the NFT collection

## Game Rules

### Basic Mechanics
- Grid of face-down cards (e.g., 4x4 = 8 pairs, 6x6 = 18 pairs)
- Players flip two cards to find matching pairs
- If cards match, player scores a point and cards stay revealed
- If cards don't match, cards flip back face-down
- Game ends when all pairs are found

### Game Modes

#### 1. Turn-Taking Mode (Classic)
Traditional memory game with shared board state.

- Players take turns in order (supports 2+ players)
- Current player highlighted in UI
- **All players see the same board** - everyone watches flips in real-time
- Learning from others' mistakes is part of the strategy
- On match: Player scores, gets another turn
- On miss: Cards flip back, turn passes to next player

**Strategic element:** Pay attention to what others reveal - their mistakes become your advantage.

#### 2. Race Mode (Competitive)
Fast-paced simultaneous play with independent boards.

- All players play at the same time on **their own view** of the board
- Same card layout for all players (same shuffle seed)
- Players cannot see each other's flips
- First player to match a pair **claims it** - pair is then revealed/locked for all
- Claimed pairs show who found them
- Race to find pairs before opponents

**Strategic element:** Speed vs accuracy trade-off. Flip fast but remember positions.

**Variant - "Snipe Mode":** 
- Players see each other's first card flip
- Can "snipe" by quickly finding the match before the original player
- Higher risk/reward

### Multiplayer Support (2-8 players)

| Aspect | Turn-Taking | Race Mode |
|--------|-------------|-----------|
| Player count | 2-8 | 2-8 |
| Board view | Shared | Independent (until claimed) |
| Pacing | Sequential | Simultaneous |
| Turn indicator | Yes | No |
| Flip visibility | All see all | Only own flips (until match) |
| Match claim | Automatic | First to complete |

### Scoring
- +1 point per matched pair
- Race mode: Bonus for being first to N pairs
- Optional: Streak bonus for consecutive matches (turn-taking)
- Optional: Speed bonus (race mode)

## Architecture

### State Model

```rust
/// Complete game state (server-side, authoritative)
struct MemoryGameState {
    /// Game configuration
    config: GameConfig,
    /// All cards on the board (shared layout)
    cards: Vec<Card>,
    /// Current game phase
    phase: GamePhase,
    /// Player data (user_id -> PlayerState)
    players: HashMap<String, PlayerState>,
    /// Turn order (list of user_ids) - used in turn-taking mode
    turn_order: Vec<String>,
    /// Current turn index - used in turn-taking mode
    current_turn: usize,
    /// Shared flipped cards (turn-taking mode only)
    shared_flipped: Vec<usize>,
    /// Timestamp when cards were flipped (for auto-flip-back)
    flip_time: Option<u64>,
}

struct PlayerState {
    user_id: String,
    user_name: String,
    score: u32,
    /// Player's currently flipped cards (race mode - each player has own view)
    flipped: Vec<usize>,
    /// Join timestamp
    joined_at: u64,
}

struct GameConfig {
    /// Grid dimensions
    grid_size: (u8, u8),  // e.g., (4, 4) = 16 cards = 8 pairs
    /// Game mode
    mode: GameMode,
    /// Policy ID for assets
    policy_id: String,
    /// Match delay (ms before non-matching cards flip back)
    flip_delay_ms: u64,
    /// Shuffle seed (ensures same layout for all players)
    shuffle_seed: u64,
}

enum GameMode {
    /// Classic turn-taking with shared board view
    TurnTaking,
    /// Simultaneous race with independent views
    Race,
    /// Race variant where first flip is visible to all
    RaceSnipe,
}

struct Card {
    /// Asset identifier
    asset_id: String,
    /// Display name
    name: String,
    /// Image URL
    image_url: String,
    /// Matched state (shared - once matched, matched for all)
    matched: bool,
    /// Who matched this card (user_id)
    matched_by: Option<String>,
    /// Pair ID (cards with same pair_id match)
    pair_id: u8,
}

enum GamePhase {
    /// Waiting for players to join
    Lobby { min_players: u8, max_players: u8 },
    /// Countdown before start
    Starting { countdown: u8 },
    /// Game in progress
    Playing,
    /// Game finished
    Finished { 
        winner: Option<String>,
        final_rankings: Vec<(String, u32)>,  // (user_id, score) sorted
    },
}

/// What each player sees (derived from server state)
/// Server sends personalized view based on game mode
struct PlayerView {
    /// Cards with visibility based on mode
    cards: Vec<CardView>,
    /// All player scores (always visible)
    scores: Vec<(String, String, u32)>,  // (user_id, name, score)
    /// Current turn player (turn-taking mode)
    current_turn: Option<String>,
    /// Is it my turn? (turn-taking mode)
    is_my_turn: bool,
    /// My currently flipped cards
    my_flipped: Vec<usize>,
}

struct CardView {
    /// Card index
    index: usize,
    /// Visible if: matched, or flipped by me, or (turn-taking & flipped)
    visible: bool,
    /// Card face data (only if visible)
    face: Option<CardFace>,
    /// Match info
    matched: bool,
    matched_by: Option<String>,
}

struct CardFace {
    asset_id: String,
    name: String,
    image_url: String,
}
```

### Deltas

```rust
enum MemoryDelta {
    /// Player joined the game lobby
    PlayerJoined { user_id: String, user_name: String },
    /// Player left the game
    PlayerLeft { user_id: String },
    /// Game configuration changed (host setting mode, grid size)
    ConfigChanged { config: GameConfig },
    /// Countdown started
    CountdownTick { seconds: u8 },
    /// Game started - includes card layout for all players
    GameStarted { 
        turn_order: Vec<String>,  // Only relevant for turn-taking
        card_count: usize,        // Number of cards (face data sent separately)
    },
    
    // === Turn-Taking Mode Deltas ===
    /// Card was flipped (visible to all in turn-taking)
    CardFlipped { index: usize, by: String, face: CardFace },
    /// Turn changed to next player
    TurnChanged { player: String },
    
    // === Race Mode Deltas ===
    /// Your own card flip result (only you see until match)
    OwnCardFlipped { index: usize, face: CardFace },
    /// Another player's first flip (snipe mode only)
    OpponentFirstFlip { by: String, index: usize, face: CardFace },
    
    // === Shared Deltas (both modes) ===
    /// A pair was matched and claimed
    PairMatched { 
        indices: [usize; 2], 
        by: String, 
        by_name: String,
        new_score: u32,
        face: CardFace,  // Reveal to all players
    },
    /// Cards didn't match - flip back (turn-taking: all see, race: only flipper)
    CardsReset { indices: [usize; 2], for_player: Option<String> },
    /// Score update
    ScoreChanged { user_id: String, score: u32 },
    /// Game ended
    GameEnded { 
        winner: Option<String>,
        rankings: Vec<(String, String, u32)>,  // (user_id, name, score)
    },
}
```

### Actions

```rust
enum MemoryAction {
    /// Join the game lobby
    JoinGame,
    /// Leave the game
    LeaveGame,
    /// Update game config (host only, in lobby)
    SetConfig { 
        mode: Option<GameMode>,
        grid_size: Option<(u8, u8)>,
    },
    /// Start the game (host only, when enough players)
    StartGame,
    /// Flip a card
    FlipCard { index: usize },
    /// Request rematch (after game ends)
    RequestRematch,
}
```

### Events (Notifications)

```rust
enum MemoryEvent {
    /// Countdown tick before game starts
    StartingIn { seconds: u8 },
    /// Player found a match (celebration moment)
    MatchFound { player_name: String, pair_name: String },
    /// Streak notification (turn-taking mode)
    Streak { player_name: String, count: u8 },
    /// Race mode: Someone is close to winning
    NearVictory { player_name: String, pairs_remaining: u8 },
    /// Snipe! Someone stole a match
    Sniped { sniper: String, victim: String },
}
```

### Protocol Consideration: Personalized Snapshots

In race mode, each player needs a **personalized view** since they can't see others' flips. Options:

1. **Personalized Snapshots**: Server sends different snapshot to each player
   - Pro: Clean separation
   - Con: More complex server logic, can't use broadcast

2. **Shared State + Client Filtering**: Server sends full state, client filters
   - Pro: Simple broadcast
   - Con: Cheating possible (client sees all card positions)

3. **Hybrid**: Shared matched pairs, personalized flip state
   - Server broadcasts: matched pairs, scores, game phase
   - Server unicasts: player's own flipped cards
   - **Recommended approach**

```rust
/// Broadcast to all players
struct SharedGameState {
    phase: GamePhase,
    matched_pairs: Vec<MatchedPair>,  // Revealed to all
    scores: Vec<(String, String, u32)>,
    current_turn: Option<String>,  // Turn-taking only
}

/// Sent only to specific player
struct PrivateFlipState {
    my_flipped: Vec<(usize, CardFace)>,
}
```

## Component Migration

### AssetCard → shared-ui/components

The existing `AssetCard` from `augminted-bots/widgets/widget-common/src/web_components/asset_card.rs` needs to be migrated to `shared-ui/components/`.

**Migration considerations:**
1. Remove maud dependency - use HTML string generation like `connection_status`
2. Use `scss_inline!` for styles instead of inline CSS
3. Simplify for memory game use case (may not need all contexts)
4. Add `flipped` state for card face-down/face-up animation

**New attributes for memory game:**
- `flipped` - Whether card is face-up
- `matched` - Whether card has been matched (stays revealed)
- `disabled` - Cannot be clicked

**New events:**
- `card-flip` - Dispatched when card is clicked

### MemoryCard Component (Alternative)

Instead of modifying AssetCard, create a simpler `MemoryCard` component specifically for the game:

```rust
struct MemoryCard {
    /// Asset image URL (shown when flipped)
    image_url: String,
    /// Card back image/pattern
    back_pattern: String,
    /// Current state
    state: CardState,
    /// Click enabled
    clickable: bool,
}
```

**Recommendation:** Create `MemoryCard` as a focused component, potentially wrapping or using `AssetCard` internally for the face-up state.

## API Integration

### Asset Fetching

Use PFP City API to fetch random assets from the Black Flag collection:

```
GET https://a2.pfp.city/v3/api/collections/{policy_id}/assets?limit={n}&offset={random}
```

**Server-side (Durable Object):**
1. On game creation, fetch `n` random assets (where `n = grid_size / 2`)
2. Duplicate each asset to create pairs
3. Shuffle the combined array
4. Store in game state

**Considerations:**
- Cache collection metadata to avoid repeated API calls
- Use deterministic shuffle with seeded RNG for reproducibility
- Policy ID configured per room or hardcoded for demo

### Image URLs

Use PFP City image service:
```
https://img.pfp.city/{policy_id}/img/thumb/{asset_id}.png
```

Thumbnail size (400px) is sufficient for card display.

## UI Layout

```
┌─────────────────────────────────────────────────────┐
│  Black Flag Memory                    [Connected]   │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌─────────────────────────┐  ┌──────────────────┐ │
│  │                         │  │  Players         │ │
│  │   ┌──┐ ┌──┐ ┌──┐ ┌──┐  │  │                  │ │
│  │   │??│ │??│ │☠️│ │??│  │  │  🟢 Player1 (me) │ │
│  │   └──┘ └──┘ └──┘ └──┘  │  │     Score: 3     │ │
│  │                         │  │                  │ │
│  │   ┌──┐ ┌──┐ ┌──┐ ┌──┐  │  │  ⚪ Player2      │ │
│  │   │??│ │🏴│ │??│ │??│  │  │     Score: 2     │ │
│  │   └──┘ └──┘ └──┘ └──┘  │  │                  │ │
│  │                         │  │  Current turn:   │ │
│  │   ┌──┐ ┌──┐ ┌──┐ ┌──┐  │  │  → Player1       │ │
│  │   │☠️│ │??│ │🏴│ │??│  │  │                  │ │
│  │   └──┘ └──┘ └──┘ └──┘  │  ├──────────────────┤ │
│  │                         │  │  Game Info       │ │
│  │   ┌──┐ ┌──┐ ┌──┐ ┌──┐  │  │  Pairs: 3/8      │ │
│  │   │??│ │??│ │??│ │??│  │  │  Grid: 4x4       │ │
│  │   └──┘ └──┘ └──┘ └──┘  │  │                  │ │
│  │                         │  │  [New Game]      │ │
│  └─────────────────────────┘  └──────────────────┘ │
│                                                     │
└─────────────────────────────────────────────────────┘
```

## Implementation Phases

### Phase 1: Core Game Logic
- [ ] Define state/delta/action types in `flow-demo/src/types.rs`
- [ ] Implement game state management in DO
- [ ] Add asset fetching from PFP City API
- [ ] Handle turn logic and match detection

### Phase 2: Component Migration
- [ ] Create `MemoryCard` component in `shared-ui/components/`
- [ ] Add flip animation CSS
- [ ] Add storybook story for MemoryCard
- [ ] Consider migrating full `AssetCard` for future use

### Phase 3: Frontend Integration
- [ ] Add game board Leptos component
- [ ] Add player list with scores
- [ ] Wire up card click → FlipCard action
- [ ] Handle state updates and animations

### Phase 4: Polish
- [ ] Add lobby/waiting room UI
- [ ] Add game end screen with results
- [ ] Add sound effects (optional)
- [ ] Add streak/combo notifications
- [ ] Mobile responsive layout

## Decisions

1. **Asset source**: PFP City API - server fetches assets and distributes via flow protocol
2. **Solo mode**: Yes - good for practice
3. **Room management**: Single room for MVP, expand later
4. **Persistence**: Future feature
5. **Card back design**: Custom Black Flag themed (asset TBD, black fill placeholder)
6. **Anti-cheat**: Trust clients for now
7. **Spectators**: Yes - can join mid-game to watch
8. **Grid sizes**: 8x8 (32 pairs, 64 cards)
9. **Mode selection**: Host picks in lobby

## Dependencies

**Worker:**
- `reqwest` or `worker` fetch for PFP City API calls
- Existing `ui-flow-protocol` for state sync

**Frontend:**
- `MemoryCard` component (new)
- Existing `connection-status` component
- CSS animations for card flip

**Shared:**
- Asset types (may need new crate or add to `ui-flow-protocol`)

## File Structure

```
shared-ui/
├── components/
│   ├── memory_card.rs       # NEW: Memory game card component
│   └── asset_card.rs        # FUTURE: Migrated from augminted-bots
├── workers/
│   └── flow-demo/
│       ├── src/
│       │   ├── types.rs     # Add MemoryGameState, etc.
│       │   ├── session.rs   # Add memory game handlers
│       │   └── assets.rs    # NEW: PFP City API integration
│       └── frontend/
│           └── src/
│               ├── components/
│               │   ├── game_board.rs  # NEW: Grid of cards
│               │   └── player_list.rs # NEW: Scores & turns
│               └── lib.rs   # Add game mode routing
```

## Status

**Status:** Design phase

**Next steps:**
1. Review design with stakeholder
2. Decide on asset source approach
3. Begin Phase 1 implementation
