//! Shared UI web components
//!
//! Reusable custom elements that can be used in any web framework.
//! All components use Shadow DOM for style isolation.
//!
//! ## Available Components
//!
//! - `<image-card>` - Basic image card with optional name overlay
//! - `<asset-card>` - Cardano NFT asset card with IIIF URL generation (wraps image-card)
//! - `<connection-status>` - WebSocket/realtime connection indicator with click-to-reconnect
//! - `<memory-card>` - Flippable card for memory matching game (wraps image-card)
//!
//! ## Usage
//!
//! ```ignore
//! // Register all components at app startup
//! components::define_all();
//!
//! // Use in HTML - basic image
//! <image-card image-url="https://..." name="Image Name" show-name></image-card>
//!
//! // Cardano NFT with automatic IIIF URL
//! <asset-card asset-id="{policy_id}{asset_name_hex}" name="Pirate #189" show-name></asset-card>
//!
//! // Connection status
//! <connection-status status="connected"></connection-status>
//!
//! // Memory game card
//! <memory-card image-url="https://..." name="Asset Name"></memory-card>
//! ```

mod asset_card;
mod connection_status;
mod image_card;
mod memory_card;

pub use asset_card::{AssetCard, IiifSize};
pub use connection_status::{ConnectionState, ConnectionStatus};
pub use image_card::{CardSize, ImageCard};
pub use memory_card::MemoryCard;

use std::sync::Once;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

/// Render HTML content into an element's shadow root.
///
/// Handles the custom-elements JS shim quirk where `element` may be either
/// the host element or the shadow root itself. See `primitives::get_shadow_and_host`.
pub fn render_to_shadow(element: &HtmlElement, html: &str) {
    let (shadow, _host) = primitives::get_shadow_and_host(element);
    shadow.set_inner_html(html);
}

static INIT: Once = Once::new();

/// Register all custom elements. Safe to call multiple times.
#[wasm_bindgen]
pub fn define_all() {
    INIT.call_once(|| {
        // Register in dependency order - ImageCard first since others wrap it
        ImageCard::define();
        AssetCard::define();
        ConnectionStatus::define();
        MemoryCard::define();
    });
}
