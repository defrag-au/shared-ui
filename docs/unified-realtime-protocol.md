# Unified Realtime Protocol

A single WebSocket protocol for real-time state synchronization and event notifications, using MessagePack binary serialization.

## Overview

This protocol combines two complementary patterns over a single WebSocket connection:

1. **State Sync** - Snapshot + delta pattern for UI state consistency
2. **Notifications** - Domain-tagged events for rich application events

Both patterns share connection management, authentication, and serialization infrastructure.

## Design Goals

- **Single connection** - One WebSocket per client handles all real-time needs
- **Binary protocol** - MessagePack throughout for efficiency and consistency
- **Framework agnostic** - Works with Seed, Leptos, Yew, or vanilla JS
- **Optimistic UI** - Built-in operation tracking for responsive interfaces
- **Gradual adoption** - Can be adopted incrementally alongside existing systems

## Protocol Messages

All messages are MessagePack-encoded binary frames using u16 integer tags for efficient wire format and extensibility.

### Message Tag Ranges

Tags are organized into logical ranges for clarity and future expansion:

| Range | Category | Notes |
|-------|----------|-------|
| 0-999 | Core protocol | Connection lifecycle, errors |
| 1000-1999 | State sync | Snapshots, deltas |
| 2000-2999 | Presence | User presence tracking |
| 3000-3999 | Signalling | WebRTC connection setup |
| 4000-4999 | Notifications | Application events |
| 5000-5999 | Action feedback | Optimistic UI support |
| 6000-65535 | Reserved | Future protocol extensions |

This gives ~60,000 slots for future expansion while keeping related messages grouped.

### Server → Client

```rust
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Message type tag for ServerMessage variants
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ServerTag {
    // Core protocol (0-999)
    Connected = 0,
    Pong = 1,
    Error = 2,
    
    // State sync (1000-1999)
    Snapshot = 1000,
    Delta = 1001,
    Deltas = 1002,
    
    // Presence (2000-2999)
    Presence = 2000,
    
    // WebRTC signalling (3000-3999)
    Signal = 3000,
    
    // Notifications (4000-4999)
    Notify = 4000,
    
    // Action feedback (5000-5999)
    Progress = 5000,
    ActionOk = 5001,
    ActionErr = 5002,
}

/// Server-to-client message
/// 
/// Generic over:
/// - `State`: Full state type for snapshots
/// - `Delta`: Incremental state change type
/// - `Event`: Application-specific event type (flows through Notify)
#[derive(Serialize, Deserialize)]
pub struct ServerMessage<State, Delta, Event> {
    /// Message type tag (u16)
    pub t: ServerTag,
    /// Message payload (variant-specific)
    #[serde(flatten)]
    pub payload: ServerPayload<State, Delta, Event>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerPayload<State, Delta, Event> {
    // ─────────────────────────────────────────────────────────────
    // Connection Lifecycle (0-999)
    // ─────────────────────────────────────────────────────────────
    
    /// Connection established, server ready (tag: 0)
    Connected {
        /// Protocol version for compatibility checking
        protocol_version: u8,
        /// Server-assigned connection ID (for debugging/logging)
        connection_id: String,
    },
    
    /// Response to client ping (tag: 1)
    Pong {
        /// Echo back the client's timestamp for latency measurement
        client_ts: u64,
        /// Server timestamp
        server_ts: u64,
    },
    
    /// Server-initiated error (tag: 2)
    Error {
        /// Error code
        code: Option<String>,
        /// Error message
        message: String,
        /// Whether client should disconnect
        fatal: bool,
    },
    
    // ─────────────────────────────────────────────────────────────
    // State Synchronization (1000-1999)
    // ─────────────────────────────────────────────────────────────
    
    /// Full state snapshot - sent on connect and on resync requests (tag: 1000)
    Snapshot {
        /// Complete state
        state: State,
        /// Sequence number for this state
        seq: u64,
        /// Server timestamp when snapshot was generated
        timestamp: u64,
    },
    
    /// Incremental state update (tag: 1001)
    Delta {
        /// Changes to apply
        delta: Delta,
        /// New sequence number after applying this delta
        seq: u64,
        /// Server timestamp
        timestamp: u64,
    },
    
    /// Batched deltas for catch-up scenarios (tag: 1002)
    Deltas {
        /// Ordered list of deltas to apply
        deltas: Vec<Delta>,
        /// Final sequence number after all deltas applied
        seq: u64,
        /// Server timestamp
        timestamp: u64,
    },
    
    // ─────────────────────────────────────────────────────────────
    // Presence (2000-2999)
    // ─────────────────────────────────────────────────────────────
    
    /// Presence update - who's connected (tag: 2000)
    Presence {
        /// Users currently connected to this resource
        users: Vec<PresenceInfo>,
    },
    
    // ─────────────────────────────────────────────────────────────
    // WebRTC Signalling (3000-3999)
    // ─────────────────────────────────────────────────────────────
    
    /// WebRTC signalling message from another peer (tag: 3000)
    Signal {
        /// Source user ID
        from_user_id: String,
        /// Signalling payload
        signal: SignalPayload,
    },
    
    // ─────────────────────────────────────────────────────────────
    // Notifications (4000-4999)
    // ─────────────────────────────────────────────────────────────
    
    /// Application event notification (tag: 4000)
    /// 
    /// The `Event` type parameter allows applications to define their own
    /// strongly-typed event enums. All application-specific events flow
    /// through this single variant.
    Notify {
        /// Domain identifier (e.g., "world", "widget_bridge", "rewards")
        domain: String,
        /// Event payload - application-defined type
        event: Event,
        /// Optional correlation ID (links to triggering action)
        correlation_id: Option<OpId>,
    },
    
    // ─────────────────────────────────────────────────────────────
    // Action Feedback (5000-5999)
    // ─────────────────────────────────────────────────────────────
    
    /// Progress update for an in-flight action (tag: 5000)
    Progress {
        op_id: OpId,
        /// Progress percentage (0-100), if determinable
        percent: Option<u8>,
        /// Human-readable status message
        message: Option<String>,
    },
    
    /// Action completed successfully (tag: 5001)
    ActionOk {
        op_id: OpId,
        /// Optional result data
        result: Option<serde_json::Value>,
    },
    
    /// Action failed (tag: 5002)
    ActionErr {
        op_id: OpId,
        /// Error code for programmatic handling
        code: Option<String>,
        /// Human-readable error message
        message: String,
    },
}
```

### Client → Server

```rust
/// Message type tag for ClientMessage variants
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ClientTag {
    // Core protocol (0-999)
    Ping = 0,
    Resync = 1,
    
    // Actions (1000-1999)
    Action = 1000,
    
    // Subscriptions (2000-2999)
    Subscribe = 2000,
    Unsubscribe = 2001,
    
    // WebRTC signalling (3000-3999)
    Signal = 3000,
}

/// Client-to-server message
/// 
/// Generic over:
/// - `Action`: Application-specific action type
#[derive(Serialize, Deserialize)]
pub struct ClientMessage<Action> {
    /// Message type tag (u16)
    pub t: ClientTag,
    /// Message payload (variant-specific)
    #[serde(flatten)]
    pub payload: ClientPayload<Action>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientPayload<Action> {
    // ─────────────────────────────────────────────────────────────
    // Connection Lifecycle (0-999)
    // ─────────────────────────────────────────────────────────────
    
    /// Keepalive ping (tag: 0)
    Ping {
        /// Client timestamp for latency measurement
        ts: u64,
    },
    
    /// Request state resynchronization (tag: 1)
    Resync {
        /// Last known sequence number (server may send delta if close enough)
        last_seq: Option<u64>,
    },
    
    // ─────────────────────────────────────────────────────────────
    // Actions (1000-1999)
    // ─────────────────────────────────────────────────────────────
    
    /// Client action with operation tracking (tag: 1000)
    Action {
        /// Operation ID for correlation
        op_id: OpId,
        /// The action payload - application-defined type
        action: Action,
    },
    
    // ─────────────────────────────────────────────────────────────
    // Subscriptions (2000-2999)
    // ─────────────────────────────────────────────────────────────
    
    /// Subscribe to notification domain(s) (tag: 2000)
    Subscribe {
        /// Domains to subscribe to
        domains: Vec<String>,
    },
    
    /// Unsubscribe from notification domain(s) (tag: 2001)
    Unsubscribe {
        /// Domains to unsubscribe from
        domains: Vec<String>,
    },
    
    // ─────────────────────────────────────────────────────────────
    // WebRTC Signalling (3000-3999)
    // ─────────────────────────────────────────────────────────────
    
    /// Send WebRTC signalling message to a peer (tag: 3000)
    Signal {
        /// Target user ID
        target_user_id: String,
        /// Signalling payload (offer, answer, ICE candidate)
        signal: SignalPayload,
    },
}
```

## Type Parameters

The protocol is generic over four type parameters:

| Parameter | Purpose | Example |
|-----------|---------|---------|
| `State` | Full state for snapshots | `WorldSnapshot`, `GameState` |
| `Delta` | Incremental state changes | `TickDelta`, `GameDelta` |
| `Event` | Application-specific notifications | `WorldEvent`, `RewardEvent` |
| `Action` | Client action payloads | `PlayerAction`, `Command` |

### Application Events (the `Event` type)

The protocol keeps its message types fixed - applications extend the protocol through the generic `Event` type parameter in the `Notify` variant. This means:

- **Protocol is stable** - Adding new application events never requires protocol changes
- **Type-safe events** - Applications define their event enum, getting compile-time type checking
- **Domain routing** - The `domain` field in `Notify` allows event filtering/routing

**Example application event type:**

```rust
/// Application-specific events for a game world
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorldEvent {
    /// An encounter was completed
    EncounterCompleted {
        encounter_id: EntityId,
        position: HexCoord,
        loot: Vec<LootItem>,
    },
    
    /// A reward was earned
    RewardEarned {
        user_id: String,
        amount: u64,
        source: RewardSource,
    },
    
    /// Entity state changed (for notifications, not full delta sync)
    EntityNotification {
        entity_id: EntityId,
        notification_type: String,
        data: serde_json::Value,
    },
}

// Usage: ServerMessage<WorldSnapshot, TickDelta, WorldEvent>
```

**For applications that need multiple event types**, use an outer enum:

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "domain", rename_all = "snake_case")]
pub enum AppEvent {
    World(WorldEvent),
    Rewards(RewardEvent),
    System(SystemEvent),
}
```

The `domain` field in `Notify` provides additional routing context but the event type itself can also carry domain information through its structure.

### FlowState Trait

State types should implement `FlowState` to enable delta application:

```rust
pub trait FlowState: Clone + Default {
    type Delta: DeserializeOwned;
    
    /// Apply a delta to update the state
    fn apply_delta(&mut self, delta: Self::Delta);
}
```

### Supporting Types

```rust
/// Presence information for a connected user
#[derive(Serialize, Deserialize, Clone)]
pub struct PresenceInfo {
    pub user_id: String,
    /// Display name if available
    pub name: Option<String>,
    /// User's current status
    pub status: PresenceStatus,
    /// When they connected (unix ms)
    pub connected_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum PresenceStatus {
    Active,
    Idle,
    Away,
}

/// WebRTC signalling payload
#[derive(Serialize, Deserialize, Clone)]
pub enum SignalPayload {
    /// SDP offer
    Offer { sdp: String },
    /// SDP answer
    Answer { sdp: String },
    /// ICE candidate
    IceCandidate {
        candidate: String,
        sdp_mid: Option<String>,
        sdp_m_line_index: Option<u16>,
    },
}
```

## Connection Lifecycle

```
┌─────────┐                              ┌─────────┐
│  Client │                              │  Server │
└────┬────┘                              └────┬────┘
     │                                        │
     │  ──── WebSocket Connect ────────────►  │
     │  ◄─── Connected { protocol_version } ─ │
     │                                        │
     │  ◄─── Snapshot { state, seq } ──────── │
     │                                        │
     │  ──── Subscribe { domains } ────────►  │
     │                                        │
     │           ┌─────────────────────────┐  │
     │           │   Normal Operation      │  │
     │           └─────────────────────────┘  │
     │                                        │
     │  ◄─── Delta { delta, seq } ─────────── │
     │  ◄─── Notify { domain, event } ─────── │
     │                                        │
     │  ──── Action { op_id, action } ─────►  │
     │  ◄─── Progress { op_id, ... } ──────── │
     │  ◄─── ActionOk { op_id } ───────────── │
     │                                        │
     │  ──── Ping { ts } ──────────────────►  │
     │  ◄─── Pong { client_ts, server_ts } ── │
     │                                        │
```

### Connection URL

```
wss://{host}/realtime/{resource_id}?token={jwt}
```

- `resource_id` - Identifies the resource to sync (e.g., world ID, room ID)
- `token` - JWT for authentication (can also be sent in first message)

### Reconnection

On disconnect:
1. Client attempts reconnection with exponential backoff
2. On reconnect, client sends `Resync { last_seq }`
3. Server responds with either:
   - `Snapshot` if too far behind or no last_seq
   - `Delta` batch if within recoverable range

## Serialization

### MessagePack Configuration

All messages use MessagePack with these settings:
- Structs serialize as maps (not positional arrays) for forward compatibility
- Enums use `{tag: "variant_name", ...fields}` format
- Timestamps are u64 milliseconds since Unix epoch
- Binary data uses MessagePack bin type

### No wasm_safe_serde Required

Unlike JSON serialization, MessagePack natively supports 64-bit integers. This eliminates the need for `wasm_safe_serde` workarounds that convert u64 to strings to avoid JavaScript's `Number.MAX_SAFE_INTEGER` (2^53-1) limitation.

With MessagePack:
- `u64` fields serialize/deserialize correctly without string conversion
- Discord snowflake IDs, timestamps, and tick counts work naturally
- Simpler derive macros - just `#[derive(Serialize, Deserialize)]`
- No `#[serde(with = "wasm_safe_serde::u64_required")]` annotations needed

### Wire Format

Messages are serialized as MessagePack maps with a `t` field (tag) followed by payload fields:

```
{
  "t": 1000,           // u16 tag: Snapshot
  "state": {...},      // State payload
  "seq": 42,
  "timestamp": 1706123456789
}
```

The tag is always the first field, allowing efficient dispatch without parsing the full payload. Unknown tags should be ignored (forward compatibility).

### Versioning

Protocol version is exchanged in `Connected` message. Version changes:
- **Patch** (0.0.x) - Additive changes, backward compatible
- **Minor** (0.x.0) - New message types, old clients ignore unknown tags
- **Major** (x.0.0) - Breaking changes, requires client update

**Adding new message types:**
1. Allocate a tag from the appropriate range (see Message Tag Ranges)
2. Add the variant to the tag enum and payload enum
3. Old clients will ignore the unknown tag (no breaking change)

## Notification Domains

Domains provide namespacing for different event types:

| Domain | Purpose | Example Events |
|--------|---------|----------------|
| `world` | Game world events | EntityUpdated, EncounterCompleted |
| `widget_bridge` | UI entity sync | PlacementFilled, EntityCreated |
| `rewards` | Reward notifications | RewardEarned, LootDropped |
| `system` | System notifications | MaintenanceScheduled |

### Domain Subscription

Clients subscribe to domains they're interested in:

```rust
// Subscribe to world events only
ClientMessage::Subscribe { domains: vec!["world".into()] }

// Subscribe to multiple domains
ClientMessage::Subscribe { domains: vec!["world".into(), "rewards".into()] }
```

Unsubscribed domains are filtered server-side (no bandwidth waste).

## Optimistic UI Pattern

For responsive UIs, clients can apply changes optimistically:

```
┌─────────┐                              ┌─────────┐
│  Client │                              │  Server │
└────┬────┘                              └────┬────┘
     │                                        │
     │  1. User clicks "Move North"           │
     │     - Generate OpId                    │
     │     - Apply optimistic state change    │
     │     - Show pending indicator           │
     │                                        │
     │  ──── Action { op_id, Move::North } ─► │
     │                                        │
     │  ◄─── Progress { op_id, "Moving..." }  │  2. Optional progress
     │                                        │
     │  ◄─── Delta { ... }  ───────────────── │  3. Server state update
     │                                        │
     │  ◄─── ActionOk { op_id } ───────────── │  4. Confirm success
     │     - Remove pending indicator         │
     │     - State already matches (delta)    │
     │                                        │
```

On `ActionErr`:
1. Client rolls back optimistic change
2. Shows error to user
3. State will match server (via deltas)

## Client Implementation

### Rust/WASM (ui-flow crate)

```rust
use ui_flow::{FlowConnection, FlowState, ServerMessage, Notification};

// Define state types
#[derive(Clone, Default, Deserialize)]
struct GameState { /* ... */ }

#[derive(Deserialize)]
enum GameDelta { /* ... */ }

impl FlowState for GameState {
    type Delta = GameDelta;
    fn apply_delta(&mut self, delta: Self::Delta) { /* ... */ }
}

// Connect
let conn = FlowConnection::<GameState, GameAction>::builder()
    .url("wss://example.com/realtime/world_123?token=...")
    .on_snapshot(|state, seq| {
        // Replace local state
        set_state(state);
    })
    .on_delta(|delta, seq| {
        // Apply incremental update
        update_state(|s| s.apply_delta(delta));
    })
    .on_notify(|domain, event_type, event| {
        // Handle notification
        match domain.as_str() {
            "world" => handle_world_event(event),
            "rewards" => show_reward_toast(event),
            _ => {}
        }
    })
    .on_status(|status| {
        // Update connection indicator
    })
    .subscribe(["world", "rewards"])
    .connect()?;

// Send action with tracking
let op_id = OpId::new();
conn.send_action(op_id, GameAction::Move { direction: "north" })?;
```

## Server Implementation

### Handler Trait

```rust
#[async_trait]
pub trait FlowHandler: Send + Sync {
    type State: Serialize + Send;
    type Delta: Serialize + Send;
    type Event: Serialize + Send;
    type Action: DeserializeOwned + Send;
    
    /// Get current state snapshot for a new connection
    async fn snapshot(&self, resource_id: &str) -> Result<(Self::State, u64)>;
    
    /// Get deltas since a sequence number (for resync)
    async fn deltas_since(&self, resource_id: &str, seq: u64) -> Result<Vec<(Self::Delta, u64)>>;
    
    /// Handle a client action
    async fn handle_action(
        &self,
        resource_id: &str,
        user_id: &str,
        op_id: OpId,
        action: Self::Action,
    ) -> Result<ActionResult>;
    
    /// Get subscribed domains for visibility filtering
    fn filter_notification(&self, user_id: &str, domain: &str, event: &Self::Event) -> bool;
}
```

### Broadcasting

```rust
// In tick processing or event handler
session.broadcast_delta(delta, new_seq).await;

// For notifications (filtered by subscription)
session.broadcast_notify("world", "entity_updated", event).await;

// For targeted notifications
session.notify_user(user_id, "rewards", "loot_dropped", event).await;
```

## Migration Strategy

### Phase 1: Add Protocol Support
- Implement unified protocol in `ui-flow` crate
- Add MessagePack serialization
- Maintain backward compatibility with existing JSON protocol

### Phase 2: Server Integration
- Add protocol negotiation (version in connect)
- Implement dual-protocol support in realtime worker
- Route based on client capability

### Phase 3: Client Migration
- New widgets use unified protocol directly
- Existing widgets migrate gradually
- Provide migration utilities/adapters

### Phase 4: Legacy Removal
- Remove JSON protocol paths
- Simplify realtime worker
- Update documentation

## Crate Structure

```
shared-ui/
├── ui-flow/
│   ├── src/
│   │   ├── lib.rs           # Public API
│   │   ├── protocol.rs      # Message types
│   │   ├── connection.rs    # WebSocket management
│   │   ├── state.rs         # FlowState trait
│   │   ├── operation.rs     # OpId, tracking
│   │   ├── status.rs        # ConnectionStatus
│   │   └── codec.rs         # MessagePack serialization
│   └── Cargo.toml
```

## Design Decisions

### Delta Batching

The server may batch multiple deltas into a single message for efficiency. This is useful when:
- Client reconnects and needs to catch up
- Multiple rapid changes occur within a short window

```rust
/// Batched deltas for catch-up scenarios
Deltas {
    /// Ordered list of deltas to apply
    deltas: Vec<Delta>,
    /// Final sequence number after all deltas applied
    seq: u64,
    /// Server timestamp
    timestamp: u64,
},
```

Clients apply deltas in order. The final `seq` represents state after all deltas.

### Compression (Future)

Optional compression for large snapshots will be negotiated during connection handshake:

```rust
// In Connected message (future)
Connected {
    protocol_version: u8,
    connection_id: String,
    compression: Option<Compression>, // None, Zstd, Lz4
}
```

Not implemented in v1 - added when snapshot sizes warrant it.

### Presence

Presence information (who's connected, activity status) is a first-class feature:

```rust
/// Server → Client: Presence update
Presence {
    /// Users currently connected to this resource
    users: Vec<PresenceInfo>,
},

#[derive(Serialize, Deserialize)]
struct PresenceInfo {
    user_id: String,
    /// Display name if available
    name: Option<String>,
    /// User's current status
    status: PresenceStatus,
    /// When they connected
    connected_at: u64,
}

#[derive(Serialize, Deserialize)]
enum PresenceStatus {
    Active,
    Idle,
    Away,
}
```

Clients receive presence updates when users join/leave/change status.

### History

Historical events and states are fetched via separate REST API, not through the WebSocket. The realtime protocol is for live synchronization only.

### WebRTC Signalling

The notification system supports WebRTC connection establishment through a `signalling` domain:

```rust
// Client → Server: Send signalling message to peer
ClientMessage::Signal {
    /// Target user ID
    target_user_id: String,
    /// Signalling payload (offer, answer, ICE candidate)
    signal: SignalPayload,
}

#[derive(Serialize, Deserialize)]
enum SignalPayload {
    /// SDP offer
    Offer { sdp: String },
    /// SDP answer  
    Answer { sdp: String },
    /// ICE candidate
    IceCandidate { candidate: String, sdp_mid: Option<String>, sdp_m_line_index: Option<u16> },
}

// Server → Client: Receive signalling message from peer
ServerMessage::Signal {
    /// Source user ID
    from_user_id: String,
    /// Signalling payload
    signal: SignalPayload,
}
```

This enables:
- Peer discovery via presence
- SDP offer/answer exchange
- ICE candidate trickling
- Establishing WebRTC data channels for direct P2P communication

The WebSocket acts as the signalling server; once WebRTC connection is established, high-bandwidth/low-latency data flows directly between peers.

## Future Considerations

- **Compression** - Zstd/Lz4 for large snapshots (negotiate in handshake)
- **Rooms/Channels** - Subscribe to multiple resources on single connection
- **Rate limiting** - Server-side throttling for misbehaving clients

## References

- [MessagePack Specification](https://msgpack.org/)
- [WebSocket Protocol (RFC 6455)](https://tools.ietf.org/html/rfc6455)
- Current implementations:
  - `augminted-bots/services/realtime/` - Current event-based system
  - `augminted-bots/services/microversus-replication/` - Snapshot+delta diffing
  - `cnft.dev-workers/frontends/flow/` - Original Flow pattern
