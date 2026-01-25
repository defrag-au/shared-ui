//! CompactCard component story

use crate::stories::helpers::AttributeCard;
use leptos::prelude::*;
use ui_cardkit::{CompactCard, CompactSize, Owner};

#[component]
pub fn CompactCardStory() -> impl IntoView {
    let (click_count, set_click_count) = signal(0u32);

    view! {
        <div>
            <div class="story-header">
                <h2>"CompactCard"</h2>
                <p>"Square format card for deployed engines. Shows assets in a compact view suitable for displaying on the game board. Supports owner indicators and stat overlays."</p>
            </div>

            // Size Variants section
            <div class="story-section">
                <h3>"Size Variants"</h3>
                <div class="story-canvas" style="background: #0d0d1a; padding: 2rem;">
                    <div style="display: flex; align-items: flex-end; gap: 1.5rem; flex-wrap: wrap;">
                        <div style="text-align: center;">
                            <CompactCard
                                asset_id="b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839"
                                size=CompactSize::Sm
                            />
                            <div style="color: #888; font-size: 12px; margin-top: 4px;">"Sm (48px)"</div>
                        </div>
                        <div style="text-align: center;">
                            <CompactCard
                                asset_id="b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465323030"
                                size=CompactSize::Md
                            />
                            <div style="color: #888; font-size: 12px; margin-top: 4px;">"Md (64px)"</div>
                        </div>
                        <div style="text-align: center;">
                            <CompactCard
                                asset_id="b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465333333"
                                size=CompactSize::Lg
                            />
                            <div style="color: #888; font-size: 12px; margin-top: 4px;">"Lg (80px)"</div>
                        </div>
                    </div>
                </div>
            </div>

            // Owner Indicators section
            <div class="story-section">
                <h3>"Owner Indicators"</h3>
                <p class="story-description">"Visual distinction between your deployed cards and others' cards."</p>
                <div class="story-canvas" style="background: #0d0d1a; padding: 2rem;">
                    <div style="display: flex; gap: 2rem; flex-wrap: wrap;">
                        <div style="text-align: center;">
                            <CompactCard
                                asset_id="b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839"
                                size=CompactSize::Lg
                                owner=Signal::derive(|| Owner::You)
                            />
                            <div style="color: #4caf50; font-size: 12px; margin-top: 4px;">"Your Engine"</div>
                        </div>
                        <div style="text-align: center;">
                            <CompactCard
                                asset_id="b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465323030"
                                size=CompactSize::Lg
                                owner=Signal::derive(|| Owner::Other)
                            />
                            <div style="color: #9c27b0; font-size: 12px; margin-top: 4px;">"Fleet Engine"</div>
                        </div>
                        <div style="text-align: center;">
                            <CompactCard
                                asset_id="b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465333333"
                                size=CompactSize::Lg
                            />
                            <div style="color: #888; font-size: 12px; margin-top: 4px;">"No owner"</div>
                        </div>
                    </div>
                </div>
            </div>

            // Interactive Demo
            <div class="story-section">
                <h3>"Interactive Demo"</h3>
                <p class="story-description">"Click cards to open detail modal (simulated here with click counter)."</p>
                <div class="story-canvas" style="background: #0d0d1a; padding: 2rem;">
                    <div style="display: flex; gap: 1rem; flex-wrap: wrap;">
                        {(0..6).map(|i| {
                            let owner = if i < 3 { Owner::You } else { Owner::Other };
                            let asset_ids = [
                                "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839",
                                "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465323030",
                                "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465333333",
                                "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465343434",
                                "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465353535",
                                "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465363636",
                            ];
                            view! {
                                <CompactCard
                                    asset_id=asset_ids[i]
                                    size=CompactSize::Md
                                    owner=Signal::derive(move || owner)
                                    on_click=move |_| set_click_count.update(|c| *c += 1)
                                />
                            }
                        }).collect_view()}
                    </div>
                    <div style="margin-top: 1rem; color: #888;">
                        {move || format!("Cards clicked: {}", click_count.get())}
                    </div>
                </div>
            </div>

            // Props section
            <div class="story-section">
                <h3>"Props"</h3>
                <div class="story-canvas">
                    <div class="story-grid">
                        <AttributeCard
                            name="asset_id"
                            values="Signal<String>"
                            description="Cardano asset ID for IIIF image lookup"
                        />
                        <AttributeCard
                            name="image_url"
                            values="Signal<String>"
                            description="Direct image URL (fallback when asset_id not available)"
                        />
                        <AttributeCard
                            name="size"
                            values="CompactSize (Sm|Md|Lg)"
                            description="Square dimensions: 48px, 64px (default), 80px"
                        />
                        <AttributeCard
                            name="owner"
                            values="Signal<Owner> (You|Other)"
                            description="Owner indicator - adds colored border/background"
                        />
                        <AttributeCard
                            name="on_click"
                            values="Callback<()>"
                            description="Called when card is clicked (typically opens detail modal)"
                        />
                        <AttributeCard
                            name="stats"
                            values="ChildrenFn"
                            description="Overlay content for stats badges"
                        />
                    </div>
                </div>
            </div>

            // Code example
            <div class="story-section">
                <h3>"Usage"</h3>
                <pre class="code-block">{r##"use ui_cardkit::{CompactCard, CompactSize, Owner};

// Basic deployed engine
view! {
    <CompactCard
        asset_id=engine.asset_id
        size=CompactSize::Md
        owner=Signal::derive(|| Owner::You)
        on_click=move |_| show_detail_modal()
    />
}

// With stat overlay
view! {
    <CompactCard
        asset_id=asset_id
        owner=owner
        stats=view! {
            <StatBadge value="350" icon="⚡" />
        }
    />
}"##}</pre>
            </div>
        </div>
    }
}
