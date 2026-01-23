//! Shared UI Leptos Components
//!
//! Reusable Leptos components for Cardano applications.
//!
//! ## Available Components
//!
//! - `ImageCard` - Basic image card with optional name overlay
//! - `AssetCard` - Cardano NFT asset card with IIIF URL generation (wraps ImageCard)
//! - `MemoryCard` - Flippable card for memory matching game (wraps AssetCard)
//! - `ConnectionStatus` - WebSocket/realtime connection indicator
//! - `AssetCache` - Non-visual component for preloading NFT images
//!
//! ## Usage
//!
//! ```ignore
//! use ui_components::{ImageCard, AssetCard, MemoryCard, ConnectionStatus, CardSize};
//!
//! // Basic image card
//! <ImageCard
//!     image_url="https://..."
//!     name="Image Name"
//!     show_name=true
//! />
//!
//! // Cardano NFT with automatic IIIF URL
//! <AssetCard
//!     asset_id="{policy_id}{asset_name_hex}"
//!     name="Pirate #189"
//!     show_name=true
//! />
//!
//! // Memory game card
//! <MemoryCard
//!     asset_id="..."
//!     name="Asset Name"
//!     flipped=is_flipped
//!     on_click=move |_| { flip(); }
//! />
//!
//! // Connection status
//! <ConnectionStatus
//!     status=conn_status
//!     on_reconnect=move |_| { reconnect(); }
//! />
//! ```

mod asset_cache;
mod asset_card;
mod connection_status;
pub mod image_cache;
mod image_card;
mod memory_card;

pub use asset_cache::{AssetCache, PreloadAsset};
pub use asset_card::{generate_iiif_url, AssetCard, IiifSize};
pub use connection_status::{ConnectionState, ConnectionStatus};
pub use image_card::{parse_card_size, CardSize, ImageCard};
pub use memory_card::{MemoryCard, MemoryCardSize};
