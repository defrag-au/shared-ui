//! Memory Game Board Component
//!
//! Displays the grid of cards and handles card flip interactions.

use leptos::*;

/// Card view data for the frontend
#[derive(Debug, Clone)]
pub struct CardView {
    /// Card index
    pub index: usize,
    /// Whether the card face is visible
    pub visible: bool,
    /// Asset ID (policy_id + asset_name_hex) for IIIF URL generation
    pub asset_id: Option<String>,
    /// Asset name (if visible)
    pub name: Option<String>,
    /// Whether this card has been matched
    pub matched: bool,
    /// Who matched this card
    pub matched_by: Option<String>,
}

/// Memory game board displaying a grid of cards
#[component]
pub fn GameBoard(
    /// Grid dimensions (cols, rows)
    #[prop(into)]
    grid_size: Signal<(u8, u8)>,
    /// Card data
    #[prop(into)]
    cards: Signal<Vec<CardView>>,
    /// Currently flipped card indices
    #[prop(into)]
    flipped_indices: Signal<Vec<usize>>,
    /// Whether it's this player's turn
    #[prop(into)]
    is_my_turn: Signal<bool>,
    /// Callback when a card is clicked
    on_flip: impl Fn(usize) + 'static,
    /// Callback when a card's image has loaded
    on_card_loaded: impl Fn(usize) + 'static,
    /// Whether input is disabled
    #[prop(into)]
    disabled: Signal<bool>,
) -> impl IntoView {
    let on_flip = std::rc::Rc::new(on_flip);
    let on_card_loaded = std::rc::Rc::new(on_card_loaded);

    view! {
        <div class="game-board-container">
            <div
                class="game-board"
                style=move || {
                    let (cols, _rows) = grid_size.get();
                    format!(
                        "display: grid; grid-template-columns: repeat({}, 1fr); gap: 8px;",
                        cols
                    )
                }
            >
                {move || {
                    let cards_vec = cards.get();
                    let flipped = flipped_indices.get();
                    let my_turn = is_my_turn.get();
                    let is_disabled = disabled.get();

                    cards_vec.into_iter().enumerate().map(|(idx, card)| {
                        let on_flip = on_flip.clone();
                        let on_card_loaded = on_card_loaded.clone();
                        let is_flipped = flipped.contains(&idx) || card.visible;
                        let can_click = my_turn && !is_disabled && !card.matched && !is_flipped;

                        if idx == 0 {
                            tracing::debug!(
                                my_turn,
                                is_disabled,
                                card_matched = card.matched,
                                is_flipped,
                                can_click,
                                "GameBoard card[0] state"
                            );
                        }

                        // For boolean attributes, use Option<&str> - None removes the attribute
                        let flipped_attr = is_flipped.then_some("");
                        let matched_attr = card.matched.then_some("");
                        let disabled_attr = (!can_click).then_some("");

                        view! {
                            <memory-card
                                attr:asset-id=card.asset_id.clone().unwrap_or_default()
                                attr:name=card.name.clone().unwrap_or_default()
                                attr:flipped=flipped_attr
                                attr:matched=matched_attr
                                attr:matched-by=card.matched_by.clone().unwrap_or_default()
                                attr:disabled=disabled_attr
                                on:card-click=move |_: web_sys::CustomEvent| {
                                    if can_click {
                                        on_flip(idx);
                                    }
                                }
                                on:card-loaded=move |_: web_sys::CustomEvent| {
                                    // Notify server that card image has loaded
                                    on_card_loaded(idx);
                                }
                            />
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}
