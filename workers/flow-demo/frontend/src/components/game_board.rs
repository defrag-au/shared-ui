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
    /// Asset image URL (if visible)
    pub image_url: Option<String>,
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
    /// Whether input is disabled
    #[prop(into)]
    disabled: Signal<bool>,
) -> impl IntoView {
    let on_flip = std::rc::Rc::new(on_flip);

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
                        let is_flipped = flipped.contains(&idx) || card.visible;
                        let can_click = my_turn && !is_disabled && !card.matched && !is_flipped;

                        view! {
                            <memory-card
                                attr:image-url=card.image_url.clone().unwrap_or_default()
                                attr:name=card.name.clone().unwrap_or_default()
                                attr:flipped=if is_flipped { "true" } else { "" }
                                attr:matched=if card.matched { "true" } else { "" }
                                attr:matched-by=card.matched_by.clone().unwrap_or_default()
                                attr:disabled=if !can_click { "true" } else { "" }
                                on:card-click=move |_: web_sys::CustomEvent| {
                                    if can_click {
                                        on_flip(idx);
                                    }
                                }
                            />
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}
