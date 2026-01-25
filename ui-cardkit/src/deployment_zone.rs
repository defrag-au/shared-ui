//! DeploymentZone Component
//!
//! Container for deployed cards positioned over the monster stage.
//! Supports different layout styles for organizing deployed engines.
//!
//! ## Usage
//!
//! ```ignore
//! <DeploymentZone
//!     cards=your_deployed
//!     key_fn=|e| e.id.clone()
//!     render_card=|engine| view! {
//!         <CompactCard
//!             asset_id=engine.asset_id
//!             owner=Owner::You
//!             on_click=move |_| show_detail(engine.id)
//!         />
//!     }
//!     layout=DeploymentLayout::Scatter
//! />
//! ```

use leptos::prelude::*;
use std::hash::Hash;

/// Layout style for deployed cards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeploymentLayout {
    /// Random-ish scattered positions
    #[default]
    Scatter,
    /// Organized grid
    Grid,
    /// Horizontal row
    Row,
}

impl DeploymentLayout {
    /// Get the CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            DeploymentLayout::Scatter => "scatter",
            DeploymentLayout::Grid => "grid",
            DeploymentLayout::Row => "row",
        }
    }
}

/// Container for deployed cards
#[component]
pub fn DeploymentZone<C, K, KF, RF, V>(
    /// Deployed cards
    #[prop(into)]
    cards: Signal<Vec<C>>,
    /// Key function for each card
    key_fn: KF,
    /// Render function for each card
    render_card: RF,
    /// Layout style
    #[prop(optional, default = DeploymentLayout::Scatter)]
    layout: DeploymentLayout,
    /// Optional label (e.g., "Your Engines")
    #[prop(into, optional)]
    label: Option<String>,
) -> impl IntoView
where
    C: Clone + Send + Sync + 'static,
    K: Eq + Hash + Clone + Send + Sync + 'static,
    KF: Fn(&C) -> K + Clone + Send + Sync + 'static,
    RF: Fn(C) -> V + Clone + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let layout_class = format!("cardkit-deployment-zone--{}", layout.class_suffix());

    let class_string = format!("cardkit-deployment-zone {layout_class}");

    view! {
        <div class=class_string>
            {label.map(|l| view! {
                <div class="cardkit-deployment-zone__label">{l}</div>
            })}

            <div class="cardkit-deployment-zone__cards">
                <For
                    each=move || cards.get()
                    key=key_fn.clone()
                    let:card
                >
                    <div class="cardkit-deployment-zone__card">
                        {render_card.clone()(card)}
                    </div>
                </For>
            </div>
        </div>
    }
}
