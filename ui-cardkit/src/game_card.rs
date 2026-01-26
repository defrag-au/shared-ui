//! GameCard Component
//!
//! The base card display component for full-sized cards (hand, detail view).
//! Provides consistent framing with optional visual states.
//!
//! ## Usage
//!
//! ```ignore
//! <GameCard
//!     size=CardSize::Md
//!     highlighted=is_selected
//!     on_click=move |_| select_card()
//! >
//!     // Card face content (image, stats, etc.)
//!     <img src="card-art.png" />
//!     <div class="card-stats">...</div>
//! </GameCard>
//! ```

use leptos::prelude::*;

/// Card size variants for full-sized cards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardSize {
    /// 100px width - compact hand display
    Sm,
    /// 140px width - default hand size
    #[default]
    Md,
    /// 200px width - larger display
    Lg,
    /// 280px width - detail/focus view
    Xl,
}

impl CardSize {
    /// Get the CSS class suffix for this size
    pub fn class_suffix(&self) -> &'static str {
        match self {
            CardSize::Sm => "sm",
            CardSize::Md => "md",
            CardSize::Lg => "lg",
            CardSize::Xl => "xl",
        }
    }
}

/// Full-sized card component for hand display and detail views
#[component]
pub fn GameCard(
    /// Card size
    #[prop(optional, default = CardSize::Md)]
    size: CardSize,
    /// Highlighted state (selected, hoverable)
    #[prop(into, optional)]
    highlighted: Option<Signal<bool>>,
    /// Disabled state (greyed out, non-interactive)
    #[prop(into, optional)]
    disabled: Option<Signal<bool>>,
    /// Click callback
    #[prop(into, optional)]
    on_click: Option<Callback<()>>,
    /// Card content
    children: Children,
) -> impl IntoView {
    let size_class = format!("cardkit-game-card--{}", size.class_suffix());
    let is_clickable = on_click.is_some();

    let class_string = move || {
        let mut classes = vec!["cardkit-game-card", &size_class];

        if highlighted.map(|s| s.get()).unwrap_or(false) {
            classes.push("cardkit-game-card--highlighted");
        }
        if disabled.map(|s| s.get()).unwrap_or(false) {
            classes.push("cardkit-game-card--disabled");
        }
        if is_clickable {
            classes.push("cardkit-game-card--clickable");
        }

        classes.join(" ")
    };

    let is_disabled = move || disabled.map(|s| s.get()).unwrap_or(false);

    let handle_click = move |_| {
        if !is_disabled() {
            if let Some(cb) = on_click {
                cb.run(());
            }
        }
    };

    view! {
        <div
            class=class_string
            on:click=handle_click
        >
            <div class="cardkit-game-card__inner">
                {children()}
            </div>
        </div>
    }
}
