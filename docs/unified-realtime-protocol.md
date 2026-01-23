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

All messages are MessagePack-encoded binary frames.

### Server → Client

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "snake_case")]
pub enum ServerMessage<State, Delta, Event> {
    // ─────────────────────────────────────────────────────────────
    // Connection Lifecycle
    // ─────────────────────────────────────────────────────────────
    
    /// Connection established, server ready
    Connected {
        /// Protocol version for compatibility checking
        protocol_version: u8,
        /// Server-assigned connection ID (for debugging/logging)
        connection_id: String,
    },
    
    /// Response to client ping
    Pong {
        /// Echo back the client's timestamp for latency measurement
        client_ts: u64,
        /// Server timestamp
        server_ts: u64,
    },
    
    // ─────────────────────────────────────────────────────────────
    // State Synchronization (Snapshot + Delta)
    // ─────────────────────────────────────────────────────────────
    
    /// Full state snapshot - sent on connect and on resync requests
    Snapshot {
        /// Complete state
        state: State,
        /// Sequence number for this state
        seq: u64,
        /// Server timestamp when snapshot was generated
        timestamp: u64,
    },
    
    /// Incremental state update
    Delta {
        /// Changes to apply
        delta: Delta,
        /// New sequence number after applying this delta
        seq: u64,
        /// Server timestamp
        timestamp: u64,
    },
    
    // ─────────────────────────────────────────────────────────────
    // Notifications (Domain Events)
    // ─────────────────────────────────────────────────────────────
    
    /// Application event notification
    Notify {
        /// Domain identifier (e.g., "world", "widget_bridge", "rewards")
        domain: String,
        /// Event type within the domain
        event_type: String,
        /// Event payload (domain-specific)
        event: Event,
        /// Optional correlation ID (links to triggering action)
        correlation_id: Option<OpId>,
    },
    
    // ─────────────────────────────────────────────────────────────
    // Action Feedback (Optimistic UI)
    // ─────────────────────────────────────────────────────────────
    
    /// Progress update for an in-flight action
    Progress {
        op_id: OpId,
        /// Progress percentage (0-100), if determinable
        percent: Option<u8>,
        /// Human-readable status message
        message: Option<String>,
    },
    
    /// Action completed successfully
    ActionOk {
        op_id: OpId,
        /// Optional result data
        result: Option<serde_json::Value>,
    },
    
    /// Action failed
    ActionErr {
        op_id: OpId,
        /// Error code for programmatic handling
        code: Option<String>,
        /// Human-readable error message
        message: String,
    },
    
    // ─────────────────────────────────────────────────────────────
    // Errors
    // ─────────────────────────────────────────────────────────────
    
    /// Server-initiated error (not tied to specific action)
    Error {
        /// Error code
        code: Option<String>,
        /// Error message
        message: String,
        /// Whether client should disconnect
        fatal: bool,
    },
}
```

### Client → Server

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "snake_case")]
pub enum ClientMessage<Action> {
    // ─────────────────────────────────────────────────────────────
    // Connection Lifecycle
    // ─────────────────────────────────────────────────────────────
    
    /// Keepalive ping
    Ping {
        /// Client timestamp for latency measurement
        ts: u64,
    },
    
    /// Request state resynchronization
    Resync {
        /// Last known sequence number (server may send delta if close enough)
        last_seq: Option<u64>,
    },
    
    // ─────────────────────────────────────────────────────────────
    // Actions
    // ─────────────────────────────────────────────────────────────
    
    /// Client action with operation tracking
    Action {
        /// Operation ID for correlation
        op_id: OpId,
        /// The action payload
        action: Action,
    },
    
    // ─────────────────────────────────────────────────────────────
    // Subscriptions
    // ─────────────────────────────────────────────────────────────
    
    /// Subscribe to notification domain(s)
    Subscribe {
        /// Domains to subscribe to
        domains: Vec<String>,
    },
    
    /// Unsubscribe from notification domain(s)
    Unsubscribe {
        /// Domains to unsubscribe from
        domains: Vec<String>,
    },
}
```

## Type Parameters

The protocol is generic over three type parameters:

| Parameter | Purpose | Example |
|-----------|---------|---------|
| `State` | Full state for snapshots | `WorldSnapshot`, `GameState` |
| `Delta` | Incremental state changes | `TickDelta`, `GameDelta` |
| `Event` | Notification payloads | `WorldEvent`, `serde_json::Value` |
| `Action` | Client action payloads | `PlayerAction`, `Command` |

### FlowState Trait

State types should implement `FlowState` to enable delta application:

```rust
pub trait FlowState: Clone + Default {
    type Delta: DeserializeOwned;
    
    /// Apply a delta to update the state
    fn apply_delta(&mut self, delta: Self::Delta);
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

### Versioning

Protocol version is exchanged in `Connected` message. Version changes:
- **Patch** (0.0.x) - Additive changes, backward compatible
- **Minor** (0.x.0) - New message types, old clients ignore unknown
- **Major** (x.0.0) - Breaking changes, requires client update

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

## Open Questions

1. **Delta batching** - Should server batch multiple deltas into single message?
2. **Compression** - Add optional compression for large snapshots?
3. **Binary blobs** - How to handle large binary data (images, etc.)?
4. **Presence** - Add presence/typing indicators as first-class feature?
5. **History** - Support for fetching historical events/states?

## References

- [MessagePack Specification](https://msgpack.org/)
- [WebSocket Protocol (RFC 6455)](https://tools.ietf.org/html/rfc6455)
- Current implementations:
  - `augminted-bots/services/realtime/` - Current event-based system
  - `augminted-bots/services/microversus-replication/` - Snapshot+delta diffing
  - `cnft.dev-workers/frontends/flow/` - Original Flow pattern
