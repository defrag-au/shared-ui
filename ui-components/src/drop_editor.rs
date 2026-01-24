//! Drop Editor Component
//!
//! A compact horizontal editor for configuring reward drops (tips and wallet sends).
//! Used for raffle prizes, achievement rewards, giveaways, etc.
//!
//! ## Features
//!
//! - Horizontal stack of AssetCards showing prizes
//! - Drag-and-drop reordering
//! - Modal for adding new drops (tips or CNFTs)
//! - Remove button on hover
//! - Skeleton loading for NFT images
//!
//! # Example
//!
//! ```ignore
//! use ui_components::DropEditor;
//! use asset_intents::Drop;
//!
//! let (drops, set_drops) = signal(vec![
//!     Drop::tip("ADA", 100.0),
//!     Drop::wallet_send_single(asset_id),
//! ]);
//!
//! view! {
//!     <DropEditor
//!         drops=drops
//!         on_change=move |new_drops| set_drops.set(new_drops)
//!     />
//! }
//! ```

use crate::{AssetCard, Button, ButtonVariant, CardSize, Modal, Select, SelectOption};
use asset_intents::{AssetId, Drop};
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Editor for a list of reward drops - horizontal compact layout
#[component]
pub fn DropEditor(
    /// The current list of drops
    #[prop(into)]
    drops: Signal<Vec<Drop>>,
    /// Called when drops are modified
    #[prop(into)]
    on_change: Callback<Vec<Drop>>,
    /// If true, disables editing (no add/remove/reorder)
    #[prop(into, optional)]
    readonly: Signal<bool>,
    /// Card size for drop items
    #[prop(optional, default = CardSize::Sm)]
    size: CardSize,
) -> impl IntoView {
    // Modal state
    let (show_add_modal, set_show_add_modal) = signal(false);

    // Drag state
    let (drag_index, set_drag_index) = signal(Option::<usize>::None);
    let (drag_over_index, set_drag_over_index) = signal(Option::<usize>::None);

    // Helper to update drops
    let update_drops = move |new_drops: Vec<Drop>| {
        on_change.run(new_drops);
    };

    // Remove a drop at index
    let remove_drop = move |idx: usize| {
        let mut new_drops = drops.get();
        if idx < new_drops.len() {
            new_drops.remove(idx);
            update_drops(new_drops);
        }
    };

    // Handle drop reorder from drag
    let handle_drop = move |target_idx: usize| {
        if let Some(source_idx) = drag_index.get() {
            if source_idx != target_idx {
                let mut new_drops = drops.get();
                let item = new_drops.remove(source_idx);
                // Adjust target index if we removed before it
                let adjusted_target = if source_idx < target_idx {
                    target_idx - 1
                } else {
                    target_idx
                };
                new_drops.insert(adjusted_target, item);
                update_drops(new_drops);
            }
        }
        set_drag_index.set(None);
        set_drag_over_index.set(None);
    };

    // Add a new drop from modal
    let add_drop = move |drop: Drop| {
        let mut new_drops = drops.get();
        new_drops.push(drop);
        update_drops(new_drops);
        set_show_add_modal.set(false);
    };

    view! {
        <div class="drop-editor">
            // Horizontal stack of drop items
            <div class="drop-editor__items">
                <Show
                    when=move || !drops.get().is_empty()
                    fallback=move || {
                        view! {
                            <Show when=move || !readonly.get()>
                                <div class="drop-editor__empty">
                                    "No prizes yet"
                                </div>
                            </Show>
                        }
                    }
                >
                    <For
                        each={move || drops.get().into_iter().enumerate().collect::<Vec<_>>()}
                        key=|(idx, drop)| format!("{}-{}", idx, drop_key(drop))
                        let:item
                    >
                        {
                            let (idx, drop) = item;
                            let is_dragging = move || drag_index.get() == Some(idx);
                            let is_drag_over = move || drag_over_index.get() == Some(idx);

                            view! {
                                <DropItem
                                    index=idx
                                    drop=drop
                                    size=size
                                    readonly=readonly
                                    is_dragging=is_dragging
                                    is_drag_over=is_drag_over
                                    on_remove=move || remove_drop(idx)
                                    on_drag_start=move || set_drag_index.set(Some(idx))
                                    on_drag_end=move || {
                                        set_drag_index.set(None);
                                        set_drag_over_index.set(None);
                                    }
                                    on_drag_over=move || set_drag_over_index.set(Some(idx))
                                    on_drop=move || handle_drop(idx)
                                />
                            }
                        }
                    </For>
                </Show>

                // Add button (hidden in readonly mode)
                <Show when=move || !readonly.get()>
                    <button
                        class="drop-editor__add-btn"
                        on:click=move |_| set_show_add_modal.set(true)
                        title="Add prize"
                    >
                        <span class="drop-editor__add-icon">"+"</span>
                    </button>
                </Show>
            </div>

            // Add drop modal
            <AddDropModal
                open=show_add_modal
                on_close=move || set_show_add_modal.set(false)
                on_add=add_drop
            />
        </div>
    }
}

/// Generate a unique key for a drop (for list rendering)
fn drop_key(drop: &Drop) -> String {
    match drop {
        Drop::Tip { token, amount } => format!("tip-{token}-{amount}"),
        Drop::WalletSend { asset_id, amount } => {
            format!("ws-{}-{amount}", asset_id.concatenated())
        }
    }
}

/// Format drop display name for AssetCard
fn drop_display_name(drop: &Drop) -> String {
    match drop {
        Drop::Tip { token, amount } => {
            if *amount == amount.floor() {
                format!("{} {}", *amount as i64, token)
            } else {
                format!("{} {}", amount, token)
            }
        }
        Drop::WalletSend { asset_id, amount } => {
            let name = asset_id.asset_name();
            if *amount > 1 {
                format!("{} x{}", name, amount)
            } else {
                name
            }
        }
    }
}

/// A single drop item in the horizontal list
#[component]
fn DropItem(
    index: usize,
    drop: Drop,
    size: CardSize,
    #[prop(into)] readonly: Signal<bool>,
    #[prop(into)] is_dragging: Signal<bool>,
    #[prop(into)] is_drag_over: Signal<bool>,
    on_remove: impl Fn() + Send + Sync + 'static + Copy,
    on_drag_start: impl Fn() + Send + Sync + 'static + Copy,
    on_drag_end: impl Fn() + Send + Sync + 'static + Copy,
    on_drag_over: impl Fn() + Send + Sync + 'static + Copy,
    on_drop: impl Fn() + Send + Sync + 'static + Copy,
) -> impl IntoView {
    let display_name = drop_display_name(&drop);

    // Get asset_id for NFTs
    let asset_id = match &drop {
        Drop::WalletSend { asset_id, .. } => Some(asset_id.concatenated()),
        Drop::Tip { .. } => None,
    };

    let item_class = move || {
        let mut classes = vec!["drop-item"];
        if is_dragging.get() {
            classes.push("drop-item--dragging");
        }
        if is_drag_over.get() {
            classes.push("drop-item--drag-over");
        }
        classes.join(" ")
    };

    view! {
        <div
            class=item_class
            draggable=move || if readonly.get() { "false" } else { "true" }
            on:dragstart=move |ev| {
                if !readonly.get() {
                    ev.data_transfer()
                        .map(|dt| dt.set_effect_allowed("move"));
                    on_drag_start();
                }
            }
            on:dragend=move |_| on_drag_end()
            on:dragover=move |ev| {
                if !readonly.get() {
                    ev.prevent_default();
                    on_drag_over();
                }
            }
            on:dragleave=move |_| {}
            on:drop=move |ev| {
                ev.prevent_default();
                on_drop();
            }
        >
            // Position indicator
            <div class="drop-item__position">{index + 1}</div>

            // Card display
            <div class="drop-item__card">
                {match asset_id {
                    Some(id) => view! {
                        <AssetCard
                            asset_id=id
                            name=display_name.clone()
                            size=size
                            show_name=true
                            is_static=true
                        />
                    }.into_any(),
                    None => view! {
                        // Tip display - text card
                        <div class="drop-item__tip-card" style=format!(
                            "width: {}px; height: {}px;",
                            size.pixels().unwrap_or(120),
                            size.pixels().unwrap_or(120)
                        )>
                            <div class="drop-item__tip-icon">"$"</div>
                            <div class="drop-item__tip-name">{display_name.clone()}</div>
                        </div>
                    }.into_any(),
                }}
            </div>

            // Remove button (shown on hover, hidden in readonly)
            <Show when=move || !readonly.get()>
                <button
                    class="drop-item__remove"
                    on:click=move |ev| {
                        ev.stop_propagation();
                        on_remove();
                    }
                    title="Remove"
                >
                    {"\u{00D7}"} // ×
                </button>
            </Show>
        </div>
    }
}

/// Modal for adding a new drop
#[component]
fn AddDropModal(
    #[prop(into)] open: Signal<bool>,
    on_close: impl Fn() + Send + Sync + 'static + Copy,
    on_add: impl Fn(Drop) + Send + Sync + 'static + Copy,
) -> impl IntoView {
    // Form state
    let (drop_type, set_drop_type) = signal("tip".to_string());
    let (tip_token, set_tip_token) = signal("ADA".to_string());
    let (tip_amount, set_tip_amount) = signal(100.0f64);
    let (cnft_input, set_cnft_input) = signal(String::new());
    let (cnft_error, set_cnft_error) = signal(Option::<String>::None);

    // Reset form when modal opens
    Effect::new(move |_| {
        if open.get() {
            set_drop_type.set("tip".to_string());
            set_tip_token.set("ADA".to_string());
            set_tip_amount.set(100.0);
            set_cnft_input.set(String::new());
            set_cnft_error.set(None);
        }
    });

    let handle_add = move |_| {
        let dtype = drop_type.get();
        if dtype == "tip" {
            let drop = Drop::tip(tip_token.get(), tip_amount.get());
            on_add(drop);
        } else {
            // CNFT - parse asset ID
            let input = cnft_input.get();
            match AssetId::parse_smart(input.trim()) {
                Ok(asset_id) => {
                    let drop = Drop::wallet_send_single(asset_id);
                    on_add(drop);
                }
                Err(_) => {
                    set_cnft_error.set(Some("Invalid asset ID format".to_string()));
                }
            }
        }
    };

    let type_options = Signal::derive(|| {
        vec![
            SelectOption::new("tip", "Tip (Fungible Token)"),
            SelectOption::new("cnft", "CNFT (NFT Transfer)"),
        ]
    });

    let can_add = move || {
        let dtype = drop_type.get();
        if dtype == "tip" {
            tip_amount.get() > 0.0 && !tip_token.get().is_empty()
        } else {
            !cnft_input.get().trim().is_empty()
        }
    };

    view! {
        <Modal
            open=open
            title="Add Prize"
            on_close=on_close
        >
            <div class="add-drop-modal">
                <div class="add-drop-modal__field">
                    <label>"Prize Type"</label>
                    <Select
                        value=drop_type
                        options=type_options
                        on_change=move |v| set_drop_type.set(v)
                    />
                </div>

                // Tip fields
                <Show when=move || drop_type.get() == "tip">
                    <div class="add-drop-modal__row">
                        <div class="add-drop-modal__field add-drop-modal__field--amount">
                            <label>"Amount"</label>
                            <input
                                type="number"
                                class="add-drop-modal__input"
                                prop:value=move || tip_amount.get().to_string()
                                on:input=move |ev| {
                                    let target = ev.target().unwrap();
                                    let input: web_sys::HtmlInputElement = target.unchecked_into();
                                    if let Ok(val) = input.value().parse::<f64>() {
                                        set_tip_amount.set(val);
                                    }
                                }
                                min="0"
                                step="0.01"
                            />
                        </div>
                        <div class="add-drop-modal__field add-drop-modal__field--token">
                            <label>"Token"</label>
                            <input
                                type="text"
                                class="add-drop-modal__input"
                                prop:value=tip_token
                                on:input=move |ev| {
                                    let target = ev.target().unwrap();
                                    let input: web_sys::HtmlInputElement = target.unchecked_into();
                                    set_tip_token.set(input.value());
                                }
                                placeholder="ADA"
                            />
                        </div>
                    </div>
                </Show>

                // CNFT fields
                <Show when=move || drop_type.get() == "cnft">
                    <div class="add-drop-modal__field">
                        <label>"Asset ID"</label>
                        <input
                            type="text"
                            class="add-drop-modal__input"
                            prop:value=cnft_input
                            on:input=move |ev| {
                                let target = ev.target().unwrap();
                                let input: web_sys::HtmlInputElement = target.unchecked_into();
                                set_cnft_input.set(input.value());
                                set_cnft_error.set(None);
                            }
                            placeholder="Paste asset ID (policy + asset name hex)"
                        />
                        <Show when=move || cnft_error.get().is_some()>
                            <span class="add-drop-modal__error">{move || cnft_error.get()}</span>
                        </Show>
                        <span class="add-drop-modal__hint">
                            "Formats: policy_id + asset_name_hex, or policy_id.asset_name_hex"
                        </span>
                    </div>
                </Show>

                <div class="add-drop-modal__actions">
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=move |()| on_close()
                    >
                        "Cancel"
                    </Button>
                    <Button
                        variant=ButtonVariant::Primary
                        on_click=handle_add
                        disabled=Signal::derive(move || !can_add())
                    >
                        "Add Prize"
                    </Button>
                </div>
            </div>
        </Modal>
    }
}
