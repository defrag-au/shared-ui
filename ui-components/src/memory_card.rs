//! Memory Card Web Component
//!
//! A flippable card for the memory matching game. Shows a card back when
//! face-down and wraps an `<asset-card>` when face-up. The asset-card handles
//! IIIF URL generation from the asset ID automatically.
//!
//! ## Attributes
//!
//! - `asset-id` - Cardano asset ID (policy_id + asset_name hex) for IIIF image
//! - `name` - Name shown in overlay when flipped
//! - `flipped` - Whether the card is face-up (present = face-up)
//! - `matched` - Whether the card has been matched (stays revealed, shows glow)
//! - `matched-by` - Name of player who matched this card
//! - `disabled` - Whether the card can be clicked
//!
//! ## Events
//!
//! - `card-click` - Dispatched when card is clicked (if not disabled)
//!
//! ## Usage
//!
//! ```html
//! <memory-card asset-id="b3dab69f...506972617465313839" name="Captain Jack"></memory-card>
//! <memory-card asset-id="b3dab69f..." flipped matched matched-by="Player1"></memory-card>
//! ```

use crate::render_to_shadow;
use custom_elements::CustomElement;
use primitives::{dispatch_event, on_click};
use scss_macros::scss_inline;
use web_sys::HtmlElement;

/// Memory card custom element - a flippable wrapper around asset-card
#[derive(Default)]
pub struct MemoryCard {
    asset_id: String,
    name: String,
    flipped: bool,
    matched: bool,
    matched_by: Option<String>,
    disabled: bool,
}

impl MemoryCard {
    /// Register the custom element. Call once at app startup.
    pub fn define() {
        <Self as CustomElement>::define("memory-card");
    }

    /// Render HTML string for the component
    fn render_html(&self) -> String {
        let card_classes = self.build_card_classes();
        let asset_card = self.build_asset_card();
        let matched_overlay = self.build_matched_overlay();

        format!(
            r#"<style>{COMPONENT_STYLES}</style>
<div class="{card_classes}">
    <div class="memory-card__inner">
        <div class="memory-card__front">
            <div class="memory-card__back-design">
                <div class="memory-card__skull"></div>
            </div>
        </div>
        <div class="memory-card__back">
            {asset_card}
            {matched_overlay}
        </div>
    </div>
</div>"#
        )
    }

    fn build_card_classes(&self) -> String {
        let mut classes = vec!["memory-card"];

        if self.flipped || self.matched {
            classes.push("memory-card--flipped");
        }
        if self.matched {
            classes.push("memory-card--matched");
        }
        if self.disabled {
            classes.push("memory-card--disabled");
        }

        classes.join(" ")
    }

    fn build_asset_card(&self) -> String {
        let mut attrs = vec![
            format!(r#"asset-id="{}""#, html_escape(&self.asset_id)),
            "size=\"md\"".to_string(), // Medium size for game cards
            "show-name".to_string(),
            "static".to_string(), // Memory card handles its own click events
        ];

        if !self.name.is_empty() {
            attrs.push(format!(r#"name="{}""#, html_escape(&self.name)));
        }

        format!("<asset-card {}></asset-card>", attrs.join(" "))
    }

    fn build_matched_overlay(&self) -> String {
        if let Some(by) = &self.matched_by {
            format!(
                r#"<div class="memory-card__matched-by">{}</div>"#,
                html_escape(by)
            )
        } else {
            String::new()
        }
    }

    /// Setup click handler after rendering
    fn setup_click_handler(&self, element: &HtmlElement) {
        tracing::debug!("memory-card: setup_click_handler called");

        let (shadow, host) = primitives::get_shadow_and_host(element);

        if let Ok(Some(card)) = shadow.query_selector(".memory-card") {
            tracing::debug!("memory-card: found .memory-card element, attaching click handler");
            on_click(&card, move |_| {
                tracing::debug!("memory-card: click detected!");
                // Check disabled/matched state at click time via attributes
                let is_disabled = host.has_attribute("disabled");
                let is_matched = host.has_attribute("matched");
                tracing::debug!(
                    is_disabled,
                    is_matched,
                    "memory-card: checking state at click time"
                );
                if !is_disabled && !is_matched {
                    tracing::debug!("memory-card: dispatching card-click event");
                    dispatch_event(&host, "card-click");
                }
            });
        } else {
            tracing::warn!("memory-card: .memory-card element NOT found in shadow DOM");
        }
    }
}

impl CustomElement for MemoryCard {
    fn observed_attributes() -> &'static [&'static str] {
        &[
            "asset-id",
            "name",
            "flipped",
            "matched",
            "matched-by",
            "disabled",
        ]
    }

    fn constructor(&mut self, this: &HtmlElement) {
        self.asset_id = this.get_attribute("asset-id").unwrap_or_default();
        self.name = this.get_attribute("name").unwrap_or_default();
        self.flipped = this.has_attribute("flipped");
        self.matched = this.has_attribute("matched");
        self.matched_by = this.get_attribute("matched-by");
        self.disabled = this.has_attribute("disabled");
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
            "name" => self.name = new_value.unwrap_or_default(),
            "flipped" => self.flipped = new_value.is_some(),
            "matched" => self.matched = new_value.is_some(),
            "matched-by" => self.matched_by = new_value,
            "disabled" => self.disabled = new_value.is_some(),
            _ => {}
        }

        render_to_shadow(this, &self.render_html());
        // Re-setup click handler since render_to_shadow replaces DOM content
        self.setup_click_handler(this);
    }

    fn inject_children(&mut self, this: &HtmlElement) {
        tracing::debug!("memory-card: inject_children called");
        render_to_shadow(this, &self.render_html());
        tracing::debug!("memory-card: after render_to_shadow, calling setup_click_handler");
        self.setup_click_handler(this);
        tracing::debug!("memory-card: inject_children complete");
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
        perspective: 1000px;
    }

    .memory-card {
        position: relative;
        width: 100%;
        aspect-ratio: 1;
        cursor: pointer;
        transform-style: preserve-3d;
        transition: transform 0.1s ease;

        &:hover:not(.memory-card--disabled):not(.memory-card--matched) {
            transform: scale(1.02);
        }

        &:active:not(.memory-card--disabled):not(.memory-card--matched) {
            transform: scale(0.98);
        }

        &--disabled {
            cursor: not-allowed;
            opacity: 0.7;
        }

        &--matched {
            cursor: default;
        }

        &--matched &__inner {
            box-shadow: 0 0 8px rgba(255, 215, 0, 0.3);
        }

        &__inner {
            position: relative;
            width: 100%;
            height: 100%;
            transform-style: preserve-3d;
            transition: transform 0.5s ease;
            border-radius: 8px;
        }

        &--flipped &__inner {
            transform: rotateY(180deg);
        }

        &__front,
        &__back {
            position: absolute;
            width: 100%;
            height: 100%;
            backface-visibility: hidden;
            border-radius: 8px;
            overflow: hidden;
        }

        &__front {
            background: #1a1a2e;
            border: 2px solid #2a2a4e;
        }

        &__back {
            background: #1a1a2e;
            border: 2px solid #2a2a4e;
            transform: rotateY(180deg);
            position: relative;
        }

        // Card back design - Black Flag themed
        &__back-design {
            width: 100%;
            height: 100%;
            background: linear-gradient(135deg, #0d0d1a 0%, #1a1a2e 50%, #0d0d1a 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            position: relative;

            // Border pattern
            &::before {
                content: "";
                position: absolute;
                inset: 8px;
                border: 2px solid rgba(255, 215, 0, 0.3);
                border-radius: 4px;
            }

            // Corner accents
            &::after {
                content: "";
                position: absolute;
                inset: 4px;
                background:
                    linear-gradient(135deg, rgba(255, 215, 0, 0.2) 0%, transparent 20%),
                    linear-gradient(225deg, rgba(255, 215, 0, 0.2) 0%, transparent 20%),
                    linear-gradient(315deg, rgba(255, 215, 0, 0.2) 0%, transparent 20%),
                    linear-gradient(45deg, rgba(255, 215, 0, 0.2) 0%, transparent 20%);
            }
        }

        // Skull placeholder for card back
        &__skull {
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

        // Nested asset-card styling
        &__back asset-card {
            display: block;
            width: 100%;
            height: 100%;
            max-width: none;
        }

        // Matched by overlay
        &__matched-by {
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
    }

    // Matched card glow animation - subtle pulse
    @keyframes matched-glow {
        0%, 100% {
            box-shadow: 0 0 6px rgba(255, 215, 0, 0.2);
        }
        50% {
            box-shadow: 0 0 10px rgba(255, 215, 0, 0.35);
        }
    }

    .memory-card--matched .memory-card__inner {
        animation: matched-glow 2s ease-in-out infinite;
    }
"#
);
