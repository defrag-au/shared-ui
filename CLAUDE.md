# shared-ui

Shared Rust/WASM UI primitives and web components for the defrag ecosystem.

## Overview

This workspace provides framework-agnostic UI building blocks:

- **primitives**: Reactive helpers, DOM utilities, web component base traits
- **wallet-core**: CIP-30 Cardano wallet detection, connection, and signing
- **components**: Reusable custom elements (wallet-connector, token-picker, etc.)

## Commands

```bash
cargo build                    # Build all crates
cargo test                     # Run tests
cargo clippy --all-targets     # Lint
cargo fmt                      # Format
```

## Architecture

```
shared-ui/
├── primitives/             # DOM helpers, reactive bindings
│   ├── reactive.rs         # futures-signals bindings (bind_class, etc.)
│   ├── dom.rs              # Element creation, event listeners
│   └── component.rs        # WebComponent trait, shadow DOM helpers
│
├── wallet-core/            # CIP-30 wallet integration
│   ├── cip30.rs            # JS bindings for wallet API
│   ├── types.rs            # WalletProvider, ConnectionState
│   ├── storage.rs          # localStorage persistence
│   └── error.rs            # Typed errors (not string matching!)
│
└── components/             # Web components (custom elements)
    └── lib.rs              # define_all() to register components
```

## Usage in Consumer Crates

### From augminted-bots widgets:

```toml
[dependencies]
primitives = { git = "https://github.com/defrag-crypto/shared-ui" }
wallet-core = { git = "https://github.com/defrag-crypto/shared-ui" }
components = { git = "https://github.com/defrag-crypto/shared-ui" }
```

### From cnft.dev-workers Leptos apps:

```toml
[dependencies]
components = { git = "https://github.com/defrag-crypto/shared-ui" }
```

Then in your app:

```rust
// Register all custom elements
components::define_all();

// Use in HTML/Leptos views
// <wallet-connector></wallet-connector>
// <token-picker></token-picker>
```

## Code Style

- Use inline format args: `format!("Hello {name}")` not `format!("Hello {}", name)`
- NEVER use string matching for error detection - use typed errors
- Web components should be framework-agnostic (no Leptos/Yew dependencies in components)
- Use `futures-signals` for reactivity, not framework-specific signals

## Relationship to Other Workspaces

- **augminted-bots**: Game/Discord bots - consumes primitives and components for widget-map, DropEditor, etc.
- **cnft.dev-workers**: NFT platform - consumes components for admin UIs (Leptos)
- **survival-guide**: Original wallet-core patterns came from here

## Adding New Components

1. Create module in `crates/components/src/`
2. Implement `WebComponent` trait from `primitives`
3. Use shadow DOM for style encapsulation
4. Add to `define_all()` registration
5. Document usage in this file
