//! Memory Game Board Component
//!
//! Displays the grid of cards and handles card flip interactions.

use leptos::*;
use ui_components::MemoryCard;

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

    // Create derived signals for each card's state to enable CSS transitions
    // The key insight: we need stable DOM elements that update their classes,
    // not recreated elements, for CSS transitions to animate.

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
                <For
                    each=move || {
                        let cards_vec = cards.get();
                        cards_vec.into_iter().enumerate().collect::<Vec<_>>()
                    }
                    key=|(idx, _)| *idx
                    children=move |(idx, card)| {
                        let on_flip = on_flip.clone();
                        let on_card_loaded = on_card_loaded.clone();

                        // Create reactive derived signals for this card's state
                        let is_flipped = Signal::derive(move || {
                            let flipped = flipped_indices.get();
                            let cards_vec = cards.get();
                            let card_visible = cards_vec.get(idx).map(|c| c.visible).unwrap_or(false);
                            flipped.contains(&idx) || card_visible
                        });

                        let is_matched = Signal::derive(move || {
                            cards.get().get(idx).map(|c| c.matched).unwrap_or(false)
                        });

                        let matched_by_signal = Signal::derive(move || {
                            cards.get().get(idx).and_then(|c| c.matched_by.clone()).unwrap_or_default()
                        });

                        let can_click = Signal::derive(move || {
                            let my_turn = is_my_turn.get();
                            let is_disabled_val = disabled.get();
                            let matched = is_matched.get();
                            let flipped = is_flipped.get();
                            my_turn && !is_disabled_val && !matched && !flipped
                        });

                        view! {
                            <MemoryCard
                                asset_id=card.asset_id.clone().unwrap_or_default()
                                name=card.name.clone().unwrap_or_default()
                                flipped=is_flipped
                                matched=is_matched
                                matched_by=matched_by_signal
                                disabled=Signal::derive(move || !can_click.get())
                                on_click=move |()| {
                                    if can_click.get_untracked() {
                                        on_flip(idx);
                                    }
                                }
                                on_load=move |()| {
                                    on_card_loaded(idx);
                                }
                            />
                        }
                    }
                />
            </div>
        </div>
    }
}
