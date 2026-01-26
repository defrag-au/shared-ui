//! HealthBar Component
//!
//! Visual health/damage display, typically for the monster.
//!
//! ## Usage
//!
//! ```ignore
//! <HealthBar
//!     current=monster_health
//!     max=monster_max_health
//!     show_value=true
//! />
//! ```

use leptos::prelude::*;

/// Health bar display variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HealthBarVariant {
    /// Standard green/red health bar
    #[default]
    Standard,
    /// Blue shield bar
    Shield,
    /// Yellow energy bar
    Energy,
}

impl HealthBarVariant {
    /// Get the CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            HealthBarVariant::Standard => "standard",
            HealthBarVariant::Shield => "shield",
            HealthBarVariant::Energy => "energy",
        }
    }
}

/// Health bar component
#[component]
pub fn HealthBar(
    /// Current value
    #[prop(into)]
    current: Signal<u32>,
    /// Maximum value
    #[prop(into)]
    max: Signal<u32>,
    /// Show numeric value
    #[prop(optional, default = true)]
    show_value: bool,
    /// Visual variant
    #[prop(optional, default = HealthBarVariant::Standard)]
    variant: HealthBarVariant,
) -> impl IntoView {
    let variant_class = format!("cardkit-health-bar--{}", variant.class_suffix());

    let percent = move || {
        let m = max.get();
        if m == 0 {
            0.0
        } else {
            (current.get() as f32 / m as f32).min(1.0)
        }
    };

    let is_low = move || percent() <= 0.25;

    let class_string = move || {
        let mut classes = vec!["cardkit-health-bar", &variant_class];
        if is_low() {
            classes.push("cardkit-health-bar--low");
        }
        classes.join(" ")
    };

    let fill_width = move || format!("{}%", percent() * 100.0);

    view! {
        <div class=class_string>
            <div class="cardkit-health-bar__track">
                <div
                    class="cardkit-health-bar__fill"
                    style:width=fill_width
                />
            </div>

            {show_value.then(|| view! {
                <div class="cardkit-health-bar__value">
                    {move || current.get()}
                    " / "
                    {move || max.get()}
                </div>
            })}
        </div>
    }
}
