# Leptos 0.6 → 0.8 Migration Guide

This guide documents the key differences between Leptos 0.6 and 0.8, compiled from official release notes and documentation. Use this as a reference when working with Leptos code in this workspace.

## Overview

Leptos 0.7 was a near-complete rewrite of the framework internals, with 0.8 adding smaller refinements. The main goals were:
- Maintain backwards compatibility for most user code
- Improve async handling and fix Suspense edge cases
- Enhance ergonomics (prop spreading, HTML shell access)
- Reduce binary size through statically-typed views

## Quick Reference: API Renames

| 0.6 Pattern | 0.8 Pattern |
|-------------|-------------|
| `use leptos::*;` | `use leptos::prelude::*;` |
| `create_signal(value)` | `signal(value)` |
| `create_rw_signal(value)` | `RwSignal::new(value)` |
| `create_memo(closure)` | `Memo::new(closure)` |
| `create_resource(source, fetcher)` | `Resource::new(source, fetcher)` |
| `create_effect(closure)` | `Effect::new(closure)` |
| `MaybeSignal<T>` | `Signal<T>` (deprecated in 0.7) |
| `view.into_view()` | `view.into_any()` (for type erasure) |

## Signal System Changes

### Creating Signals

**0.6:**
```rust
let (count, set_count) = create_signal(0);
let count_rw = create_rw_signal(0);
```

**0.8:**
```rust
let (count, set_count) = signal(0);
let count_rw = RwSignal::new(0);
```

### Reading Signals

Three methods available:
- `.get()` - Clones the value, tracks reactively (most common)
- `.read()` - Returns a read guard, no clone (efficient for `.len()` etc.)
- `.with(|val| ...)` - Takes closure with reference

**0.8 improvement - read guards:**
```rust
// 0.6: nested .with() calls for multiple signals
let len = move || long_vec.with(|v| short_vec.with(|v2| v.len() + v2.len()));

// 0.8: cleaner with .read()
let len = move || long_vec.read().len() + short_vec.read().len();
```

### Writing Signals

- `.set(value)` - Replace entire value
- `.write()` - Returns mutable guard
- `.update(|val| ...)` - Takes closure with mutable reference

### Thread Safety

Signals now require `Send + Sync` by default. For non-thread-safe types (like `Rc`, browser JS objects):

```rust
// Use _local variants
let (foo, set_foo) = signal_local(Rc::new("value"));
let rw = RwSignal::new_local(Rc::new("value"));
```

### MaybeSignal → Signal

`MaybeSignal<T>` is deprecated. Use `Signal<T>` instead, which now covers both plain values and reactive signals.

**Important:** `Signal<T>` no longer implements `From<Fn() -> T>`. Use `Signal::derive()` explicitly:

```rust
// 0.6: implicit conversion
let sig: Signal<bool> = (move || some_signal.get()).into();

// 0.8: explicit derivation
let sig: Signal<bool> = Signal::derive(move || some_signal.get());
```

### Arc Variants

New reference-counted signal types that are `Clone` but not `Copy`:
- `ArcRwSignal`, `ArcReadSignal`, `ArcWriteSignal`, `ArcMemo`

```rust
let arc_sig = ArcRwSignal::new(0);
let copy_sig: RwSignal<_> = arc_sig.into(); // Convert when needed
```

## View System Changes

### Statically-Typed Views

Views are no longer a `View` enum - they're statically typed. This improves binary size but affects branching.

**For conditional rendering, use `Either`:**
```rust
use leptos::either::Either;

if condition {
    Either::Left(view! { <p>"True"</p> })
} else {
    Either::Right(view! { <p>"False"</p> })
}
```

**Or use `.into_any()` for type erasure:**
```rust
if condition {
    view! { <p>"True"</p> }.into_any()
} else {
    view! { <p>"False"</p> }.into_any()
}
```

### Show Component

Syntax remains similar, but `fallback` cannot be optional:

```rust
<Show 
    when=move || value.get() > 5
    fallback=|| view! { <p>"Small"</p> }
>
    <p>"Big"</p>
</Show>

// For empty fallback, use unit type:
<Show when=move || condition.get() fallback=|| ()>
    <p>"Content"</p>
</Show>
```

## For Component Changes

### Basic Syntax

The `let` syntax is now preferred for cleaner code:

**0.6:**
```rust
<For
    each=move || items.get()
    key=|item| item.id
    children=move |item| {
        view! { <p>{item.name}</p> }
    }
/>
```

**0.8:**
```rust
<For
    each=move || items.get()
    key=|item| item.id
    let(item)
>
    <p>{item.name}</p>
</For>
```

### Destructuring in let

```rust
<For
    each=move || entries.get()
    key=|e| e.id
    let(Entry { id, value })
>
    <p>{id}": "{value}</p>
</For>
```

### ForEnumerate

For indexed iteration:
```rust
<ForEnumerate
    each=move || items.get()
    key=|item| item.id
    let(index, item)
>
    <p>{move || index.get()}": "{item.name}</p>
</ForEnumerate>
```

## Component Props

### #[prop(into)] with Signal<T>

The recommended pattern for flexible props:

```rust
#[component]
pub fn MyComponent(
    #[prop(into)] value: Signal<String>,
    #[prop(into, optional)] enabled: Signal<bool>,
) -> impl IntoView {
    // ...
}

// Usage - all these work:
<MyComponent value="static string" />
<MyComponent value=my_signal />
<MyComponent value=Signal::derive(move || computed()) />
```

### Passing Closures to Signal Props

Since `Signal<T>` no longer implements `From<Fn() -> T>`:

```rust
// This no longer works:
<MyComponent is_active=move || some_condition.get() />

// Do this instead:
<MyComponent is_active=Signal::derive(move || some_condition.get()) />

// Or for simple values:
<MyComponent is_active=false />
```

## Router Changes

### Imports

```rust
// 0.6
use leptos_router::*;

// 0.8
use leptos_router::components::*;
use leptos_router::hooks::*;
```

### Route Definitions

Routes now use the `path!()` macro and require a fallback:

**0.6:**
```rust
<Routes>
    <Route path="/" view=Home />
    <Route path="/posts/:id" view=Post />
</Routes>
```

**0.8:**
```rust
<Routes fallback=|| view! { <p>"404 Not Found"</p> }>
    <Route path=path!("/") view=Home />
    <Route path=path!("/posts/:id") view=Post />
</Routes>
```

### FlatRoutes vs ParentRoute

- Use `<FlatRoutes/>` for simple, non-nested routes
- Use `<ParentRoute/>` for routes with children/outlets

## New Features in 0.8

### Two-Way Binding

```rust
let text = RwSignal::new(String::new());
let checked = RwSignal::new(false);

view! {
    <input type="text" bind:value=text />
    <input type="checkbox" bind:checked=checked />
}
```

### Attribute Spreading

```rust
<MyComponent
    class:active=is_active
    style:color="red"
    on:click=handle_click
    {..}  // Everything after is HTML attribute
    title="tooltip"
    data-id="123"
/>
```

### WebSocket Server Functions

```rust
#[server(protocol = "Websocket")]
pub async fn streaming_fn() -> BoxedStream<String> {
    // Return streaming data
}
```

## Compilation Issues

### Recursion Limit

Large view hierarchies may hit recursion limits:

```rust
// Add to lib.rs or main.rs if needed
#![recursion_limit = "256"]
```

### Type Complexity (macOS Linker)

If hitting linker limits on macOS:

1. Use `lld` linker (add to `.cargo/config.toml`):
```toml
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

2. Enable component erasure for dev builds:
```bash
RUSTFLAGS="--cfg erase_components" cargo leptos serve
```

### Large Components

Use `.into_any()` on complex view components to reduce type complexity:

```rust
fn big_component() -> impl IntoView {
    view! {
        // lots of nested elements
    }.into_any()
}
```

## Migration Checklist

1. [ ] Update `Cargo.toml` dependencies to `leptos = "0.8"`
2. [ ] Change imports from `use leptos::*` to `use leptos::prelude::*`
3. [ ] Rename `create_signal` → `signal`, `create_rw_signal` → `RwSignal::new()`, etc.
4. [ ] Replace `MaybeSignal<T>` with `Signal<T>`
5. [ ] Add explicit `Signal::derive()` for closure-to-signal conversions
6. [ ] Update `<For>` components to use `let(item)` syntax (optional but cleaner)
7. [ ] Add `fallback` to all `<Routes>` components
8. [ ] Use `path!()` macro for route paths
9. [ ] Update router imports to `leptos_router::components::*` and `leptos_router::hooks::*`
10. [ ] Use `signal_local()` / `RwSignal::new_local()` for non-Send types
11. [ ] Add `.into_any()` for conditional view branches if needed
12. [ ] Test and add `#![recursion_limit = "256"]` if compilation fails

## Sources

- [Leptos 0.8.0 Release Notes](https://github.com/leptos-rs/leptos/releases/tag/v0.8.0)
- [Leptos 0.7.0 Release Notes](https://github.com/leptos-rs/leptos/releases/tag/v0.7.0)
- [Leptos Book - Working with Signals](https://book.leptos.dev/reactivity/working_with_signals.html)
- [Leptos Book - Iteration](https://book.leptos.dev/view/04b_iteration.html)
- [Leptos Book - Control Flow](https://book.leptos.dev/view/06_control_flow.html)
- [Migration Issues Discussion](https://github.com/leptos-rs/leptos/issues/3433)
