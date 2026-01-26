# Launcher Templates

HTML launcher templates for WASM widgets/games that need to handle authentication before the Rust code starts.

## macroquad.html

Launcher for macroquad-based games. Handles JWT token extraction and passes identity to the game.

### Usage

1. Copy `macroquad.html` to your project (e.g., `client/web/index.html`)
2. Replace placeholders:
   - `<!-- TITLE -->` - Your game title
   - `<!-- WASM_FILE -->` - Your compiled WASM filename (e.g., `my-game.wasm`)
3. Use `ui_loader_macros::macroquad_main!` in your Rust code

### How it works

1. **Token extraction**: Reads `?token=xxx` from URL
2. **JWT decoding**: Decodes the payload (validation happens server-side)
3. **Identity setup**: Sets `window.__WIDGET_IDENTITY__` with user info
4. **WASM loading**: Loads the macroquad game

### Rust side

```rust
ui_loader_macros::macroquad_main! {
    init: |identity: Option<Identity>| {
        // identity contains user_id, display_name, avatar_url, token
        MyGame::new(identity)
    },
    update: |game: &mut MyGame| {
        game.update();
    },
    draw: |game: &MyGame| {
        game.draw();
    },
}
```

### Identity struct

The `Identity` struct is defined by the macro:

```rust
pub struct Identity {
    pub user_id: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub token: Option<String>,  // For authenticated API calls
}
```

### Anonymous mode

If no token is provided, `identity` will be `None`. Your game can decide whether to allow anonymous play or show an error.

### Discord bot integration

The Discord bot creates a signed JWT and redirects users:

```
/play command -> Bot creates JWT -> Redirect to:
https://game.example.com/?token=eyJ...
```
