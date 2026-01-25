//! Leviathan Hunt Demo - game layout prototype

use crate::stories::helpers::AttributeCard;
use leptos::prelude::*;
use ui_cardkit::{
    CardSize, CompactCard, CompactSize, DeploymentSummary, GameCard, HealthBar, MonsterStage, Owner,
};

/// Leviathan Hunt game layout demo
#[component]
pub fn LeviathanDemoStory() -> impl IntoView {
    // Game state
    let (monster_health, set_monster_health) = signal(7500u32);
    let monster_max_health = 10000u32;

    // Deployed counts
    let (your_count, _) = signal(3usize);
    let (others_count, _) = signal(5usize);
    let (your_power, _) = signal(850u32);
    let (others_power, _) = signal(1305u32);

    // Selected card in hand
    let (selected_index, set_selected_index) = signal(Option::<usize>::None);

    // Asset IDs for demo
    let hand_assets = [
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839",
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465323030",
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465333333",
    ];

    let your_deployed_assets = [
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465343434",
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465353535",
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465363636",
    ];

    let others_deployed_assets = [
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465373737",
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465383838",
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465393939",
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465303030",
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313131",
    ];

    view! {
        <div>
            <div class="story-header">
                <h2>"Leviathan Hunt Demo"</h2>
                <p>"Game layout prototype. MonsterStage is just a themed background - deployment UI is composed separately and can adapt to screen size."</p>
            </div>

            <div class="story-section">
                <div class="story-canvas" style="background: #0a0a14; padding: 0; overflow: hidden; border-radius: 8px;">
                    <div style="display: flex; flex-direction: column; min-height: 550px;">

                        // Monster Stage (background) with overlaid UI
                        <div style="flex: 1; position: relative;">
                            // The monster background
                            <MonsterStage
                                monster_image=Signal::derive(|| "https://images.unsplash.com/photo-1518709268805-4e9042af9f23?w=800".to_string())
                                monster_name=Signal::derive(|| "Leviathan".to_string())
                                health_percent=Signal::derive(move || monster_health.get() as f32 / monster_max_health as f32)
                            />

                            // Health bar - positioned over the stage
                            <div style="position: absolute; top: 16px; left: 16px; right: 16px;">
                                <div style="background: rgba(0,0,0,0.7); padding: 8px 16px; border-radius: 8px;">
                                    <div style="color: #e0e0e0; font-size: 14px; margin-bottom: 4px;">"Leviathan"</div>
                                    <HealthBar
                                        current=monster_health
                                        max=Signal::derive(move || monster_max_health)
                                        show_value=true
                                    />
                                </div>
                            </div>

                            // Deployed engines - positioned over the stage
                            <div style="position: absolute; bottom: 60px; left: 20px; display: flex; gap: 8px;">
                                {your_deployed_assets.iter().map(|asset_id| {
                                    view! {
                                        <CompactCard
                                            asset_id=asset_id.to_string()
                                            size=CompactSize::Md
                                            owner=Signal::derive(|| Owner::You)
                                        />
                                    }
                                }).collect_view()}
                            </div>

                            <div style="position: absolute; bottom: 60px; right: 20px; display: flex; gap: 8px;">
                                {others_deployed_assets.iter().map(|asset_id| {
                                    view! {
                                        <CompactCard
                                            asset_id=asset_id.to_string()
                                            size=CompactSize::Md
                                            owner=Signal::derive(|| Owner::Other)
                                        />
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        // Deployment Summary
                        <div style="padding: 8px 16px; background: #0d0d1a;">
                            <DeploymentSummary
                                your_count=Signal::derive(move || your_count.get())
                                others_count=Signal::derive(move || others_count.get())
                                your_power=Signal::derive(move || your_power.get())
                                others_power=Signal::derive(move || others_power.get())
                            />
                        </div>

                        // Player's Hand
                        <div style="background: #1a1a2e; border-top: 1px solid #2a2a4e; padding: 16px;">
                            <div style="display: flex; justify-content: center; gap: 12px;">
                                {hand_assets.iter().enumerate().map(|(i, asset_id)| {
                                    let is_selected = Signal::derive(move || selected_index.get() == Some(i));
                                    let asset_id = asset_id.to_string();
                                    view! {
                                        <GameCard
                                            size=CardSize::Sm
                                            highlighted=is_selected
                                            on_click=move |_| {
                                                if selected_index.get() == Some(i) {
                                                    set_monster_health.update(|h| *h = h.saturating_sub(250));
                                                    set_selected_index.set(None);
                                                } else {
                                                    set_selected_index.set(Some(i));
                                                }
                                            }
                                        >
                                            <div style="width: 100%; height: 100%; position: relative;">
                                                <img
                                                    src=format!("https://iiif.hodlcroft.com/iiif/3/{}:{}/full/400,/0/default.jpg",
                                                        &asset_id[..56],
                                                        &asset_id[56..])
                                                    style="width: 100%; height: 100%; object-fit: cover;"
                                                />
                                                <div style="position: absolute; bottom: 4px; right: 4px; background: rgba(0,0,0,0.8); padding: 2px 6px; border-radius: 4px; font-size: 11px; color: #ffd700;">
                                                    {format!("⚡{}", 250 + i * 35)}
                                                </div>
                                            </div>
                                        </GameCard>
                                    }
                                }).collect_view()}
                            </div>
                            <div style="text-align: center; margin-top: 8px; color: #888; font-size: 12px;">
                                {move || match selected_index.get() {
                                    Some(_) => "Click selected card again to deploy",
                                    None => "Click a card to select it",
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Architecture note
            <div class="story-section">
                <h3>"Architecture"</h3>
                <div class="story-canvas">
                    <p style="color: #e0e0e0; margin-bottom: 1rem;">
                        "MonsterStage is just a themed background with health state. All deployment UI (engine positions, layout) is composed on top by the consumer. This allows different layouts for desktop vs mobile."
                    </p>
                    <div class="story-grid">
                        <AttributeCard
                            name="MonsterStage"
                            values="Background"
                            description="Monster image with health-based damage effects. No children - pure visual."
                        />
                        <AttributeCard
                            name="Deployment UI"
                            values="Consumer's choice"
                            description="Positioned absolutely over the stage. Can be scatter, grid, list - whatever fits the screen."
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
