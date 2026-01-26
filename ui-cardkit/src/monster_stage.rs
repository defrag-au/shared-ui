//! MonsterStage Component
//!
//! A themed game background showing the target monster. Displays health-based
//! damage effects but doesn't manage placement or deployment UI - that's
//! handled by the consumer.
//!
//! ## Usage
//!
//! ```ignore
//! <MonsterStage
//!     monster_image="https://example.com/leviathan.png"
//!     monster_name="Leviathan"
//!     health_percent=move || current_health as f32 / max_health as f32
//! />
//! ```

use leptos::prelude::*;

/// Monster stage - themed game background with health-based damage state
#[component]
pub fn MonsterStage(
    /// Monster image URL
    #[prop(into)]
    monster_image: Signal<String>,
    /// Monster name (for accessibility)
    #[prop(into)]
    monster_name: Signal<String>,
    /// Health percentage (0.0 - 1.0) for visual damage state
    #[prop(into, optional)]
    health_percent: Option<Signal<f32>>,
) -> impl IntoView {
    // Calculate damage visual state based on health
    let damage_class = move || {
        health_percent
            .map(|hp| {
                let h = hp.get();
                if h <= 0.25 {
                    "cardkit-monster-stage--critical"
                } else if h <= 0.5 {
                    "cardkit-monster-stage--damaged"
                } else if h <= 0.75 {
                    "cardkit-monster-stage--wounded"
                } else {
                    ""
                }
            })
            .unwrap_or("")
    };

    let class_string = move || {
        let base = "cardkit-monster-stage";
        let damage = damage_class();
        if damage.is_empty() {
            base.to_string()
        } else {
            format!("{base} {damage}")
        }
    };

    view! {
        <div class=class_string>
            <img
                class="cardkit-monster-stage__image"
                src=move || monster_image.get()
                alt=move || monster_name.get()
            />

            // Damage overlay effect
            {move || health_percent.map(|hp| {
                let h = hp.get();
                if h < 1.0 {
                    Some(view! {
                        <div
                            class="cardkit-monster-stage__damage-overlay"
                            style=format!("opacity: {}", (1.0 - h) * 0.5)
                        />
                    })
                } else {
                    None
                }
            })}
        </div>
    }
}
