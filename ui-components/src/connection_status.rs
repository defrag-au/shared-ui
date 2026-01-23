//! Connection Status Leptos Component
//!
//! A status indicator showing WebSocket/realtime connection state.
//! Supports click-to-reconnect behavior.
//!
//! ## Props
//!
//! - `status` - Connection state: Connected, Connecting, Disconnected, Error
//! - `show_text` - Whether to show status text (default: true)
//! - `on_reconnect` - Callback when user clicks to reconnect (only when disconnected/error)
//!
//! ## Usage
//!
//! ```ignore
//! <ConnectionStatus
//!     status=connection_status
//!     on_reconnect=move |_| { reconnect(); }
//! />
//! ```

use leptos::*;

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

/// Connection status component
#[component]
pub fn ConnectionStatus(
    /// Connection state
    #[prop(into)]
    status: MaybeSignal<ConnectionState>,
    /// Whether to show status text
    #[prop(optional, default = true)]
    show_text: bool,
    /// Reconnect callback (called when clicking in disconnected/error state)
    #[prop(into, optional)]
    on_reconnect: Option<Callback<()>>,
) -> impl IntoView {
    let status_class = move || {
        let s = status.get();
        let clickable = s.is_reconnectable() && on_reconnect.is_some();
        let mut classes = vec![
            "connection-status",
            match s {
                ConnectionState::Connected => "connection-status--connected",
                ConnectionState::Connecting => "connection-status--connecting",
                ConnectionState::Disconnected => "connection-status--disconnected",
                ConnectionState::Error => "connection-status--error",
            },
        ];
        if clickable {
            classes.push("connection-status--clickable");
        }
        classes.join(" ")
    };

    let handle_click = move |_| {
        if status.get().is_reconnectable() {
            if let Some(cb) = on_reconnect {
                cb.call(());
            }
        }
    };

    view! {
        <style>{COMPONENT_STYLES}</style>
        <div class=status_class on:click=handle_click>
            <span class="connection-status__indicator"></span>
            {move || show_text.then(|| {
                let s = status.get();
                let clickable = s.is_reconnectable() && on_reconnect.is_some();
                let text_class = if clickable {
                    "connection-status__text connection-status__text--clickable"
                } else {
                    "connection-status__text"
                };
                view! {
                    <span class=text_class>{s.display_text()}</span>
                }
            })}
        </div>
    }
}

const COMPONENT_STYLES: &str = r##"
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
}

.connection-status--clickable {
    cursor: pointer;
}

.connection-status--clickable:hover {
    filter: brightness(1.1);
}

.connection-status--clickable:active {
    transform: scale(0.97);
}

.connection-status__indicator {
    width: 0.5em;
    height: 0.5em;
    border-radius: 50%;
    flex-shrink: 0;
}

.connection-status__text--clickable::after {
    content: " (click to reconnect)";
    opacity: 0;
    font-size: 0.85em;
    transition: opacity 0.15s ease;
}

.connection-status--clickable:hover .connection-status__text--clickable::after {
    opacity: 0.7;
}

/* Connected state */
.connection-status--connected {
    background: rgba(25, 135, 84, 0.15);
    color: #198754;
}

.connection-status--connected .connection-status__indicator {
    background: #198754;
    box-shadow: 0 0 4px #198754;
}

/* Connecting state */
.connection-status--connecting {
    background: rgba(255, 193, 7, 0.15);
    color: #997404;
}

.connection-status--connecting .connection-status__indicator {
    background: #ffc107;
    animation: pulse 1.5s ease-in-out infinite;
}

/* Disconnected state */
.connection-status--disconnected {
    background: rgba(108, 117, 125, 0.15);
    color: #6c757d;
}

.connection-status--disconnected .connection-status__indicator {
    background: #6c757d;
}

/* Error state */
.connection-status--error {
    background: rgba(220, 53, 69, 0.15);
    color: #dc3545;
}

.connection-status--error .connection-status__indicator {
    background: #dc3545;
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

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
    .connection-status--connected {
        background: rgba(25, 135, 84, 0.25);
        color: #75b798;
    }

    .connection-status--connecting {
        background: rgba(255, 193, 7, 0.2);
        color: #ffda6a;
    }

    .connection-status--disconnected {
        background: rgba(108, 117, 125, 0.25);
        color: #adb5bd;
    }

    .connection-status--error {
        background: rgba(220, 53, 69, 0.25);
        color: #ea868f;
    }
}
"##;
