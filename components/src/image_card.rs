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
//! - `accent-color` - Optional accent/tier color for top bar
//! - `static` - If present, card is non-interactive (no hover effect)
//! - `show-name` - If present, show name overlay (default: hidden)
//!
//! ## Events
//!
//! - `card-click` - Dispatched when card is clicked (if not static)
//!
//! ## Usage
//!
//! ```html
//! <image-card
//!     image-url="https://example.com/image.png"
//!     name="My Image"
//!     show-name
//! ></image-card>
//! ```

use crate::render_to_shadow;
use custom_elements::CustomElement;
use primitives::{dispatch_event, on_click};
use scss_macros::scss_inline;
use web_sys::HtmlElement;

/// Image card custom element
#[derive(Default)]
pub struct ImageCard {
    image_url: String,
    name: String,
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
        let mut classes = vec!["image-card"];
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
            format!(
                r#"<img class="image-card__image" src="{}" alt="{}" loading="lazy" />"#,
                html_escape(&self.image_url),
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

        if let Some(shadow) = element.shadow_root() {
            if let Ok(Some(card)) = shadow.query_selector(".image-card") {
                let host = element.clone();
                on_click(&card, move |_| {
                    dispatch_event(&host, "card-click");
                });
            }
        }
    }
}

impl CustomElement for ImageCard {
    fn observed_attributes() -> &'static [&'static str] {
        &["image-url", "name", "accent-color", "static", "show-name"]
    }

    fn constructor(&mut self, this: &HtmlElement) {
        self.image_url = this.get_attribute("image-url").unwrap_or_default();
        self.name = this.get_attribute("name").unwrap_or_default();
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
            "accent-color" => self.accent_color = new_value,
            "static" => self.is_static = new_value.is_some(),
            "show-name" => self.show_name = new_value.is_some(),
            _ => {}
        }

        render_to_shadow(this, &self.render_html());
        self.setup_click_handler(this);
    }

    fn inject_children(&mut self, this: &HtmlElement) {
        render_to_shadow(this, &self.render_html());
        self.setup_click_handler(this);
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
    r#"
    :host {
        display: block;
        width: 100%;
        max-width: 160px;
    }

    .image-card {
        position: relative;
        border-radius: 8px;
        overflow: hidden;
        background: var(--card-bg, #1a1a2e);
        cursor: pointer;
        transition: transform 0.15s ease, box-shadow 0.15s ease;
        width: 100%;

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
"#
);
