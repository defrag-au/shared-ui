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
//! ## Attributes
//!
//! - `asset-id` - Cardano asset ID (policy_id + asset_name hex)
//! - `size` - Image size: "thumb" (400px, default) or "large" (1686px)
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
//! <!-- Thumbnail (400px, default) -->
//! <asset-card
//!     asset-id="b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839"
//!     name="Pirate #189"
//!     show-name
//! ></asset-card>
//!
//! <!-- Large image (1686px) -->
//! <asset-card
//!     asset-id="b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839"
//!     size="large"
//!     name="Pirate #189"
//! ></asset-card>
//! ```

use crate::render_to_shadow;
use custom_elements::CustomElement;
use phf::phf_map;
use scss_macros::scss_inline;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// IIIF base URL for image lookups
const IIIF_BASE_URL: &str = "https://iiif.hodlcroft.com/iiif/3";

/// Image size variants for IIIF requests
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageSize {
    /// 400px width - fast, cached thumbnails
    #[default]
    Thumb,
    /// 1686px width - high resolution
    Large,
}

impl ImageSize {
    /// Get the IIIF size parameter value
    pub fn iiif_size(&self) -> u16 {
        match self {
            ImageSize::Thumb => 400,
            ImageSize::Large => 1686,
        }
    }
}

/// Map from attribute string to ImageSize variant
static SIZE_MAP: phf::Map<&'static str, ImageSize> = phf_map! {
    "thumb" => ImageSize::Thumb,
    "thumbnail" => ImageSize::Thumb,
    "sm" => ImageSize::Thumb,
    "small" => ImageSize::Thumb,
    "large" => ImageSize::Large,
    "lg" => ImageSize::Large,
    "full" => ImageSize::Large,
};

/// Parse size attribute string to ImageSize
fn parse_size(s: &str) -> ImageSize {
    SIZE_MAP
        .get(s.to_lowercase().as_str())
        .copied()
        .unwrap_or_default()
}

/// Generate IIIF URL from asset ID and size
///
/// Asset ID format: `{policy_id_hex}{asset_name_hex}` (total 56+ chars)
/// Policy ID is always 56 hex chars, asset name is the remainder.
///
/// Returns None if asset_id is too short to contain a valid policy ID.
pub fn generate_iiif_url(asset_id: &str, size: ImageSize) -> Option<String> {
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
        size.iiif_size()
    ))
}

/// Asset card custom element - wraps image-card with IIIF URL generation
#[derive(Default)]
pub struct AssetCard {
    asset_id: String,
    size: ImageSize,
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

    /// Get the IIIF image URL for this asset
    fn image_url(&self) -> Option<String> {
        if self.asset_id.is_empty() {
            return None;
        }
        generate_iiif_url(&self.asset_id, self.size)
    }

    /// Render HTML string for the component
    fn render_html(&self) -> String {
        let image_url = self.image_url().unwrap_or_default();
        let mut attrs = vec![format!(r#"image-url="{}""#, html_escape(&image_url))];

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
            "size",
            "name",
            "accent-color",
            "static",
            "show-name",
        ]
    }

    fn constructor(&mut self, this: &HtmlElement) {
        self.asset_id = this.get_attribute("asset-id").unwrap_or_default();
        self.size = this
            .get_attribute("size")
            .map(|s| parse_size(&s))
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
            "size" => self.size = new_value.map(|s| parse_size(&s)).unwrap_or_default(),
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
        max-width: 160px;
    }

    image-card {
        display: block;
        width: 100%;
        max-width: inherit;
    }
"#
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_iiif_url() {
        let asset_id = "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839";

        let thumb_url = generate_iiif_url(asset_id, ImageSize::Thumb).unwrap();
        assert!(thumb_url.contains("/400,/"));
        assert!(thumb_url.contains(
            "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6:506972617465313839"
        ));

        let large_url = generate_iiif_url(asset_id, ImageSize::Large).unwrap();
        assert!(large_url.contains("/1686,/"));
    }

    #[test]
    fn test_generate_iiif_url_short_id() {
        let short_id = "abc123";
        assert!(generate_iiif_url(short_id, ImageSize::Thumb).is_none());
    }

    #[test]
    fn test_generate_iiif_url_policy_only() {
        // 56 chars but no asset name
        let policy_only = "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6";
        assert!(generate_iiif_url(policy_only, ImageSize::Thumb).is_none());
    }

    #[test]
    fn test_parse_size() {
        assert_eq!(parse_size("thumb"), ImageSize::Thumb);
        assert_eq!(parse_size("THUMB"), ImageSize::Thumb);
        assert_eq!(parse_size("thumbnail"), ImageSize::Thumb);
        assert_eq!(parse_size("sm"), ImageSize::Thumb);
        assert_eq!(parse_size("large"), ImageSize::Large);
        assert_eq!(parse_size("lg"), ImageSize::Large);
        assert_eq!(parse_size("full"), ImageSize::Large);
        assert_eq!(parse_size("unknown"), ImageSize::Thumb); // default
    }
}
