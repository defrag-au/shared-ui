//! Policy Folder Component
//!
//! A collapsible folder for displaying NFTs grouped by policy ID.
//! Supports infinite scroll via IntersectionObserver for large collections.
//!
//! ## Features
//!
//! - Collapsible with expand/collapse toggle
//! - Infinite scroll pagination (loads more as user scrolls)
//! - Optional custom title (for known collection names)
//! - Controlled or uncontrolled expansion state
//!
//! ## Usage
//!
//! ```ignore
//! use ui_components::PolicyFolder;
//! use wallet_pallas::PolicyGroup;
//!
//! <PolicyFolder
//!     group=policy_group
//!     title="Unsigned Algorithms"
//!     initial_batch_size=12
//! />
//! ```

use crate::asset_card::AssetCard;
use crate::image_card::CardSize;
use crate::loading_overlay::Spinner;
use leptos::prelude::*;
use wallet_pallas::{NativeToken, PolicyGroup};
use wasm_bindgen::prelude::*;

/// Collapsible folder for displaying NFTs from a single policy
#[component]
pub fn PolicyFolder(
    /// Policy group data (contains policy_id, tokens, nft_count)
    group: PolicyGroup,

    /// Optional title override (for when we know the collection name)
    #[prop(into, optional)]
    title: Option<String>,

    /// Initial batch size for infinite scroll
    #[prop(optional, default = 12)]
    initial_batch_size: usize,

    /// Batch size for loading more
    #[prop(optional, default = 12)]
    load_more_batch_size: usize,

    /// Controlled expanded state (optional - uses internal state if not provided)
    #[prop(into, optional)]
    expanded: Option<Signal<bool>>,

    /// Callback when folder is toggled
    #[prop(into, optional)]
    on_toggle: Option<Callback<bool>>,

    /// Additional CSS class
    #[prop(into, optional)]
    class: Option<String>,
) -> impl IntoView {
    // Internal expansion state (used if not controlled)
    let (internal_expanded, set_internal_expanded) = signal(false);

    // Use controlled or internal state
    let is_expanded = move || {
        expanded
            .map(|s| s.get())
            .unwrap_or_else(|| internal_expanded.get())
    };

    // Toggle handler
    let toggle = move |_| {
        let new_state = !is_expanded();
        if expanded.is_none() {
            set_internal_expanded.set(new_state);
        }
        if let Some(cb) = on_toggle {
            cb.run(new_state);
        }
    };

    // Pagination state
    let (visible_count, set_visible_count) = signal(initial_batch_size);

    // Extract data from group
    let policy_id = group.policy_id.clone();
    let policy_id_short = group.policy_id_short.clone();
    let display_title = title.unwrap_or_else(|| policy_id_short.clone());
    let nfts: Vec<NativeToken> = group.nfts().into_iter().cloned().collect();
    let total_count = nfts.len();

    // Sentinel ref for intersection observer
    let sentinel_ref = NodeRef::<leptos::html::Div>::new();

    // Set up intersection observer when expanded
    Effect::new(move |_| {
        if !is_expanded() {
            return;
        }

        let Some(sentinel) = sentinel_ref.get() else {
            return;
        };

        let batch_size = load_more_batch_size;
        let callback = Closure::wrap(Box::new(
            move |entries: js_sys::Array, _observer: web_sys::IntersectionObserver| {
                for entry in entries.iter() {
                    let entry: web_sys::IntersectionObserverEntry = entry.unchecked_into();
                    if entry.is_intersecting() {
                        set_visible_count.update(|count| {
                            *count = (*count + batch_size).min(total_count);
                        });
                    }
                }
            },
        )
            as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);

        let options = web_sys::IntersectionObserverInit::new();
        options.set_root_margin("100px");

        if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
            callback.as_ref().unchecked_ref(),
            &options,
        ) {
            observer.observe(&sentinel);
            callback.forget();
        }
    });

    let wrapper_class = move || {
        let mut classes = vec!["ui-policy-folder"];
        if is_expanded() {
            classes.push("ui-policy-folder--expanded");
        }
        if let Some(ref c) = class {
            classes.push(c);
        }
        classes.join(" ")
    };

    view! {
        <div class=wrapper_class>
            <button class="ui-policy-folder__header" on:click=toggle>
                <span class="ui-policy-folder__icon">
                    {move || if is_expanded() { "▼" } else { "▶" }}
                </span>
                <span class="ui-policy-folder__title">{display_title.clone()}</span>
                <span class="ui-policy-folder__count">{total_count} " NFTs"</span>
            </button>

            {move || is_expanded().then(|| {
                let visible = visible_count.get();
                let nfts_to_show: Vec<NativeToken> = nfts.clone().into_iter().take(visible).collect();
                let has_more = visible < total_count;
                let policy_for_display = policy_id.clone();

                view! {
                    <div class="ui-policy-folder__content">
                        <div class="ui-policy-folder__grid">
                            {nfts_to_show.into_iter().map(|nft| {
                                let asset_id = nft.asset_id();
                                let name = nft.display_name();

                                view! {
                                    <AssetCard
                                        asset_id=Signal::derive(move || asset_id.clone())
                                        name=Signal::derive(move || name.clone())
                                        size=CardSize::Auto
                                        show_name=true
                                        is_static=true
                                    />
                                }
                            }).collect::<Vec<_>>()}
                        </div>

                        // Sentinel for infinite scroll
                        {has_more.then(|| view! {
                            <div class="ui-policy-folder__sentinel" node_ref=sentinel_ref>
                                <Spinner />
                                <span>"Loading more..."</span>
                            </div>
                        })}

                        // Full policy ID for reference
                        <div class="ui-policy-folder__policy-id">
                            <span class="ui-policy-folder__policy-label">"Policy ID:"</span>
                            <code class="ui-policy-folder__policy-value">{policy_for_display}</code>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
