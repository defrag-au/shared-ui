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
//! ## Styles
//!
//! Include the component styles once at your app root:
//!
//! ```ignore
//! use ui_components::STYLES;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <style>{STYLES}</style>
//!         // ... rest of app
//!     }
//! }
//! ```
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
//!     size=CardSize::Sm
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
mod asset_detail_card;
mod asset_grid;
mod asset_picker;
mod badge;
mod button;
mod button_group;
mod card;
mod connection_status;
mod empty_state;
mod header;
mod helpers;
pub mod image_cache;
mod image_card;
mod memory_card;
mod modal;
mod progress_bar;
mod select;
mod stat_pill;
mod styles;
mod tabs;
mod toast;

pub use asset_cache::{AssetCache, PreloadAsset};
pub use asset_card::{generate_iiif_url, AssetCard, IiifSize};
pub use asset_detail_card::AssetDetailCard;
pub use asset_grid::AssetGrid;
pub use asset_picker::{AssetPicker, PickerAsset};
pub use badge::{Badge, BadgeSize, BadgeVariant};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use button_group::ButtonGroup;
pub use card::Card;
pub use connection_status::{ConnectionState, ConnectionStatus};
pub use empty_state::EmptyState;
pub use header::PageHeader;
pub use helpers::children_fn;
pub use image_card::{parse_card_size, CardSize, ImageCard};
pub use memory_card::MemoryCard;
pub use modal::Modal;
pub use progress_bar::ProgressBar;
pub use select::{Select, SelectOption};
pub use stat_pill::{StatPill, StatPillColor, StatPillColorInput, StatPillSize};
pub use styles::STYLES;
pub use tabs::{TabDef, TabPanel, TabPanelControlled, Tabs, TabsContext};
pub use toast::{
    try_use_toasts, use_toasts, Toast, ToastContainer, ToastContext, ToastKind, ToastProvider,
    DEFAULT_TOAST_DURATION_MS,
};
