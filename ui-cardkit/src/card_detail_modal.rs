//! CardDetailModal Component
//!
//! Full card detail view shown when clicking a deployed engine.
//! Wraps ui-components Modal with card-specific styling.
//!
//! ## Usage
//!
//! ```ignore
//! <CardDetailModal
//!     open=show_detail
//!     on_close=move |_| set_show_detail.set(false)
//! >
//!     <GameCard size=CardSize::Xl>
//!         // Full card content
//!     </GameCard>
//! </CardDetailModal>
//! ```

use leptos::prelude::*;
use ui_components::Modal;

/// Modal for showing full card details
#[component]
pub fn CardDetailModal(
    /// Modal open state
    #[prop(into)]
    open: Signal<bool>,
    /// Close callback
    #[prop(into)]
    on_close: Callback<()>,
    /// Card content
    children: Children,
) -> impl IntoView {
    view! {
        <Modal
            open=open
            on_close=on_close
            flush=true
        >
            <div class="cardkit-card-detail-modal">
                {children()}
            </div>
        </Modal>
    }
}
