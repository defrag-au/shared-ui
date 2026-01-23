//! Memory Card Leptos Component
//!
//! A flippable card for the memory matching game. Shows a card back when
//! face-down and an asset image when face-up.
//!
//! ## Props
//!
//! - `asset_id` - Cardano asset ID (policy_id + asset_name hex) for IIIF image
//! - `name` - Name shown when flipped
//! - `flipped` - Whether the card is face-up
//! - `matched` - Whether the card has been matched (stays revealed, shows glow)
//! - `matched_by` - Name of player who matched this card
//! - `disabled` - Whether the card can be clicked
//! - `on_click` - Callback when card is clicked
//! - `on_load` - Callback when card image has loaded
//!
//! ## Usage
//!
//! ```ignore
//! <MemoryCard
//!     asset_id="b3dab69f...506972617465313839"
//!     name="Captain Jack"
//!     flipped=is_flipped
//!     matched=is_matched
//!     on_click=move |_| { flip_card(); }
//!     on_load=move |_| { card_loaded(); }
//! />
//! ```

use crate::asset_card::AssetCard;
use crate::image_card::CardSize;
use leptos::*;
use scss_macros::scss;

/// Compiled SCSS styles for MemoryCard
const COMPONENT_STYLES: &str = scss!("src/styles/memory_card.scss");

/// Memory card size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MemoryCardSize {
    /// 80px
    Xs,
    /// 100px (default for memory games)
    #[default]
    Sm,
    /// 120px
    Md,
    /// 160px
    Lg,
}

impl MemoryCardSize {
    fn css_class(&self) -> &'static str {
        match self {
            MemoryCardSize::Xs => "memory-card--xs",
            MemoryCardSize::Sm => "memory-card--sm",
            MemoryCardSize::Md => "memory-card--md",
            MemoryCardSize::Lg => "memory-card--lg",
        }
    }

    fn to_inner_card_size(&self) -> CardSize {
        // Map to appropriate CardSize for inner AssetCard image resolution
        match self {
            MemoryCardSize::Xs | MemoryCardSize::Sm => CardSize::Xs,
            MemoryCardSize::Md => CardSize::Sm,
            MemoryCardSize::Lg => CardSize::Sm,
        }
    }
}

/// Memory card component - a flippable wrapper around AssetCard
#[component]
pub fn MemoryCard(
    /// Cardano asset ID for IIIF image generation
    #[prop(into, optional)]
    asset_id: Option<MaybeSignal<String>>,
    /// Asset name (shown when flipped)
    #[prop(into, optional)]
    name: Option<MaybeSignal<String>>,
    /// Card size (default: Sm = 100px)
    #[prop(optional, default = MemoryCardSize::Sm)]
    size: MemoryCardSize,
    /// Whether card is face-up
    #[prop(into)]
    flipped: MaybeSignal<bool>,
    /// Whether card has been matched
    #[prop(into, optional, default = false.into())]
    matched: MaybeSignal<bool>,
    /// Name of player who matched this card
    #[prop(into, optional)]
    matched_by: Option<MaybeSignal<String>>,
    /// Whether card is disabled (can't be clicked)
    #[prop(into, optional, default = false.into())]
    disabled: MaybeSignal<bool>,
    /// Click callback
    #[prop(into, optional)]
    on_click: Option<Callback<()>>,
    /// Image loaded callback
    #[prop(into, optional)]
    on_load: Option<Callback<()>>,
) -> impl IntoView {
    let size_class = size.css_class();
    let inner_card_size = size.to_inner_card_size();

    let card_class = move || {
        let mut classes = vec!["memory-card", size_class];
        if flipped.get() || matched.get() {
            classes.push("memory-card--flipped");
        }
        if matched.get() {
            classes.push("memory-card--matched");
        }
        if disabled.get() {
            classes.push("memory-card--disabled");
        }
        classes.join(" ")
    };

    let handle_click = move |_| {
        if !disabled.get() && !matched.get() {
            if let Some(cb) = on_click {
                cb.call(());
            }
        }
    };

    // Convert matched_by to memo for reactive access
    let matched_by_value: Memo<Option<String>> =
        create_memo(move |_| matched_by.as_ref().map(|m| m.get()));

    view! {
        <style>{COMPONENT_STYLES}</style>
        <div class=card_class on:click=handle_click>
            <div class="memory-card__inner">
                // Front (card back - shown when face-down)
                <div class="memory-card__front">
                    <div class="memory-card__back-design">
                        <div class="memory-card__skull"></div>
                    </div>
                </div>

                // Back (card face - shown when flipped)
                <div class="memory-card__back">
                    <AssetCard
                        asset_id=Signal::derive({
                            let asset_id = asset_id.clone();
                            move || asset_id.as_ref().map(|a| a.get()).unwrap_or_default()
                        })
                        name=Signal::derive({
                            let name = name.clone();
                            move || name.as_ref().map(|n| n.get()).unwrap_or_default()
                        })
                        size=inner_card_size
                        is_static=true
                        on_load=move |()| {
                            if let Some(cb) = on_load {
                                cb.call(());
                            }
                        }
                    />

                    {move || matched_by_value.get().map(|by| view! {
                        <div class="memory-card__matched-by">{by}</div>
                    })}
                </div>
            </div>
        </div>
    }
}
