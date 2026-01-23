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

## Leptos 0.8 (CRITICAL)

This workspace uses **Leptos 0.8**. The API changed significantly from 0.6.

**Key differences from 0.6:**

| 0.6 Pattern | 0.8 Pattern |
|-------------|-------------|
| `use leptos::*` | `use leptos::prelude::*` |
| `create_signal(value)` | `signal(value)` |
| `create_memo(fn)` | `Memo::new(fn)` |
| `create_effect(fn)` | `Effect::new(fn)` |
| `MaybeSignal<T>` | `Signal<T>` |
| `callback.call(args)` | `callback.run(args)` |
| `.into_view()` (type erasure) | `.into_any()` |
| `mount_to_body(App).forget()` | `let _ = mount_to_body(App);` |

**Send + Sync requirements:**

Leptos 0.8 requires closures in views to be `Send + Sync`. For `Rc<RefCell<...>>` patterns common in CSR WASM apps, wrap with `SendWrapper`:

```rust
use send_wrapper::SendWrapper;

let ws: SendWrapper<Rc<RefCell<Option<WebSocket>>>> = 
    SendWrapper::new(Rc::new(RefCell::new(None)));
```

**CollectView trait:**

The `collect_view()` method requires an explicit import:

```rust
use leptos::prelude::CollectView;

items.iter().map(|item| view! { <li>{item}</li> }).collect_view()
```

**Full migration guide:** See `docs/leptos-0.6-to-0.8-migration.md` for comprehensive patterns and examples.

## DOM Construction (CRITICAL)

**DO NOT use `maud` or any HTML string templating in this workspace.**

The `primitives` crate provides proper DOM construction utilities. Use these instead:

- `primitives::dom::create_element()` - Create elements with classes
- `primitives::dom::on_click()`, `on_input()`, etc. - Attach event listeners
- `primitives::reactive::bind_text_content()`, `bind_class()` - Reactive bindings
- `primitives::component::create_shadow_root()` - Shadow DOM setup

**Why not maud/innerHTML:**
- Destroys and recreates DOM nodes on every update (inefficient)
- Loses event listeners attached to child elements
- Cannot do fine-grained reactive updates with `futures-signals`
- Potential XSS vectors if not careful with escaping

**Correct pattern:**
```rust
use primitives::dom::{create_element, on_click};
use primitives::component::create_shadow_root;

fn build_dom(shadow: &ShadowRoot) {
    let container = create_element("div", &["my-class"]);
    let button = create_element("button", &["btn"]);
    button.set_text_content(Some("Click me"));
    
    on_click(&button, |_| { /* handler */ });
    
    container.append_child(&button);
    shadow.append_child(&container);
}
```

**Wrong pattern:**
```rust
// DO NOT DO THIS
use maud::html;
shadow.set_inner_html(&html! { div { button { "Click me" } } }.into_string());
```

## Styling Components (CRITICAL)

**Use `scss-macros` for component styles, not raw CSS strings.**

The `scss-macros` crate provides compile-time SCSS compilation:

- `scss!("path/to/file.scss")` - Compile external SCSS file
- `scss_inline!(r#"..."#)` - Compile inline SCSS

**Correct pattern:**
```rust
use scss_macros::scss_inline;

const STYLES: &str = scss_inline!(r#"
    :host {
        display: inline-flex;
    }
    .my-component {
        padding: 1rem;
        &:hover {
            background: rgba(0, 0, 0, 0.1);
        }
    }
"#);
```

Or for larger stylesheets, use external files:
```rust
use scss_macros::scss;

const STYLES: &str = scss!("src/components/my_component.scss");
```

**Benefits:**
- Full SCSS syntax (variables, nesting, mixins)
- Compile-time compilation - errors caught at build time
- Compressed output for smaller bundles
- No runtime overhead

## Relationship to Other Workspaces

- **augminted-bots**: Game/Discord bots - consumes primitives and components for widget-map, DropEditor, etc.
- **cnft.dev-workers**: NFT platform - consumes components for admin UIs (Leptos)
- **survival-guide**: Original wallet-core patterns came from here

## Adding New Components

1. Create module in `components/src/`
2. Implement `CustomElement` trait from `custom-elements` crate
3. Use shadow DOM for style encapsulation
4. Use `primitives` for DOM construction (not maud/innerHTML)
5. Use `scss_inline!` or `scss!` for styles
6. Add to `define_all()` registration in `components/src/lib.rs`
7. **Add a story to the storybook** (see below)

## Storybook (CRITICAL)

All components MUST have a story in the storybook at `_storybook/`.

**Location:** `_storybook/src/lib.rs`

**Running the storybook:**
```bash
cd _storybook
trunk serve
```

**Adding a new story:**

1. Add variant to the `Story` enum:
```rust
enum Story {
    // ...existing stories
    ConnectionStatus,  // Add new story
}
```

2. Add to `Story::all()` array

3. Implement `label()` and `category()` for the new variant:
```rust
fn label(&self) -> &'static str {
    match self {
        Story::ConnectionStatus => "Connection Status",
        // ...
    }
}

fn category(&self) -> &'static str {
    match self {
        Story::ConnectionStatus => "Components",
        // ...
    }
}
```

4. Add render function and wire it up in `render_story()`:
```rust
fn render_story(story: Story) {
    match story {
        Story::ConnectionStatus => render_connection_status_story(),
        // ...
    }
}

fn render_connection_status_story() -> Element {
    // Use primitives to build the story UI
    // Show the component in different states
    // Include code examples
}
```

**Story structure pattern:**
- Header with title and description
- Live demo section with interactive examples
- Code example showing usage
