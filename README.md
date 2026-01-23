# shared-ui

Framework-agnostic UI building blocks for Cardano WASM applications. This workspace provides reusable components, real-time state synchronization, and utilities that work with any frontend framework (Seed, Leptos, Yew, etc.).

## Crates

### Core Libraries

| Crate | Description |
|-------|-------------|
| **primitives** | Reactive helpers, DOM utilities, and base traits for building web components |
| **ui-core** | Framework-agnostic utilities: auth state, HTTP helpers, JWT parsing, error handling |
| **ui-components** | Reusable web components with Shadow DOM isolation |
| **ui-flow** | Real-time state synchronization with snapshot + delta pattern |
| **ui-flow-protocol** | Wire protocol types for MessagePack-based realtime communication |
| **ui-loader** | Pre-framework loading orchestrator for widget bootstrap |
| **ui-toast** | Toast notification data model and state management |
| **wallet-core** | CIP-30 Cardano wallet detection, connection, and signing |
| **scss-macros** | Compile-time SCSS to CSS compilation macros |

### Web Components (ui-components)

All components use Shadow DOM for style isolation and work with any framework:

- `<image-card>` - Basic image card with optional name overlay
- `<asset-card>` - Cardano NFT card with automatic IIIF URL generation
- `<asset-cache>` - Non-visual image preloader for instant display
- `<connection-status>` - WebSocket connection indicator with reconnect
- `<memory-card>` - Flippable card for memory matching games

```html
<!-- Register once at startup -->
<script>ui_components.define_all();</script>

<!-- Use anywhere -->
<asset-card 
    asset-id="{policy_id}{asset_name_hex}" 
    name="Pirate #189" 
    size="md"
    show-name>
</asset-card>
```

### Real-time State (ui-flow)

Efficient state synchronization over WebSockets using snapshot + delta pattern:

```rust
use ui_flow::{FlowConnection, FlowState};

#[derive(Clone, Default, Deserialize)]
struct GameState { score: u32 }

#[derive(Deserialize)]
enum GameDelta { ScoreChanged(u32) }

impl FlowState for GameState {
    type Delta = GameDelta;
    
    fn apply_delta(&mut self, delta: Self::Delta) {
        match delta {
            GameDelta::ScoreChanged(score) => self.score = score,
        }
    }
}
```

## Development

### Prerequisites

- Rust with `wasm32-unknown-unknown` target
- [Trunk](https://trunkrs.dev/) for building WASM apps

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
```

### Build

```bash
# Check all crates
cargo check --all

# Run tests
cargo test --all

# Build for WASM
cargo build --target wasm32-unknown-unknown --release
```

### Storybook

Interactive component showcase for development:

```bash
cd _storybook
trunk serve
# Open http://localhost:8080
```

### Workers

The `workers/` directory contains Cloudflare Worker demos:

```bash
cd workers/flow-demo
wrangler dev
```

## Project Structure

```
shared-ui/
├── primitives/          # DOM helpers, reactive bindings
├── ui-core/             # Auth, HTTP, errors, runtime
├── ui-components/       # Web components (<asset-card>, etc.)
├── ui-flow/             # WebSocket state sync client
├── ui-flow-protocol/    # Wire protocol types (MessagePack)
├── ui-loader/           # Pre-framework loading orchestrator
├── ui-toast/            # Toast notification model
├── wallet-core/         # CIP-30 wallet integration
├── scss-macros/         # Compile-time SCSS processing
├── _storybook/          # Component development UI
└── workers/
    └── flow-demo/       # Demo worker with memory game
```

## License

MIT
