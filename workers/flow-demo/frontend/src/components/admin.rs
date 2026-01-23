//! Admin Panel Component
//!
//! Provides admin controls for managing the game state.

use leptos::*;

/// Admin panel component
#[component]
pub fn AdminPanel(
    /// Callback to reset the game
    on_reset: impl Fn() + 'static,
) -> impl IntoView {
    let on_reset = std::rc::Rc::new(on_reset);
    let (confirming, set_confirming) = create_signal(false);

    view! {
        <div class="admin-panel">
            <h3>"Admin"</h3>

            {move || {
                if confirming.get() {
                    let on_reset = on_reset.clone();
                    view! {
                        <div class="admin-confirm">
                            <p>"Reset all game state?"</p>
                            <button class="btn btn-danger" on:click=move |_| {
                                on_reset();
                                set_confirming.set(false);
                            }>
                                "Yes, Reset"
                            </button>
                            <button class="btn btn-secondary" on:click=move |_| set_confirming.set(false)>
                                "Cancel"
                            </button>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <button class="btn btn-warning" on:click=move |_| set_confirming.set(true)>
                            "Reset Game"
                        </button>
                    }.into_view()
                }
            }}
        </div>
    }
}
