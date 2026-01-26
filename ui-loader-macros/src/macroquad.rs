//! Macroquad framework macros for widget loading orchestration
//!
//! Provides `macroquad_main!` macro for macroquad game widgets with automatic
//! loading, authentication, and error handling via the HTML/JS launcher pattern.
//!
//! ## How It Works
//!
//! Unlike DOM-based frameworks (Leptos, Seed), macroquad renders to a canvas
//! and cannot display the loading UI directly. Instead:
//!
//! 1. The HTML launcher shows loading UI and handles authentication
//! 2. Once loaded, it passes identity info to WASM via JS interop
//! 3. The macroquad app starts with identity already resolved
//!
//! ## Usage
//!
//! The `init` function is async to support loading fonts, textures, etc.:
//!
//! ```ignore
//! ui_loader_macros::macroquad_main! {
//!     init: |identity: Option<Identity>| async move {
//!         let fonts = Fonts::load().await;
//!         MyGame::new(identity, fonts)
//!     },
//!
//!     update: |game: &mut MyGame| {
//!         game.update();
//!     },
//!
//!     draw: |game: &MyGame| {
//!         game.draw();
//!     },
//! }
//! ```
//!
//! ## Identity
//!
//! The `Identity` struct contains user info passed from the launcher:
//!
//! ```ignore
//! pub struct Identity {
//!     pub user_id: String,
//!     pub display_name: Option<String>,
//!     pub avatar_url: Option<String>,
//!     pub token: Option<String>,  // For API calls
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Token};

/// Input structure for macroquad_main! macro
pub struct MacroquadMainInput {
    pub init: Expr,
    pub update: Expr,
    pub draw: Expr,
}

impl Parse for MacroquadMainInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut init: Option<Expr> = None;
        let mut update: Option<Expr> = None;
        let mut draw: Option<Expr> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "init" => {
                    init = Some(input.parse()?);
                }
                "update" => {
                    update = Some(input.parse()?);
                }
                "draw" => {
                    draw = Some(input.parse()?);
                }
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("unknown field `{other}`, expected `init`, `update`, or `draw`"),
                    ));
                }
            }

            // Optional trailing comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(MacroquadMainInput {
            init: init.ok_or_else(|| input.error("missing `init` field"))?,
            update: update.ok_or_else(|| input.error("missing `update` field"))?,
            draw: draw.ok_or_else(|| input.error("missing `draw` field"))?,
        })
    }
}

/// Generate macroquad game entry point
///
/// This generates the `#[macroquad::main]` entry point that:
/// 1. Retrieves identity from JS (set by launcher)
/// 2. Calls init with the identity
/// 3. Runs the game loop with update/draw
pub fn generate(input: MacroquadMainInput) -> TokenStream {
    let init = &input.init;
    let update = &input.update;
    let draw = &input.draw;

    let expanded = quote! {
        /// Identity information passed from the launcher
        #[derive(Debug, Clone, Default)]
        pub struct Identity {
            pub user_id: String,
            pub display_name: Option<String>,
            pub avatar_url: Option<String>,
            pub token: Option<String>,
        }

        /// Retrieve identity from JS global (set by launcher before WASM init)
        fn get_identity_from_js() -> Option<Identity> {
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = window, js_name = __WIDGET_IDENTITY__)]
                static IDENTITY: JsValue;
            }

            if IDENTITY.is_undefined() || IDENTITY.is_null() {
                return None;
            }

            // Parse the identity object from JS
            let obj = js_sys::Object::try_from(&*IDENTITY)?;

            let get_string = |key: &str| -> Option<String> {
                let val = js_sys::Reflect::get(&obj, &JsValue::from_str(key)).ok()?;
                val.as_string()
            };

            Some(Identity {
                user_id: get_string("user_id").unwrap_or_default(),
                display_name: get_string("display_name"),
                avatar_url: get_string("avatar_url"),
                token: get_string("token"),
            })
        }

        #[macroquad::main("Game")]
        async fn main() {
            let identity = get_identity_from_js();

            let init_fn = #init;
            let update_fn = #update;
            let draw_fn = #draw;

            // Init is async to support loading fonts, textures, etc.
            let mut game = init_fn(identity).await;

            loop {
                update_fn(&mut game);
                draw_fn(&game);
                macroquad::prelude::next_frame().await;
            }
        }
    };

    TokenStream::from(expanded)
}
