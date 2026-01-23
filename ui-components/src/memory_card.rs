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

/// Memory card component - a flippable wrapper around AssetCard
#[component]
pub fn MemoryCard(
    /// Cardano asset ID for IIIF image generation
    #[prop(into, optional)]
    asset_id: Option<MaybeSignal<String>>,
    /// Asset name (shown when flipped)
    #[prop(into, optional)]
    name: Option<MaybeSignal<String>>,
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
    let card_class = move || {
        let mut classes = vec!["memory-card"];
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
                        size=CardSize::Md
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

const COMPONENT_STYLES: &str = r##"
.memory-card {
    position: relative;
    width: 100%;
    aspect-ratio: 1;
    cursor: pointer;
    perspective: 1000px;
    transition: transform 0.1s ease;
}

.memory-card:hover:not(.memory-card--disabled):not(.memory-card--matched) {
    transform: scale(1.02);
}

.memory-card:active:not(.memory-card--disabled):not(.memory-card--matched) {
    transform: scale(0.98);
}

.memory-card--disabled {
    cursor: not-allowed;
    opacity: 0.7;
}

.memory-card--matched {
    cursor: default;
}

.memory-card--matched .memory-card__inner {
    box-shadow: 0 0 8px rgba(255, 215, 0, 0.3);
    animation: matched-glow 2s ease-in-out infinite;
}

.memory-card__inner {
    position: relative;
    width: 100%;
    height: 100%;
    transform-style: preserve-3d;
    transition: transform 0.5s ease;
    border-radius: 8px;
}

.memory-card--flipped .memory-card__inner {
    transform: rotateY(180deg);
}

.memory-card__front,
.memory-card__back {
    position: absolute;
    width: 100%;
    height: 100%;
    backface-visibility: hidden;
    border-radius: 8px;
    overflow: hidden;
}

.memory-card__front {
    background: #1a1a2e;
    border: 2px solid #2a2a4e;
}

.memory-card__back {
    background: #1a1a2e;
    border: 2px solid #2a2a4e;
    transform: rotateY(180deg);
    position: relative;
}

.memory-card__back-design {
    width: 100%;
    height: 100%;
    background: linear-gradient(135deg, #0d0d1a 0%, #1a1a2e 50%, #0d0d1a 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
}

.memory-card__back-design::before {
    content: "";
    position: absolute;
    inset: 8px;
    border: 2px solid rgba(255, 215, 0, 0.3);
    border-radius: 4px;
}

.memory-card__back-design::after {
    content: "";
    position: absolute;
    inset: 4px;
    background:
        linear-gradient(135deg, rgba(255, 215, 0, 0.2) 0%, transparent 20%),
        linear-gradient(225deg, rgba(255, 215, 0, 0.2) 0%, transparent 20%),
        linear-gradient(315deg, rgba(255, 215, 0, 0.2) 0%, transparent 20%),
        linear-gradient(45deg, rgba(255, 215, 0, 0.2) 0%, transparent 20%);
}

.memory-card__skull {
    width: 40%;
    height: 40%;
    background: rgba(255, 255, 255, 0.1);
    mask-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='currentColor'%3E%3Cpath d='M12 2C6.48 2 2 6.48 2 12c0 3.31 1.61 6.24 4.09 8.05L6 22l2-2h8l2 2-.09-1.95C20.39 18.24 22 15.31 22 12c0-5.52-4.48-10-10-10zm-2 14H8v-2h2v2zm0-4H8V8h2v4zm6 4h-2v-2h2v2zm0-4h-2V8h2v4z'/%3E%3C/svg%3E");
    -webkit-mask-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='currentColor'%3E%3Cpath d='M12 2C6.48 2 2 6.48 2 12c0 3.31 1.61 6.24 4.09 8.05L6 22l2-2h8l2 2-.09-1.95C20.39 18.24 22 15.31 22 12c0-5.52-4.48-10-10-10zm-2 14H8v-2h2v2zm0-4H8V8h2v4zm6 4h-2v-2h2v2zm0-4h-2V8h2v4z'/%3E%3C/svg%3E");
    mask-size: contain;
    -webkit-mask-size: contain;
    mask-repeat: no-repeat;
    -webkit-mask-repeat: no-repeat;
    mask-position: center;
    -webkit-mask-position: center;
    z-index: 1;
}

.memory-card__matched-by {
    position: absolute;
    top: 0.25rem;
    right: 0.25rem;
    padding: 0.15rem 0.4rem;
    background: rgba(255, 215, 0, 0.9);
    color: #000;
    font-size: 0.6rem;
    font-weight: 600;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    border-radius: 4px;
    white-space: nowrap;
    z-index: 10;
}

@keyframes matched-glow {
    0%, 100% {
        box-shadow: 0 0 6px rgba(255, 215, 0, 0.2);
    }
    50% {
        box-shadow: 0 0 10px rgba(255, 215, 0, 0.35);
    }
}
"##;
