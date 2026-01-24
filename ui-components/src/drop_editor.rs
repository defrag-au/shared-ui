//! Drop Editor Component
//!
//! An editor for configuring reward drops (tips and wallet sends).
//! Used for raffle prizes, achievement rewards, giveaways, etc.
//!
//! # Example
//!
//! ```ignore
//! use ui_components::DropEditor;
//! use asset_intents::Drop;
//!
//! let (drops, set_drops) = signal(vec![
//!     Drop::tip("ADA", 100.0),
//!     Drop::tip("ADA", 50.0),
//! ]);
//!
//! view! {
//!     <DropEditor
//!         drops=drops
//!         on_change=move |new_drops| set_drops.set(new_drops)
//!     />
//! }
//! ```

use crate::{AssetCard, Button, ButtonVariant, CardSize};
use asset_intents::{AssetId, Drop};
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Editor for a list of reward drops
#[component]
pub fn DropEditor(
    /// The current list of drops
    #[prop(into)]
    drops: Signal<Vec<Drop>>,
    /// Called when drops are modified
    #[prop(into)]
    on_change: Callback<Vec<Drop>>,
    /// If true, disables editing
    #[prop(into, optional)]
    readonly: Signal<bool>,
) -> impl IntoView {
    // State for the CNFT input section
    let (show_cnft_input, set_show_cnft_input) = signal(false);
    let (cnft_input_value, set_cnft_input_value) = signal(String::new());
    let (cnft_error, set_cnft_error) = signal(Option::<String>::None);

    // Helper to update drops and notify parent
    let update_drops = move |new_drops: Vec<Drop>| {
        on_change.run(new_drops);
    };

    // Add a tip drop
    let add_tip = move |_| {
        let mut new_drops = drops.get();
        new_drops.push(Drop::tip("ADA", 100.0));
        update_drops(new_drops);
    };

    // Show CNFT input
    let show_cnft = move |_| {
        set_show_cnft_input.set(true);
        set_cnft_error.set(None);
    };

    // Cancel CNFT input
    let cancel_cnft = move |_| {
        set_show_cnft_input.set(false);
        set_cnft_input_value.set(String::new());
        set_cnft_error.set(None);
    };

    // Handle CNFT input change - auto-add on valid paste
    let on_cnft_input = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        let value = input.value();
        let trimmed = value.trim();

        if trimmed.is_empty() {
            set_cnft_input_value.set(value);
            set_cnft_error.set(None);
            return;
        }

        match AssetId::parse_smart(trimmed) {
            Ok(asset_id) => {
                // Valid asset ID - add it
                let mut new_drops = drops.get();
                new_drops.push(Drop::wallet_send(asset_id, 1));
                update_drops(new_drops);

                // Reset input
                set_show_cnft_input.set(false);
                set_cnft_input_value.set(String::new());
                set_cnft_error.set(None);
            }
            Err(_) => {
                // Not valid yet - just update value
                set_cnft_input_value.set(value);
            }
        }
    };

    view! {
        <div class="drop-editor">
            // Drop items list
            <div class="drop-editor__items">
                <Show
                    when=move || !drops.get().is_empty()
                    fallback=move || {
                        view! {
                            <Show when=move || !show_cnft_input.get()>
                                <div class="drop-editor__empty">
                                    "No prizes configured. Add a prize below."
                                </div>
                            </Show>
                        }
                    }
                >
                    <For
                        each={move || drops.get().into_iter().enumerate().collect::<Vec<_>>()}
                        key=|(idx, _)| *idx
                        let:item
                    >
                        {
                            let (idx, drop) = item;
                            let drops_len = drops.get().len();
                            view! {
                                <DropItem
                                    index=idx
                                    drop=drop
                                    total=drops_len
                                    readonly=readonly
                                    on_update=move |new_drop| {
                                        let mut new_drops = drops.get();
                                        new_drops[idx] = new_drop;
                                        update_drops(new_drops);
                                    }
                                    on_remove=move || {
                                        let mut new_drops = drops.get();
                                        new_drops.remove(idx);
                                        update_drops(new_drops);
                                    }
                                    on_move_up=move || {
                                        if idx > 0 {
                                            let mut new_drops = drops.get();
                                            new_drops.swap(idx, idx - 1);
                                            update_drops(new_drops);
                                        }
                                    }
                                    on_move_down=move || {
                                        if idx < drops_len - 1 {
                                            let mut new_drops = drops.get();
                                            new_drops.swap(idx, idx + 1);
                                            update_drops(new_drops);
                                        }
                                    }
                                />
                            }
                        }
                    </For>
                </Show>
            </div>

            // CNFT input section (shown when adding CNFT)
            <Show when=move || show_cnft_input.get() && !readonly.get()>
                <div class="drop-editor__cnft-input">
                    <input
                        type="text"
                        placeholder="Paste asset ID..."
                        prop:value=cnft_input_value
                        on:input=on_cnft_input
                        class="drop-editor__input"
                    />
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=cancel_cnft
                    >
                        "Cancel"
                    </Button>
                    <Show when=move || cnft_error.get().is_some()>
                        <span class="drop-editor__error">{move || cnft_error.get()}</span>
                    </Show>
                </div>
            </Show>

            // Add buttons (hidden in readonly mode)
            <Show when=move || !readonly.get() && !show_cnft_input.get()>
                <div class="drop-editor__actions">
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=add_tip
                    >
                        "+ Add Tip Prize"
                    </Button>
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=show_cnft
                    >
                        "+ Add CNFT Prize"
                    </Button>
                </div>
            </Show>
        </div>
    }
}

/// Position suffix (1st, 2nd, 3rd, 4th...)
fn position_suffix(index: usize) -> &'static str {
    match index + 1 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}

/// A single drop item in the editor
#[component]
fn DropItem(
    /// Zero-based index
    index: usize,
    /// The drop to display/edit
    drop: Drop,
    /// Total number of drops (for move button state)
    total: usize,
    /// Readonly mode
    #[prop(into)]
    readonly: Signal<bool>,
    /// Called when the drop is updated
    on_update: impl Fn(Drop) + Send + Sync + 'static + Copy,
    /// Called when the drop should be removed
    on_remove: impl Fn() + Send + Sync + 'static + Copy,
    /// Called when the drop should move up
    on_move_up: impl Fn() + Send + Sync + 'static + Copy,
    /// Called when the drop should move down
    on_move_down: impl Fn() + Send + Sync + 'static + Copy,
) -> impl IntoView {
    let is_readonly = move || readonly.get();
    let can_move_up = index > 0;
    let can_move_down = index < total - 1;

    view! {
        <div class="drop-item">
            // Position badge
            <div class="drop-item__position">
                {index + 1}{position_suffix(index)}
            </div>

            // Content (varies by drop type)
            <div class="drop-item__content">
                {match drop.clone() {
                    Drop::Tip { token, amount } => {
                        view! {
                            <TipDropContent
                                token=token
                                amount=amount
                                readonly=readonly
                                on_update=on_update
                            />
                        }.into_any()
                    }
                    Drop::WalletSend { asset_id, amount } => {
                        view! {
                            <WalletSendDropContent
                                asset_id=asset_id
                                amount=amount
                                readonly=readonly
                                on_update=on_update
                            />
                        }.into_any()
                    }
                }}
            </div>

            // Action buttons (hidden in readonly mode)
            <Show when=move || !is_readonly()>
                <div class="drop-item__actions">
                    <button
                        class="drop-item__move"
                        disabled=!can_move_up
                        on:click=move |_| on_move_up()
                        title="Move up"
                    >
                        {"\u{2191}"} // ↑
                    </button>
                    <button
                        class="drop-item__move"
                        disabled=!can_move_down
                        on:click=move |_| on_move_down()
                        title="Move down"
                    >
                        {"\u{2193}"} // ↓
                    </button>
                    <button
                        class="drop-item__remove"
                        on:click=move |_| on_remove()
                        title="Remove"
                    >
                        {"\u{00D7}"} // ×
                    </button>
                </div>
            </Show>
        </div>
    }
}

/// Content for a Tip drop
#[component]
fn TipDropContent(
    token: String,
    amount: f64,
    #[prop(into)] readonly: Signal<bool>,
    on_update: impl Fn(Drop) + 'static + Copy,
) -> impl IntoView {
    let (current_token, set_current_token) = signal(token);
    let (current_amount, set_current_amount) = signal(amount);

    // Update parent when values change
    let notify_update = move || {
        on_update(Drop::tip(current_token.get(), current_amount.get()));
    };

    let on_amount_input = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        if let Ok(val) = input.value().parse::<f64>() {
            set_current_amount.set(val);
            notify_update();
        }
    };

    let on_token_input = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        set_current_token.set(input.value());
        notify_update();
    };

    view! {
        <span class="drop-item__badge drop-item__badge--tip">"Tip"</span>
        <input
            type="number"
            class="drop-item__input drop-item__input--amount"
            prop:value=move || current_amount.get().to_string()
            on:input=on_amount_input
            min="0"
            step="0.01"
            disabled=readonly
        />
        <span class="drop-item__separator">{"\u{00D7}"}</span> // ×
        <input
            type="text"
            class="drop-item__input drop-item__input--token"
            prop:value=current_token
            on:input=on_token_input
            placeholder="ADA"
            disabled=readonly
        />
    }
}

/// Content for a WalletSend drop
#[component]
fn WalletSendDropContent(
    asset_id: AssetId,
    amount: u64,
    #[prop(into)] readonly: Signal<bool>,
    on_update: impl Fn(Drop) + 'static + Copy,
) -> impl IntoView {
    let (current_amount, set_current_amount) = signal(amount);
    let asset_id_clone = asset_id.clone();

    let on_amount_input = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        if let Ok(val) = input.value().parse::<u64>() {
            set_current_amount.set(val);
            on_update(Drop::wallet_send(asset_id_clone.clone(), val));
        }
    };

    // Short display of asset ID
    let short_id = format!(
        "{}...{}",
        &asset_id.policy_id()[..8],
        &asset_id.asset_name_hex()[asset_id.asset_name_hex().len().saturating_sub(6)..]
    );

    view! {
        <span class="drop-item__badge drop-item__badge--cnft">"CNFT"</span>
        <input
            type="number"
            class="drop-item__input drop-item__input--qty"
            prop:value=move || current_amount.get().to_string()
            on:input=on_amount_input
            min="1"
            disabled=readonly
        />
        <span class="drop-item__separator">{"\u{00D7}"}</span> // ×
        <div class="drop-item__asset">
            <AssetCard
                asset_id=asset_id.concatenated()
                name=asset_id.asset_name()
                size=CardSize::Xs
            />
            <div class="drop-item__asset-info">
                <div class="drop-item__asset-name">{asset_id.asset_name()}</div>
                <div class="drop-item__asset-id" title=asset_id.concatenated()>{short_id}</div>
            </div>
        </div>
    }
}
