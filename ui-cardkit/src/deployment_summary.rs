//! DeploymentSummary Component
//!
//! Shows aggregated deployment counts and optional power totals.
//!
//! ## Usage
//!
//! ```ignore
//! <DeploymentSummary
//!     your_count=move || game.your_deployed.get().len()
//!     others_count=move || game.others_deployed.get().len()
//!     your_power=move || game.your_total_power()
//!     others_power=move || game.others_total_power()
//! />
//! ```

use leptos::prelude::*;

/// Aggregated deployment summary display
#[component]
pub fn DeploymentSummary(
    /// Your deployed count
    #[prop(into)]
    your_count: Signal<usize>,
    /// Others' deployed count
    #[prop(into)]
    others_count: Signal<usize>,
    /// Your total power (optional)
    #[prop(into, optional)]
    your_power: Option<Signal<u32>>,
    /// Others' total power (optional)
    #[prop(into, optional)]
    others_power: Option<Signal<u32>>,
) -> impl IntoView {
    let _show_power = your_power.is_some() || others_power.is_some();

    view! {
        <div class="cardkit-deployment-summary">
            // Your section
            <div class="cardkit-deployment-summary__section cardkit-deployment-summary__section--you">
                <span class="cardkit-deployment-summary__label">"Your Engines"</span>
                <span class="cardkit-deployment-summary__count">{move || your_count.get()}</span>
                {your_power.map(|p| view! {
                    <span class="cardkit-deployment-summary__power">
                        <span class="cardkit-deployment-summary__power-icon">"⚡"</span>
                        {move || p.get()}
                    </span>
                })}
            </div>

            // Divider
            <div class="cardkit-deployment-summary__divider">"|"</div>

            // Others section
            <div class="cardkit-deployment-summary__section cardkit-deployment-summary__section--others">
                <span class="cardkit-deployment-summary__label">"Fleet Engines"</span>
                <span class="cardkit-deployment-summary__count">{move || others_count.get()}</span>
                {others_power.map(|p| view! {
                    <span class="cardkit-deployment-summary__power">
                        <span class="cardkit-deployment-summary__power-icon">"⚡"</span>
                        {move || p.get()}
                    </span>
                })}
            </div>
        </div>
    }
}
