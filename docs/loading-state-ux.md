# Loading State UX for Realtime Connections

## Problem

When a page loads with a WebSocket connection, users experience a "flash" as the UI renders with default/empty state, then quickly updates once the connection is established and the first snapshot arrives.

## Proposed Solution

Combine **cached state restoration** with **progressive enhancement**:

1. **Cache last known state** in localStorage when receiving snapshots
2. **Restore cached state instantly** on page load (zero latency)
3. **Show "Connecting" status** with disabled controls
4. **Apply server snapshot** when received (usually no visible change)
5. **Enable controls** once connected

## Implementation Outline

### Frontend Changes

```rust
// On snapshot received - cache to localStorage
fn handle_snapshot(state: DemoState) {
    set_state(state.clone());
    cache_state_to_storage(&state);
}

// On app init - restore from cache before connecting
fn init() {
    if let Some(cached) = load_cached_state() {
        set_state(cached);
    }
    connect_websocket();
}
```

### Cache Key Strategy

Use room-specific keys to avoid showing wrong data:
```
flow_demo_state_{room_id}
```

### Edge Cases

| Scenario | Behavior |
|----------|----------|
| First visit (no cache) | Show empty state with "Connecting..." |
| Server state unchanged | No visible change after snapshot |
| Server state changed | State updates after snapshot arrives |
| Stale cache (old session) | Brief flash as state corrects |

### Optional Enhancements

- **Cache TTL**: Expire cached state after N hours to avoid very stale data
- **Skeleton loaders**: Show placeholders instead of empty lists while connecting
- **Fade transition**: CSS opacity transition when state first loads

## Alternatives Considered

1. **Full loading screen** - Blocks interaction, feels slower even if same wall-clock time
2. **Skeleton UI only** - Better than nothing but still shows "loading" state
3. **No caching, just fade-in** - Simple but doesn't leverage available data

## Status

Deferred - documented for future implementation.
