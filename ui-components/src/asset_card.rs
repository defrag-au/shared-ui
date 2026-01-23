//! Asset Card Web Component
//!
//! A card for displaying Cardano NFT assets. Wraps `<image-card>` and adds
//! automatic IIIF URL generation from asset IDs.
//!
//! ## Image URL Resolution
//!
//! The component generates IIIF URLs from the asset ID:
//! - Asset ID format: `{policy_id}{asset_name_hex}` (56+ chars)
//! - Generated URL: `https://iiif.hodlcroft.com/iiif/3/{policy_id}:{asset_name}/full/{size},/0/default.jpg`
//!
//! The IIIF image size is automatically selected based on card size:
//! - xs, sm, md, lg (≤400px): uses 400px IIIF image (cached, fast)
//! - xl (>400px): uses 1686px IIIF image (high resolution)
//!
//! ## Attributes
//!
//! - `asset-id` - Cardano asset ID (policy_id + asset_name hex) - generates IIIF URL
//! - `image-url` - Direct image URL (fallback when asset-id not available)
//! - `size` - Card size: "xs" (80px), "sm" (120px), "md" (240px), "lg" (400px), "xl" (800px)
//! - `name` - Display name of the asset (shown in overlay)
//! - `accent-color` - Optional accent/tier color for top bar
//! - `static` - If present, card is non-interactive (no hover effect)
//! - `show-name` - If present, show name overlay (default: hidden)
//!
//! ## Events
//!
//! - `card-click` - Dispatched when card is clicked (if not static), includes asset-id in detail
//!
//! ## Usage
//!
//! ```html
//! <!-- Small card (120px, uses 400px IIIF image) -->
//! <asset-card
//!     asset-id="b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839"
//!     name="Pirate #189"
//!     size="sm"
//!     show-name
//! ></asset-card>
//!
//! <!-- Extra large card (800px, uses 1686px IIIF image) -->
//! <asset-card
//!     asset-id="b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839"
//!     size="xl"
//!     name="Pirate #189"
//! ></asset-card>
//! ```

use crate::image_card::{CardSize, parse_card_size};
use crate::render_to_shadow;
use custom_elements::CustomElement;
use scss_macros::scss_inline;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// IIIF base URL for image lookups
const IIIF_BASE_URL: &str = "https://iiif.hodlcroft.com/iiif/3";

/// IIIF image size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IiifSize {
    /// 400px width - fast, cached thumbnails
    #[default]
    Thumb,
    /// 1686px width - high resolution
    Large,
}

impl IiifSize {
    /// Get the IIIF size parameter value
    pub fn pixels(&self) -> u16 {
        match self {
            IiifSize::Thumb => 400,
            IiifSize::Large => 1686,
        }
    }

    /// Select appropriate IIIF size for a given card size
    /// Uses thumb (400px) for cards up to 400px, large for bigger cards
    pub fn for_card_size(card_size: CardSize) -> Self {
        if card_size.pixels() > 400 {
            IiifSize::Large
        } else {
            IiifSize::Thumb
        }
    }
}

/// Generate IIIF URL from asset ID and size
///
/// Asset ID format: `{policy_id_hex}{asset_name_hex}` (total 56+ chars)
/// Policy ID is always 56 hex chars, asset name is the remainder.
///
/// Returns None if asset_id is too short to contain a valid policy ID.
pub fn generate_iiif_url(asset_id: &str, size: IiifSize) -> Option<String> {
    // Policy ID is 56 hex characters (28 bytes)
    if asset_id.len() < 56 {
        return None;
    }

    let policy_id = &asset_id[..56];
    let asset_name = &asset_id[56..];

    if asset_name.is_empty() {
        return None;
    }

    // IIIF format: {policy_id}:{asset_name_hex}
    Some(format!(
        "{IIIF_BASE_URL}/{policy_id}:{asset_name}/full/{},/0/default.jpg",
        size.pixels()
    ))
}

/// Asset card custom element - wraps image-card with IIIF URL generation
#[derive(Default)]
pub struct AssetCard {
    asset_id: String,
    image_url: Option<String>,
    size: CardSize,
    name: String,
    accent_color: Option<String>,
    is_static: bool,
    show_name: bool,
}

impl AssetCard {
    /// Register the custom element. Call once at app startup.
    pub fn define() {
        <Self as CustomElement>::define("asset-card");
    }

    /// Get the image URL - uses direct image-url if provided, otherwise generates IIIF URL
    fn resolved_image_url(&self) -> Option<String> {
        // Direct image URL takes precedence
        if let Some(url) = &self.image_url {
            if !url.is_empty() {
                return Some(url.clone());
            }
        }

        // Fall back to IIIF URL generation from asset-id
        if self.asset_id.is_empty() {
            return None;
        }
        let iiif_size = IiifSize::for_card_size(self.size);
        generate_iiif_url(&self.asset_id, iiif_size)
    }

    /// Render HTML string for the component
    fn render_html(&self) -> String {
        let image_url = self.resolved_image_url().unwrap_or_default();
        let mut attrs = vec![
            format!(r#"image-url="{}""#, html_escape(&image_url)),
            format!(r#"size="{}""#, self.size.class_suffix()),
        ];

        if !self.name.is_empty() {
            attrs.push(format!(r#"name="{}""#, html_escape(&self.name)));
        }

        if let Some(color) = &self.accent_color {
            attrs.push(format!(r#"accent-color="{}""#, html_escape(color)));
        }

        if self.is_static {
            attrs.push("static".to_string());
        }

        if self.show_name {
            attrs.push("show-name".to_string());
        }

        format!(
            r#"<style>{COMPONENT_STYLES}</style>
<image-card {}></image-card>"#,
            attrs.join(" ")
        )
    }

    /// Setup click handler to forward events with asset-id
    fn setup_click_handler(&self, element: &HtmlElement) {
        if self.is_static {
            return;
        }

        if let Some(shadow) = element.shadow_root() {
            if let Ok(Some(image_card)) = shadow.query_selector("image-card") {
                let host = element.clone();
                let asset_id = self.asset_id.clone();

                let closure =
                    wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                        dispatch_event_with_detail(&host, "card-click", &asset_id);
                    }) as Box<dyn Fn(_)>);

                image_card
                    .add_event_listener_with_callback(
                        "card-click",
                        closure.as_ref().unchecked_ref(),
                    )
                    .ok();
                closure.forget();
            }
        }
    }
}

impl CustomElement for AssetCard {
    fn observed_attributes() -> &'static [&'static str] {
        &[
            "asset-id",
            "image-url",
            "size",
            "name",
            "accent-color",
            "static",
            "show-name",
        ]
    }

    fn constructor(&mut self, this: &HtmlElement) {
        self.asset_id = this.get_attribute("asset-id").unwrap_or_default();
        self.image_url = this.get_attribute("image-url").filter(|s| !s.is_empty());
        self.size = this
            .get_attribute("size")
            .map(|s| parse_card_size(&s))
            .unwrap_or_default();
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
            "asset-id" => self.asset_id = new_value.unwrap_or_default(),
            "image-url" => self.image_url = new_value.filter(|s| !s.is_empty()),
            "size" => self.size = new_value.map(|s| parse_card_size(&s)).unwrap_or_default(),
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

/// Dispatch a custom event with a string detail
fn dispatch_event_with_detail(element: &HtmlElement, event_name: &str, detail: &str) {
    use wasm_bindgen::JsValue;
    use web_sys::CustomEventInit;

    let init = CustomEventInit::new();
    init.set_bubbles(true);
    init.set_composed(true);
    init.set_detail(&JsValue::from_str(detail));

    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict(event_name, &init) {
        let _ = element.dispatch_event(&event);
    }
}

/// Simple HTML escaping
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// Minimal styles - image-card handles the display
const COMPONENT_STYLES: &str = scss_inline!(
    r#"
    :host {
        display: block;
        width: 100%;
    }

    image-card {
        display: block;
        width: 100%;
    }
"#
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_iiif_url() {
        let asset_id = "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839";

        let thumb_url = generate_iiif_url(asset_id, IiifSize::Thumb).unwrap();
        assert!(thumb_url.contains("/400,/"));
        assert!(thumb_url.contains(
            "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6:506972617465313839"
        ));

        let large_url = generate_iiif_url(asset_id, IiifSize::Large).unwrap();
        assert!(large_url.contains("/1686,/"));
    }

    #[test]
    fn test_generate_iiif_url_short_id() {
        let short_id = "abc123";
        assert!(generate_iiif_url(short_id, IiifSize::Thumb).is_none());
    }

    #[test]
    fn test_generate_iiif_url_policy_only() {
        // 56 chars but no asset name
        let policy_only = "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6";
        assert!(generate_iiif_url(policy_only, IiifSize::Thumb).is_none());
    }

    #[test]
    fn test_iiif_size_for_card_size() {
        assert_eq!(IiifSize::for_card_size(CardSize::Xs), IiifSize::Thumb);
        assert_eq!(IiifSize::for_card_size(CardSize::Sm), IiifSize::Thumb);
        assert_eq!(IiifSize::for_card_size(CardSize::Md), IiifSize::Thumb);
        assert_eq!(IiifSize::for_card_size(CardSize::Lg), IiifSize::Thumb);
        assert_eq!(IiifSize::for_card_size(CardSize::Xl), IiifSize::Large);
    }
}
