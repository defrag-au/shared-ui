//! Connection Status Web Component
//!
//! A status indicator showing WebSocket/realtime connection state.
//! Supports click-to-reconnect behavior and customizable appearance.
//!
//! ## Attributes
//!
//! - `status` - Connection state: "connected", "connecting", "disconnected", "error"
//! - `show-text` - Whether to show status text (default: true)
//! - `clickable` - Whether clicking triggers reconnect event (default: true when disconnected)
//!
//! ## Events
//!
//! - `reconnect` - Dispatched when user clicks to reconnect (only when disconnected/error)
//!
//! ## Usage
//!
//! ```html
//! <connection-status status="connected"></connection-status>
//! <connection-status status="disconnected" on:reconnect="handleReconnect"></connection-status>
//! ```

use crate::render_to_shadow;
use custom_elements::CustomElement;
use primitives::{dispatch_event, on_click};
use scss_macros::scss_inline;
use web_sys::HtmlElement;

/// Connection state values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionState {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Error,
}

impl ConnectionState {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "connected" => Self::Connected,
            "connecting" => Self::Connecting,
            "error" => Self::Error,
            _ => Self::Disconnected,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::Connecting => "connecting",
            Self::Disconnected => "disconnected",
            Self::Error => "error",
        }
    }

    pub fn display_text(&self) -> &'static str {
        match self {
            Self::Connected => "Connected",
            Self::Connecting => "Connecting...",
            Self::Disconnected => "Disconnected",
            Self::Error => "Connection Error",
        }
    }

    pub fn is_reconnectable(&self) -> bool {
        matches!(self, Self::Disconnected | Self::Error)
    }
}

/// Connection status custom element
#[derive(Default)]
pub struct ConnectionStatus {
    status: ConnectionState,
    show_text: bool,
    clickable: Option<bool>,
}

impl ConnectionStatus {
    /// Register the custom element. Call once at app startup.
    pub fn define() {
        <Self as CustomElement>::define("connection-status");
    }

    fn is_clickable(&self) -> bool {
        self.clickable
            .unwrap_or_else(|| self.status.is_reconnectable())
    }

    /// Render HTML string for the component
    fn render_html(&self) -> String {
        let status_str = self.status.as_str();
        let is_clickable = self.is_clickable();

        let clickable_class = if is_clickable {
            " connection-status--clickable"
        } else {
            ""
        };

        let text_clickable_class = if is_clickable {
            " connection-status__text--clickable"
        } else {
            ""
        };

        let text_html = if self.show_text {
            format!(
                r#"<span class="connection-status__text{text_clickable_class}">{}</span>"#,
                self.status.display_text()
            )
        } else {
            String::new()
        };

        format!(
            r#"<style>{COMPONENT_STYLES}</style>
<div class="connection-status connection-status--{status_str}{clickable_class}" data-status="{status_str}">
    <span class="connection-status__indicator"></span>
    {text_html}
</div>"#
        )
    }

    /// Setup click handler after rendering
    fn setup_click_handler(&self, element: &HtmlElement) {
        if !self.is_clickable() {
            return;
        }

        let (shadow, host) = primitives::get_shadow_and_host(element);
        if let Ok(Some(container)) = shadow.query_selector(".connection-status") {
            on_click(&container, move |_| {
                dispatch_event(&host, "reconnect");
            });
        }
    }
}

impl CustomElement for ConnectionStatus {
    fn observed_attributes() -> &'static [&'static str] {
        &["status", "show-text", "clickable"]
    }

    fn constructor(&mut self, this: &HtmlElement) {
        self.status = this
            .get_attribute("status")
            .map(|s| ConnectionState::from_str(&s))
            .unwrap_or_default();

        self.show_text = this
            .get_attribute("show-text")
            .map(|s| s != "false")
            .unwrap_or(true);

        self.clickable = this
            .get_attribute("clickable")
            .map(|s| s == "true" || s.is_empty());
    }

    fn attribute_changed_callback(
        &mut self,
        this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        match name.as_str() {
            "status" => {
                self.status = new_value
                    .map(|s| ConnectionState::from_str(&s))
                    .unwrap_or_default();
            }
            "show-text" => {
                self.show_text = new_value.map(|s| s != "false").unwrap_or(true);
            }
            "clickable" => {
                self.clickable = new_value.map(|s| s == "true" || s.is_empty());
            }
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

const COMPONENT_STYLES: &str = scss_inline!(
    r#"
    :host {
        display: inline-flex;
    }

    .connection-status {
        display: inline-flex;
        align-items: center;
        gap: 0.5em;
        padding: 0.25em 0.6em;
        border-radius: 9999px;
        font-size: 0.8125rem;
        font-weight: 500;
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        line-height: 1.2;
        white-space: nowrap;
        transition: background-color 0.15s ease, transform 0.1s ease;
        user-select: none;

        &--clickable {
            cursor: pointer;

            &:hover {
                filter: brightness(1.1);
            }

            &:active {
                transform: scale(0.97);
            }
        }

        &__indicator {
            width: 0.5em;
            height: 0.5em;
            border-radius: 50%;
            flex-shrink: 0;
        }

        &__text--clickable::after {
            content: " (click to reconnect)";
            opacity: 0;
            font-size: 0.85em;
            transition: opacity 0.15s ease;
        }

        &--clickable:hover &__text--clickable::after {
            opacity: 0.7;
        }

        // Connected state
        &--connected {
            background: rgba(25, 135, 84, 0.15);
            color: #198754;

            .connection-status__indicator {
                background: #198754;
                box-shadow: 0 0 4px #198754;
            }
        }

        // Connecting state
        &--connecting {
            background: rgba(255, 193, 7, 0.15);
            color: #997404;

            .connection-status__indicator {
                background: #ffc107;
                animation: pulse 1.5s ease-in-out infinite;
            }
        }

        // Disconnected state
        &--disconnected {
            background: rgba(108, 117, 125, 0.15);
            color: #6c757d;

            .connection-status__indicator {
                background: #6c757d;
            }
        }

        // Error state
        &--error {
            background: rgba(220, 53, 69, 0.15);
            color: #dc3545;

            .connection-status__indicator {
                background: #dc3545;
            }
        }
    }

    @keyframes pulse {
        0%, 100% {
            opacity: 1;
            transform: scale(1);
        }
        50% {
            opacity: 0.5;
            transform: scale(0.85);
        }
    }

    // Dark mode
    @media (prefers-color-scheme: dark) {
        .connection-status {
            &--connected {
                background: rgba(25, 135, 84, 0.25);
                color: #75b798;
            }

            &--connecting {
                background: rgba(255, 193, 7, 0.2);
                color: #ffda6a;
            }

            &--disconnected {
                background: rgba(108, 117, 125, 0.25);
                color: #adb5bd;
            }

            &--error {
                background: rgba(220, 53, 69, 0.25);
                color: #ea868f;
            }
        }
    }
"#
);
