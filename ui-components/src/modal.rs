//! Modal Leptos Component
//!
//! A modal dialog with backdrop using fixed positioning.
//!
//! ## Props
//!
//! - `open` - Signal controlling modal visibility
//! - `title` - Optional modal title
//! - `on_close` - Callback when modal should close (backdrop click or Escape key)
//! - `children` - Modal body content
//!
//! ## Usage
//!
//! ```ignore
//! let (show_modal, set_show_modal) = signal(false);
//!
//! <Button on_click=move |_| set_show_modal.set(true)>"Open Modal"</Button>
//!
//! <Modal
//!     open=show_modal
//!     title="Confirm Action".to_string()
//!     on_close=Callback::new(move |_| set_show_modal.set(false))
//! >
//!     <p>"Are you sure you want to proceed?"</p>
//!     <Button on_click=move |_| set_show_modal.set(false)>"Close"</Button>
//! </Modal>
//! ```

use leptos::prelude::*;

/// Modal dialog component
#[component]
pub fn Modal(
    /// Signal controlling modal visibility
    #[prop(into)]
    open: Signal<bool>,
    /// Optional modal title
    #[prop(into, optional)]
    title: Option<String>,
    /// Callback when modal should close
    #[prop(into, optional)]
    on_close: Option<Callback<()>>,
    /// Modal body content
    children: Children,
) -> impl IntoView {
    // Eagerly render all content
    let body_content = children();

    // Pre-render header content
    let header_content = title.map(|t| {
        view! {
            <div class="ui-modal__header">
                <h2 class="ui-modal__title">{t}</h2>
                {on_close.map(|cb| view! {
                    <button
                        class="ui-modal__close"
                        on:click=move |_| cb.run(())
                        aria-label="Close"
                    >
                        "\u{00D7}"
                    </button>
                })}
            </div>
        }
    });

    // Close on Escape key
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Escape" {
            if let Some(cb) = on_close {
                cb.run(());
            }
        }
    };

    // Close on backdrop click
    let handle_backdrop_click = move |_| {
        if let Some(cb) = on_close {
            cb.run(());
        }
    };

    // Prevent clicks inside modal from closing it
    let stop_propagation = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
    };

    // Visibility controlled via CSS display property
    let backdrop_style = move || {
        if open.get() {
            "display: flex;"
        } else {
            "display: none;"
        }
    };

    view! {
        <div
            class="ui-modal-backdrop"
            style=backdrop_style
            on:click=handle_backdrop_click
            on:keydown=handle_keydown
            tabindex="-1"
        >
            <div
                class="ui-modal"
                on:click=stop_propagation
                role="dialog"
                aria-modal="true"
                aria-hidden=move || (!open.get()).to_string()
            >
                {header_content}
                <div class="ui-modal__body">
                    {body_content}
                </div>
            </div>
        </div>
    }
}
