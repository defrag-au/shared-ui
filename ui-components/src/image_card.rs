//! Image Card Leptos Component
//!
//! A basic card displaying an image with optional name overlay and accent color.
//!
//! ## Props
//!
//! - `image_url` - URL for the image
//! - `name` - Display name (shown in overlay and as title tooltip)
//! - `size` - Card size: Xs (80px), Sm (120px), Md (240px), Lg (400px), Xl (800px)
//! - `accent_color` - Optional accent/tier color for top bar
//! - `is_static` - If true, card is non-interactive (no hover effect)
//! - `show_name` - If true, show name overlay
//! - `on_click` - Callback when card is clicked (if not static)
//! - `on_load` - Callback when image has loaded
//!
//! ## Usage
//!
//! ```ignore
//! <ImageCard
//!     image_url="https://example.com/image.png"
//!     name="My Image"
//!     size=CardSize::Md
//!     show_name=true
//!     on_click=move |_| { log!("clicked"); }
//! />
//! ```

use leptos::*;
use phf::phf_map;

/// Card size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardSize {
    /// 80px - Extra small, for dense grids
    Xs,
    /// 120px - Small, default size
    #[default]
    Sm,
    /// 240px - Medium, good for featured items
    Md,
    /// 400px - Large, detailed view
    Lg,
    /// 800px - Extra large, hero/spotlight
    Xl,
}

impl CardSize {
    /// Get the pixel width for this size
    pub fn pixels(&self) -> u16 {
        match self {
            CardSize::Xs => 80,
            CardSize::Sm => 120,
            CardSize::Md => 240,
            CardSize::Lg => 400,
            CardSize::Xl => 800,
        }
    }

    /// Get the CSS class suffix for this size
    pub fn class_suffix(&self) -> &'static str {
        match self {
            CardSize::Xs => "xs",
            CardSize::Sm => "sm",
            CardSize::Md => "md",
            CardSize::Lg => "lg",
            CardSize::Xl => "xl",
        }
    }
}

/// Map from attribute string to CardSize variant
static CARD_SIZE_MAP: phf::Map<&'static str, CardSize> = phf_map! {
    "xs" => CardSize::Xs,
    "extra-small" => CardSize::Xs,
    "sm" => CardSize::Sm,
    "small" => CardSize::Sm,
    "md" => CardSize::Md,
    "medium" => CardSize::Md,
    "lg" => CardSize::Lg,
    "large" => CardSize::Lg,
    "xl" => CardSize::Xl,
    "extra-large" => CardSize::Xl,
};

/// Parse size string to CardSize
pub fn parse_card_size(s: &str) -> CardSize {
    CARD_SIZE_MAP
        .get(s.to_lowercase().as_str())
        .copied()
        .unwrap_or_default()
}

/// Image card component
#[component]
pub fn ImageCard(
    /// URL for the image
    #[prop(into, optional)]
    image_url: Option<MaybeSignal<String>>,
    /// Display name (shown in overlay and as title tooltip)
    #[prop(into, optional)]
    name: Option<MaybeSignal<String>>,
    /// Card size
    #[prop(optional, default = CardSize::Sm)]
    size: CardSize,
    /// Accent color for top bar
    #[prop(into, optional)]
    accent_color: Option<MaybeSignal<String>>,
    /// If true, card is non-interactive
    #[prop(optional)]
    is_static: bool,
    /// If true, show name overlay
    #[prop(optional)]
    show_name: bool,
    /// Click callback
    #[prop(into, optional)]
    on_click: Option<Callback<()>>,
    /// Image loaded callback
    #[prop(into, optional)]
    on_load: Option<Callback<()>>,
) -> impl IntoView {
    let size_class = format!("image-card--{}", size.class_suffix());

    let card_class = move || {
        let mut classes = vec!["image-card", &size_class];
        if is_static {
            classes.push("image-card--static");
        }
        classes.join(" ")
    };

    let handle_click = move |_| {
        if !is_static {
            if let Some(cb) = on_click {
                cb.call(());
            }
        }
    };

    // Convert name to a signal for reactive access
    let name_signal: Memo<String> =
        create_memo(move |_| name.as_ref().map(|n| n.get()).unwrap_or_default());

    // Convert image_url to a signal
    let url_signal: Memo<String> =
        create_memo(move |_| image_url.as_ref().map(|u| u.get()).unwrap_or_default());

    // Convert accent_color to a signal
    let accent_signal: Memo<Option<String>> =
        create_memo(move |_| accent_color.as_ref().map(|c| c.get()));

    view! {
        <style>{COMPONENT_STYLES}</style>
        <div
            class=card_class
            title=move || name_signal.get()
            on:click=handle_click
        >
            {move || accent_signal.get().map(|color| view! {
                <div class="image-card__accent" style=format!("background-color: {color}")></div>
            })}

            <div class="image-card__image-wrapper">
                {move || {
                    let url = url_signal.get();
                    if !url.is_empty() {
                        // Check cache for preloaded blob URL
                        let resolved_url = crate::image_cache::get_cached_url(&url)
                            .unwrap_or(url);
                        view! {
                            <img
                                class="image-card__image"
                                src=resolved_url
                                alt=move || name_signal.get()
                                loading="lazy"
                                on:load=move |_| {
                                    if let Some(cb) = on_load {
                                        cb.call(());
                                    }
                                }
                            />
                        }.into_view()
                    } else {
                        view! {
                            <div class="image-card__placeholder"></div>
                        }.into_view()
                    }
                }}

                {move || {
                    if show_name {
                        let n = name_signal.get();
                        if !n.is_empty() {
                            Some(view! {
                                <div class="image-card__name">{n}</div>
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }}
            </div>
        </div>
    }
}

const COMPONENT_STYLES: &str = r##"
.image-card {
    position: relative;
    border-radius: 8px;
    overflow: hidden;
    background: var(--card-bg, #1a1a2e);
    cursor: pointer;
    transition: transform 0.15s ease, box-shadow 0.15s ease;
    width: 100%;
}

.image-card--xs { max-width: 80px; border-radius: 4px; }
.image-card--xs .image-card__name { font-size: 0.6rem; padding: 0.2rem 0.3rem; }
.image-card--xs .image-card__accent { height: 2px; }

.image-card--sm { max-width: 120px; border-radius: 6px; }
.image-card--sm .image-card__name { font-size: 0.7rem; padding: 0.25rem 0.4rem; }
.image-card--sm .image-card__accent { height: 3px; }

.image-card--md { max-width: 240px; }
.image-card--md .image-card__name { font-size: 0.85rem; padding: 0.4rem 0.6rem; }
.image-card--md .image-card__accent { height: 4px; }

.image-card--lg { max-width: 400px; border-radius: 10px; }
.image-card--lg .image-card__name { font-size: 1rem; padding: 0.5rem 0.75rem; }
.image-card--lg .image-card__accent { height: 5px; }

.image-card--xl { max-width: 800px; border-radius: 12px; }
.image-card--xl .image-card__name { font-size: 1.1rem; padding: 0.6rem 1rem; }
.image-card--xl .image-card__accent { height: 6px; }

.image-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.image-card--static {
    cursor: default;
}

.image-card--static:hover {
    transform: none;
    box-shadow: none;
}

.image-card__accent {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 4px;
    z-index: 1;
}

.image-card__image-wrapper {
    position: relative;
    aspect-ratio: 1;
    overflow: hidden;
}

.image-card__image {
    width: 100%;
    height: 100%;
    object-fit: cover;
}

.image-card__placeholder {
    width: 100%;
    height: 100%;
    background: linear-gradient(135deg, #2a2a3e 0%, #1a1a2e 100%);
    display: flex;
    align-items: center;
    justify-content: center;
}

.image-card__placeholder::after {
    content: "";
    width: 40%;
    height: 40%;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 50%;
}

.image-card__name {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    padding: 0.35rem 0.5rem;
    background: rgba(0, 0, 0, 0.75);
    color: white;
    font-size: 0.75rem;
    font-weight: 500;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    text-align: center;
}
"##;
