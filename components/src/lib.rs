//! Shared UI web components
//!
//! Reusable custom elements that can be used in any web framework.
//! All components use Shadow DOM for style isolation.
//!
//! ## Available Components
//!
//! - `<connection-status>` - WebSocket/realtime connection indicator with click-to-reconnect
//! - `<memory-card>` - Flippable card for memory matching game
//!
//! ## Usage
//!
//! ```ignore
//! // Register all components at app startup
//! components::define_all();
//!
//! // Use in HTML
//! <connection-status status="connected"></connection-status>
//! <memory-card image-url="https://..." name="Asset Name"></memory-card>
//! ```

mod connection_status;
mod memory_card;

pub use connection_status::{ConnectionState, ConnectionStatus};
pub use memory_card::MemoryCard;

use std::sync::Once;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

/// Render HTML content into an element's shadow root.
pub fn render_to_shadow(element: &HtmlElement, html: &str) {
    if let Some(shadow) = element.shadow_root() {
        shadow.set_inner_html(html);
    }
}

static INIT: Once = Once::new();

/// Register all custom elements. Safe to call multiple times.
#[wasm_bindgen]
pub fn define_all() {
    INIT.call_once(|| {
        ConnectionStatus::define();
        MemoryCard::define();
    });
}
