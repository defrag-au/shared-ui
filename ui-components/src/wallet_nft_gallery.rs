//! Wallet NFT Gallery Component
//!
//! A gallery view for displaying NFTs grouped by policy.
//! Uses PolicyFolder components for each policy group.
//!
//! ## Features
//!
//! - Displays NFTs grouped by policy ID
//! - Summary header with NFT and policy counts
//! - Loading and empty states
//! - Data-driven - takes `Vec<PolicyGroup>` as input
//!
//! ## Usage
//!
//! ```ignore
//! use ui_components::WalletNftGallery;
//! use wallet_pallas::PolicyGroup;
//!
//! let groups: Signal<Vec<PolicyGroup>> = ...;
//! let loading: Signal<bool> = ...;
//!
//! <WalletNftGallery
//!     groups=groups
//!     loading=loading
//!     show_summary=true
//! />
//! ```

use crate::empty_state::EmptyState;
use crate::loading_overlay::Spinner;
use crate::policy_folder::PolicyFolder;
use leptos::prelude::*;
use wallet_pallas::PolicyGroup;

/// Gallery view for displaying NFTs grouped by policy
#[component]
pub fn WalletNftGallery(
    /// NFT policy groups to display
    #[prop(into)]
    groups: Signal<Vec<PolicyGroup>>,

    /// Loading state
    #[prop(into, optional)]
    loading: Option<Signal<bool>>,

    /// Show summary header with NFT/policy counts
    #[prop(optional, default = true)]
    show_summary: bool,

    /// Empty state message
    #[prop(into, optional, default = "No NFTs found".into())]
    empty_message: String,

    /// Additional CSS class
    #[prop(into, optional)]
    class: Option<String>,
) -> impl IntoView {
    let is_loading = move || loading.map(|s| s.get()).unwrap_or(false);

    // Compute totals for summary
    let nft_count = Memo::new(move |_| groups.get().iter().map(|g| g.nft_count).sum::<usize>());

    let policy_count = Memo::new(move |_| groups.get().len());

    let wrapper_class = {
        let class = class.clone();
        move || {
            let mut classes = vec!["ui-nft-gallery"];
            if let Some(ref c) = class {
                classes.push(c);
            }
            classes.join(" ")
        }
    };

    view! {
        <div class=wrapper_class>
            // Loading state
            {move || is_loading().then(|| view! {
                <div class="ui-nft-gallery__loading">
                    <Spinner />
                    <span>"Loading NFTs..."</span>
                </div>
            })}

            // Summary header
            {move || (show_summary && !is_loading() && nft_count.get() > 0).then(|| view! {
                <div class="ui-nft-gallery__summary">
                    <span class="ui-nft-gallery__count">{nft_count.get()}</span>
                    <span class="ui-nft-gallery__policies">
                        " NFTs across " {policy_count.get()} " policies"
                    </span>
                </div>
            })}

            // Empty state
            {move || (!is_loading() && groups.get().is_empty()).then(|| view! {
                <EmptyState message=empty_message.clone() />
            })}

            // Policy folders
            {move || {
                let current_groups = groups.get();
                (!is_loading() && !current_groups.is_empty()).then(|| view! {
                    <div class="ui-nft-gallery__folders">
                        {current_groups.into_iter().map(|group| {
                            view! { <PolicyFolder group=group /> }
                        }).collect::<Vec<_>>()}
                    </div>
                })
            }}
        </div>
    }
}
