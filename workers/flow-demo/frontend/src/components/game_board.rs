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

    // Get the card count once at setup - this determines the grid size
    // Cards are created once and their state is updated reactively
    let card_count = create_memo(move |_| cards.get().len());

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
                // Use For with a stable index range that only changes when card count changes
                // This ensures DOM elements persist and CSS transitions can animate
                <For
                    each=move || 0..card_count.get()
                    key=|idx| *idx
                    children=move |idx| {
                        let on_flip = on_flip.clone();
                        let on_card_loaded = on_card_loaded.clone();

                        // All card state is derived reactively from the cards signal
                        let asset_id = Signal::derive(move || {
                            cards.get().get(idx).and_then(|c| c.asset_id.clone()).unwrap_or_default()
                        });

                        let name = Signal::derive(move || {
                            cards.get().get(idx).and_then(|c| c.name.clone()).unwrap_or_default()
                        });

                        let is_flipped = Signal::derive(move || {
                            let flipped = flipped_indices.get();
                            let card_visible = cards.get().get(idx).map(|c| c.visible).unwrap_or(false);
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
                                asset_id=asset_id
                                name=name
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
