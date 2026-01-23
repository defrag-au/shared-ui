//! Image Card Web Component
//!
//! A basic card displaying an image with optional name overlay and accent color.
//! Uses Shadow DOM for style isolation.
//!
//! This is the foundation component - for Cardano NFT assets with automatic
//! IIIF URL generation, use `<asset-card>` instead.
//!
//! ## Attributes
//!
//! - `image-url` - URL for the image
//! - `name` - Display name (shown in overlay and as title tooltip)
//! - `size` - Card size: "xs" (80px), "sm" (120px), "md" (240px), "lg" (400px), "xl" (800px)
//! - `accent-color` - Optional accent/tier color for top bar
//! - `static` - If present, card is non-interactive (no hover effect)
//! - `show-name` - If present, show name overlay (default: hidden)
//!
//! ## Events
//!
//! - `card-click` - Dispatched when card is clicked (if not static)
//! - `image-loaded` - Dispatched when the image has finished loading
//!
//! ## Usage
//!
//! ```html
//! <image-card
//!     image-url="https://example.com/image.png"
//!     name="My Image"
//!     size="md"
//!     show-name
//! ></image-card>
//! ```

use crate::render_to_shadow;
use custom_elements::CustomElement;
use phf::phf_map;
use primitives::{dispatch_event, on_click};
use scss_macros::scss_inline;
use web_sys::HtmlElement;

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

/// Parse size attribute string to CardSize
pub fn parse_card_size(s: &str) -> CardSize {
    CARD_SIZE_MAP
        .get(s.to_lowercase().as_str())
        .copied()
        .unwrap_or_default()
}

/// Image card custom element
#[derive(Default)]
pub struct ImageCard {
    image_url: String,
    name: String,
    size: CardSize,
    accent_color: Option<String>,
    is_static: bool,
    show_name: bool,
}

impl ImageCard {
    /// Register the custom element. Call once at app startup.
    pub fn define() {
        <Self as CustomElement>::define("image-card");
    }

    /// Render HTML string for the component
    fn render_html(&self) -> String {
        let card_classes = self.build_card_classes();
        let accent_bar = self.build_accent_bar();
        let image_html = self.build_image_html();
        let name_overlay = self.build_name_overlay();

        format!(
            r#"<style>{COMPONENT_STYLES}</style>
<div class="{card_classes}" title="{}">
    {accent_bar}
    <div class="image-card__image-wrapper">
        {image_html}
        {name_overlay}
    </div>
</div>"#,
            html_escape(&self.name)
        )
    }

    fn build_card_classes(&self) -> String {
        let size_class = format!("image-card--{}", self.size.class_suffix());
        let mut classes = vec!["image-card", &size_class];
        if self.is_static {
            classes.push("image-card--static");
        }
        classes.join(" ")
    }

    fn build_accent_bar(&self) -> String {
        if let Some(color) = &self.accent_color {
            format!(
                r#"<div class="image-card__accent" style="background-color: {}"></div>"#,
                html_escape(color)
            )
        } else {
            String::new()
        }
    }

    fn build_image_html(&self) -> String {
        if !self.image_url.is_empty() {
            // Check cache for preloaded blob URL, fall back to original URL
            let url = crate::image_cache::get_cached_url(&self.image_url)
                .unwrap_or_else(|| self.image_url.clone());
            format!(
                r#"<img class="image-card__image" src="{}" alt="{}" loading="lazy" />"#,
                html_escape(&url),
                html_escape(&self.name)
            )
        } else {
            r#"<div class="image-card__placeholder"></div>"#.to_string()
        }
    }

    fn build_name_overlay(&self) -> String {
        if self.show_name && !self.name.is_empty() {
            format!(
                r#"<div class="image-card__name">{}</div>"#,
                html_escape(&self.name)
            )
        } else {
            String::new()
        }
    }

    /// Setup click handler after rendering
    fn setup_click_handler(&self, element: &HtmlElement) {
        if self.is_static {
            return;
        }

        let (shadow, host) = primitives::get_shadow_and_host(element);
        if let Ok(Some(card)) = shadow.query_selector(".image-card") {
            on_click(&card, move |_| {
                dispatch_event(&host, "card-click");
            });
        }
    }

    /// Setup image load handler to dispatch image-loaded event
    fn setup_image_load_handler(&self, element: &HtmlElement) {
        let (shadow, host) = primitives::get_shadow_and_host(element);
        if let Ok(Some(img)) = shadow.query_selector(".image-card__image") {
            primitives::setup_image_load_handler(&img, move || {
                dispatch_event(&host, "image-loaded");
            });
        }
    }
}

impl CustomElement for ImageCard {
    fn observed_attributes() -> &'static [&'static str] {
        &[
            "image-url",
            "name",
            "size",
            "accent-color",
            "static",
            "show-name",
        ]
    }

    fn constructor(&mut self, this: &HtmlElement) {
        self.image_url = this.get_attribute("image-url").unwrap_or_default();
        self.name = this.get_attribute("name").unwrap_or_default();
        self.size = this
            .get_attribute("size")
            .map(|s| parse_card_size(&s))
            .unwrap_or_default();
        self.accent_color = this.get_attribute("accent-color");
        self.is_static = this.has_attribute("static");
        self.show_name = this.has_attribute("show-name");
    }

    fn attribute_changed_callback(
        &mut self,
        this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        match name.as_str() {
            "image-url" => self.image_url = new_value.unwrap_or_default(),
            "name" => self.name = new_value.unwrap_or_default(),
            "size" => self.size = new_value.map(|s| parse_card_size(&s)).unwrap_or_default(),
            "accent-color" => self.accent_color = new_value,
            "static" => self.is_static = new_value.is_some(),
            "show-name" => self.show_name = new_value.is_some(),
            _ => {}
        }

        render_to_shadow(this, &self.render_html());
        self.setup_click_handler(this);
        self.setup_image_load_handler(this);
    }

    fn inject_children(&mut self, this: &HtmlElement) {
        render_to_shadow(this, &self.render_html());
        self.setup_click_handler(this);
        self.setup_image_load_handler(this);
    }
}

/// Simple HTML escaping
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

const COMPONENT_STYLES: &str = scss_inline!(
    r##"
    :host {
        display: block;
        width: 100%;
    }

    .image-card {
        position: relative;
        border-radius: 8px;
        overflow: hidden;
        background: var(--card-bg, #1a1a2e);
        cursor: pointer;
        transition: transform 0.15s ease, box-shadow 0.15s ease;
        width: 100%;

        // Size variants
        &--xs {
            max-width: 80px;
            border-radius: 4px;

            .image-card__name {
                font-size: 0.6rem;
                padding: 0.2rem 0.3rem;
            }

            .image-card__accent {
                height: 2px;
            }
        }

        &--sm {
            max-width: 120px;
            border-radius: 6px;

            .image-card__name {
                font-size: 0.7rem;
                padding: 0.25rem 0.4rem;
            }

            .image-card__accent {
                height: 3px;
            }
        }

        &--md {
            max-width: 240px;

            .image-card__name {
                font-size: 0.85rem;
                padding: 0.4rem 0.6rem;
            }

            .image-card__accent {
                height: 4px;
            }
        }

        &--lg {
            max-width: 400px;
            border-radius: 10px;

            .image-card__name {
                font-size: 1rem;
                padding: 0.5rem 0.75rem;
            }

            .image-card__accent {
                height: 5px;
            }
        }

        &--xl {
            max-width: 800px;
            border-radius: 12px;

            .image-card__name {
                font-size: 1.1rem;
                padding: 0.6rem 1rem;
            }

            .image-card__accent {
                height: 6px;
            }
        }

        &:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
        }

        &--static {
            cursor: default;

            &:hover {
                transform: none;
                box-shadow: none;
            }
        }

        &__accent {
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 4px;
            z-index: 1;
        }

        &__image-wrapper {
            position: relative;
            aspect-ratio: 1;
            overflow: hidden;
        }

        &__image {
            width: 100%;
            height: 100%;
            object-fit: cover;
        }

        &__placeholder {
            width: 100%;
            height: 100%;
            background: linear-gradient(135deg, #2a2a3e 0%, #1a1a2e 100%);
            display: flex;
            align-items: center;
            justify-content: center;

            &::after {
                content: "";
                width: 40%;
                height: 40%;
                background: rgba(255, 255, 255, 0.1);
                border-radius: 50%;
            }
        }

        &__name {
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
    }
"##
);
