//! CardHand Component
//!
//! Displays the player's hand of cards in a horizontal layout.
//! Cards can be selected via click/tap (no drag mechanics).
//!
//! ## Usage
//!
//! ```ignore
//! <CardHand
//!     cards=game.hand
//!     key_fn=|c| c.id.clone()
//!     render_card=|card, state| view! {
//!         <GameCard
//!             highlighted=state.is_selected
//!             on_click=move |_| select_card(card.id)
//!         >
//!             // card content
//!         </GameCard>
//!     }
//!     on_select=move |card_id| game.deploy(card_id)
//!     max_size=5
//! />
//! ```

use leptos::prelude::*;
use std::hash::Hash;

/// State passed to the card render function
#[derive(Clone)]
pub struct HandCardState {
    /// Index in hand (0-based)
    pub index: usize,
    /// Whether this card is currently selected
    pub is_selected: Signal<bool>,
}

/// Player's hand of cards
#[component]
pub fn CardHand<C, K, KF, RF, V>(
    /// Cards in hand
    #[prop(into)]
    cards: Signal<Vec<C>>,
    /// Key function for each card
    key_fn: KF,
    /// Render function for each card
    render_card: RF,
    /// Callback when a card is selected
    #[prop(into, optional)]
    on_select: Option<Callback<K>>,
    /// Maximum hand size (shows overflow warning when exceeded)
    #[prop(into, optional)]
    max_size: Option<usize>,
    /// Currently selected card key
    #[prop(into, optional)]
    selected: Option<Signal<Option<K>>>,
) -> impl IntoView
where
    C: Clone + Send + Sync + 'static,
    K: Eq + Hash + Clone + Send + Sync + 'static,
    KF: Fn(&C) -> K + Clone + Send + Sync + 'static,
    RF: Fn(C, HandCardState) -> V + Clone + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let is_overflow = move || max_size.map(|max| cards.get().len() > max).unwrap_or(false);

    let class_string = move || {
        let mut classes = vec!["cardkit-card-hand"];
        if is_overflow() {
            classes.push("cardkit-card-hand--overflow");
        }
        classes.join(" ")
    };

    // Clone for use in nested closures
    let key_fn_for_render = key_fn.clone();
    let selected_for_render = selected;

    view! {
        <div class=class_string>
            <div class="cardkit-card-hand__cards">
                <For
                    each=move || cards.get().into_iter().enumerate()
                    key=move |(_, card)| key_fn.clone()(card)
                    children=move |(index, card)| {
                        let card_key = key_fn_for_render.clone()(&card);
                        let card_key_for_select = card_key.clone();
                        let card_key_for_signal = card_key.clone();

                        // Create a signal indicating if this card is selected
                        let is_selected = Signal::derive(move || {
                            selected_for_render
                                .map(|s| s.get().as_ref() == Some(&card_key_for_signal))
                                .unwrap_or(false)
                        });

                        let state = HandCardState { index, is_selected };

                        let handle_click = move |_: web_sys::MouseEvent| {
                            if let Some(cb) = on_select {
                                cb.run(card_key_for_select.clone());
                            }
                        };

                        view! {
                            <div
                                class="cardkit-card-hand__card"
                                on:click=handle_click
                            >
                                {render_card.clone()(card, state)}
                            </div>
                        }
                    }
                />
            </div>

            // Overflow warning
            <Show when=is_overflow>
                <div class="cardkit-card-hand__overflow-warning">
                    "Hand limit exceeded!"
                </div>
            </Show>
        </div>
    }
}
