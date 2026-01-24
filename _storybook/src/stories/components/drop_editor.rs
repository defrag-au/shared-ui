//! DropEditor component story

use crate::stories::helpers::AttributeCard;
use asset_intents::{AssetId, Drop};
use leptos::prelude::*;
use ui_components::DropEditor;

/// Sample asset IDs for the story
fn sample_asset_id() -> AssetId {
    AssetId::new_unchecked(
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6".to_string(),
        "50697261746531303836".to_string(), // Pirate1086
    )
}

fn sample_asset_id_2() -> AssetId {
    AssetId::new_unchecked(
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6".to_string(),
        "506972617465313839".to_string(), // Pirate189
    )
}

#[component]
pub fn DropEditorStory() -> impl IntoView {
    // Basic interactive example
    let (drops, set_drops) = signal(vec![
        Drop::tip("ADA", 500.0),
        Drop::tip("ADA", 250.0),
        Drop::wallet_send_single(sample_asset_id()),
    ]);

    // Readonly example
    let readonly_drops = Signal::derive(move || {
        vec![
            Drop::tip("SNEK", 1000.0),
            Drop::wallet_send_single(sample_asset_id_2()),
        ]
    });

    // Empty example
    let (empty_drops, set_empty_drops) = signal(Vec::<Drop>::new());

    // Tips only example
    let (tip_drops, set_tip_drops) = signal(vec![
        Drop::tip("ADA", 100.0),
        Drop::tip("ADA", 50.0),
        Drop::tip("ADA", 25.0),
    ]);

    view! {
        <div>
            <div class="story-header">
                <h2>"DropEditor"</h2>
                <p>"An editor for configuring reward drops (tips and wallet sends). Used for raffle prizes, achievement rewards, giveaways, etc."</p>
            </div>

            // Basic interactive example
            <div class="story-section">
                <h3>"Interactive Example"</h3>
                <p style="color: #888; margin-bottom: 1rem; font-size: 0.875rem;">
                    "Try adding, removing, and reordering drops. Changes are logged to console."
                </p>
                <div class="story-canvas">
                    <DropEditor
                        drops=drops
                        on_change=move |new_drops| {
                            web_sys::console::log_1(&format!("Drops changed: {:?}", new_drops).into());
                            set_drops.set(new_drops);
                        }
                    />
                </div>
                <div style="margin-top: 1rem; padding: 0.75rem; background: #1a1a2e; border-radius: 4px;">
                    <p style="font-size: 0.75rem; color: #888; margin-bottom: 0.5rem;">"Current state:"</p>
                    <pre style="font-size: 0.75rem; color: #888; margin: 0; white-space: pre-wrap;">
                        {move || format!("{:#?}", drops.get())}
                    </pre>
                </div>
            </div>

            // Readonly example
            <div class="story-section">
                <h3>"Readonly Mode"</h3>
                <p style="color: #888; margin-bottom: 1rem; font-size: 0.875rem;">
                    "When readonly is true, editing controls are hidden."
                </p>
                <div class="story-canvas">
                    <DropEditor
                        drops=readonly_drops
                        on_change=move |_| {}
                        readonly=true
                    />
                </div>
            </div>

            // Empty state
            <div class="story-section">
                <h3>"Empty State"</h3>
                <p style="color: #888; margin-bottom: 1rem; font-size: 0.875rem;">
                    "Shows a helpful message when no drops are configured."
                </p>
                <div class="story-canvas">
                    <DropEditor
                        drops=empty_drops
                        on_change=move |new_drops| set_empty_drops.set(new_drops)
                    />
                </div>
            </div>

            // Tips only example
            <div class="story-section">
                <h3>"Tips Only"</h3>
                <p style="color: #888; margin-bottom: 1rem; font-size: 0.875rem;">
                    "Example with only tip prizes (fungible tokens)."
                </p>
                <div class="story-canvas">
                    <DropEditor
                        drops=tip_drops
                        on_change=move |new_drops| set_tip_drops.set(new_drops)
                    />
                </div>
            </div>

            // Drop types
            <div class="story-section">
                <h3>"Drop Types"</h3>
                <div class="story-canvas">
                    <div style="display: flex; flex-direction: column; gap: 1rem;">
                        <div>
                            <h4 style="margin: 0 0 0.5rem; font-size: 0.875rem; color: #fff;">"Tip"</h4>
                            <p style="margin: 0 0 0.5rem; font-size: 0.75rem; color: #888;">
                                "Fungible token tips via tipping services (e.g., FarmBot). Supports any token symbol and fractional amounts."
                            </p>
                            <pre class="code-block" style="margin: 0; font-size: 0.75rem;">
                                {"Drop::tip(\"ADA\", 100.0)"}
                            </pre>
                        </div>
                        <div>
                            <h4 style="margin: 0 0 0.5rem; font-size: 0.875rem; color: #fff;">"WalletSend"</h4>
                            <p style="margin: 0 0 0.5rem; font-size: 0.75rem; color: #888;">
                                "NFT or token transfers via wallet services (e.g., cnft.dev). Requires an AssetId and quantity."
                            </p>
                            <pre class="code-block" style="margin: 0; font-size: 0.75rem;">
                                {"Drop::wallet_send(asset_id, 1)\nDrop::wallet_send_single(asset_id)"}
                            </pre>
                        </div>
                    </div>
                </div>
            </div>

            // Props section
            <div class="story-section">
                <h3>"Props"</h3>
                <div class="story-canvas">
                    <div class="story-grid">
                        <AttributeCard
                            name="drops"
                            values="Signal<Vec<Drop>>"
                            description="The current list of drops to display/edit"
                        />
                        <AttributeCard
                            name="on_change"
                            values="Callback<Vec<Drop>>"
                            description="Called when drops are modified (add, remove, reorder, edit)"
                        />
                        <AttributeCard
                            name="readonly"
                            values="Signal<bool> (optional)"
                            description="If true, disables all editing controls"
                        />
                    </div>
                </div>
            </div>

            // Code example
            <div class="story-section">
                <h3>"Usage"</h3>
                <pre class="code-block">{r##"use ui_components::DropEditor;
use asset_intents::{Drop, AssetId};

let (drops, set_drops) = signal(vec![
    Drop::tip("ADA", 500.0),
    Drop::tip("ADA", 250.0),
    Drop::wallet_send_single(my_asset_id),
]);

view! {
    <DropEditor
        drops=drops
        on_change=move |new_drops| set_drops.set(new_drops)
    />
}

// Readonly mode
view! {
    <DropEditor
        drops=drops
        on_change=move |_| {}
        readonly=true
    />
}"##}</pre>
            </div>

            // Integration notes
            <div class="story-section">
                <h3>"Integration Notes"</h3>
                <div class="story-canvas">
                    <ul style="margin: 0; padding-left: 1.5rem; color: #888; font-size: 0.875rem;">
                        <li style="margin-bottom: 0.5rem;">
                            "The Drop enum serializes with a \"type\" tag for JSON compatibility:"
                            <pre class="code-block" style="margin-top: 0.5rem; font-size: 0.75rem;">
                                {r#"{"type": "tip", "token": "ADA", "amount": 100.0}"#}
                            </pre>
                        </li>
                        <li style="margin-bottom: 0.5rem;">
                            "CNFT prizes are added by pasting a valid asset ID (policy_id + asset_name_hex or policy_id.asset_name_hex format)"
                        </li>
                        <li style="margin-bottom: 0.5rem;">
                            "Position numbers (1st, 2nd, 3rd...) are automatically displayed for ranked prizes"
                        </li>
                        <li>
                            "Asset thumbnails are loaded via IIIF from iiif.hodlcroft.com"
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
