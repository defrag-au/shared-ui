//! Asset Grid component story

use crate::stories::helpers::AttributeCard;
use leptos::prelude::*;
use ui_components::{AssetCard, AssetGrid, CardSize, ImageCard, StatPill};

// Sample Black Flag pirate asset IDs for demos
const SAMPLE_ASSETS: &[(&str, &str)] = &[
    (
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839",
        "Pirate #189",
    ),
    (
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465323030",
        "Pirate #200",
    ),
    (
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465333333",
        "Pirate #333",
    ),
    (
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465343434",
        "Pirate #434",
    ),
    (
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465353535",
        "Pirate #535",
    ),
    (
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465363636",
        "Pirate #636",
    ),
];

#[component]
pub fn AssetGridStory() -> impl IntoView {
    // Signals for interactive demos
    let (loading, set_loading) = signal(false);
    let (show_assets, set_show_assets) = signal(true);

    // Derive is_empty from show_assets
    let is_empty = Signal::derive(move || !show_assets.get());

    view! {
        <div>
            <div class="story-header">
                <h2>"Asset Grid"</h2>
                <p>"A responsive grid layout for displaying asset cards. Supports loading states, empty states, and flexible content including mixed card types."</p>
            </div>

            // Basic Usage
            <div class="story-section">
                <h3>"Basic Usage"</h3>
                <p class="story-description">"Grid with AssetCard components. Auto-fits columns based on available width."</p>
                <div class="story-canvas">
                    <AssetGrid is_empty=Signal::derive(|| false)>
                        {SAMPLE_ASSETS.iter().take(4).map(|(id, name)| {
                            view! {
                                <AssetCard
                                    asset_id=id.to_string()
                                    name=name.to_string()
                                    size=CardSize::Sm
                                    show_name=true
                                />
                            }
                        }).collect_view()}
                    </AssetGrid>
                </div>
            </div>

            // Mixed Content
            <div class="story-section">
                <h3>"Mixed Content (AssetCard + ImageCard)"</h3>
                <p class="story-description">"The grid accepts any children - you can mix AssetCard, ImageCard, or custom components."</p>
                <div class="story-canvas">
                    <AssetGrid
                        is_empty=Signal::derive(|| false)
                        min_column_width="150px"
                    >
                        // AssetCard with IIIF URL
                        <AssetCard
                            asset_id=SAMPLE_ASSETS[0].0.to_string()
                            name="NFT Asset"
                            size=CardSize::Sm
                            show_name=true
                        />
                        // ImageCard with direct URL
                        <ImageCard
                            image_url="https://picsum.photos/seed/grid1/200"
                            name="Direct Image"
                            size=CardSize::Sm
                            show_name=true
                        />
                        // Another AssetCard
                        <AssetCard
                            asset_id=SAMPLE_ASSETS[1].0.to_string()
                            name="Another NFT"
                            size=CardSize::Sm
                            show_name=true
                        />
                        // ImageCard placeholder
                        <ImageCard
                            image_url="https://picsum.photos/seed/grid2/200"
                            name="Placeholder"
                            size=CardSize::Sm
                            show_name=true
                        />
                    </AssetGrid>
                </div>
            </div>

            // Custom Content
            <div class="story-section">
                <h3>"Custom Card Content"</h3>
                <p class="story-description">"Grid items can be any view - here we show custom cards with overlays."</p>
                <div class="story-canvas">
                    <AssetGrid
                        is_empty=Signal::derive(|| false)
                        gap="1.5rem"
                        min_column_width="160px"
                    >
                        {SAMPLE_ASSETS.iter().take(3).enumerate().map(|(i, (id, name))| {
                            let power = 200 + (i as u32 * 50);
                            view! {
                                <div class="custom-grid-card">
                                    <AssetCard
                                        asset_id=id.to_string()
                                        name=name.to_string()
                                        size=CardSize::Sm
                                        show_name=true
                                    />
                                    <div class="custom-grid-card__overlay">
                                        <StatPill value=power.to_string() icon="⚡" />
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </AssetGrid>
                </div>
            </div>

            // Fixed Columns
            <div class="story-section">
                <h3>"Fixed Columns"</h3>
                <p class="story-description">"Use the `columns` prop for a fixed column count instead of auto-fit."</p>
                <div class="story-canvas">
                    <AssetGrid
                        is_empty=Signal::derive(|| false)
                        columns=3
                        gap="0.5rem"
                    >
                        {SAMPLE_ASSETS.iter().map(|(id, name)| {
                            view! {
                                <AssetCard
                                    asset_id=id.to_string()
                                    name=name.to_string()
                                    size=CardSize::Xs
                                    show_name=true
                                />
                            }
                        }).collect_view()}
                    </AssetGrid>
                </div>
            </div>

            // Loading & Empty States
            <div class="story-section">
                <h3>"Loading & Empty States"</h3>
                <p class="story-description">"Interactive demo - toggle loading and empty states."</p>
                <div class="story-canvas">
                    <div style="margin-bottom: 1rem; display: flex; gap: 1rem;">
                        <button
                            class="btn btn-secondary"
                            on:click=move |_| set_loading.update(|l| *l = !*l)
                        >
                            {move || if loading.get() { "Stop Loading" } else { "Start Loading" }}
                        </button>
                        <button
                            class="btn btn-secondary"
                            on:click=move |_| set_show_assets.update(|s| *s = !*s)
                        >
                            {move || if show_assets.get() { "Clear Assets" } else { "Show Assets" }}
                        </button>
                    </div>

                    <AssetGrid
                        loading=Signal::derive(move || loading.get())
                        is_empty=is_empty
                        empty_message="No pirates in your crew yet"
                    >
                        {SAMPLE_ASSETS.iter().take(4).map(|(id, name)| {
                            view! {
                                <AssetCard
                                    asset_id=id.to_string()
                                    name=name.to_string()
                                    size=CardSize::Sm
                                    show_name=true
                                />
                            }
                        }).collect_view()}
                    </AssetGrid>
                </div>
            </div>

            // Empty State
            <div class="story-section">
                <h3>"Empty State"</h3>
                <p class="story-description">"When `is_empty` is true, displays the empty message."</p>
                <div class="story-canvas">
                    <AssetGrid
                        is_empty=Signal::derive(|| true)
                        empty_message="No assets found matching your criteria"
                    >
                        // Children exist but won't render due to is_empty=true
                        <div>"This won't show"</div>
                    </AssetGrid>
                </div>
            </div>

            // Props Reference
            <div class="story-section">
                <h3>"Props"</h3>
                <div class="story-canvas">
                    <div class="story-grid">
                        <AttributeCard
                            name="children"
                            values="Children"
                            description="Grid content - any Leptos views (AssetCard, ImageCard, custom divs, etc.)"
                        />
                        <AttributeCard
                            name="is_empty"
                            values="Signal<bool>"
                            description="Signal indicating if grid is empty. When true, shows empty_message instead of children."
                        />
                        <AttributeCard
                            name="empty_message"
                            values="String (default: \"No assets\")"
                            description="Message displayed when is_empty is true."
                        />
                        <AttributeCard
                            name="loading"
                            values="Signal<bool>"
                            description="When true, shows a loading spinner instead of content."
                        />
                        <AttributeCard
                            name="min_column_width"
                            values="String (default: \"120px\")"
                            description="Minimum column width for auto-fit. CSS length value."
                        />
                        <AttributeCard
                            name="gap"
                            values="String (default: \"1rem\")"
                            description="Gap between grid items. CSS length value."
                        />
                        <AttributeCard
                            name="columns"
                            values="Option<u32>"
                            description="Fixed number of columns. Overrides auto-fit when set."
                        />
                        <AttributeCard
                            name="class"
                            values="String"
                            description="Additional CSS class for the grid wrapper."
                        />
                    </div>
                </div>
            </div>

            // Usage Examples
            <div class="story-section">
                <h3>"Usage"</h3>
                <pre class="code-block">{r##"use ui_components::{AssetGrid, AssetCard, ImageCard};

// Basic grid with AssetCards
let assets = signal(vec![...]);
let is_empty = Signal::derive(move || assets.get().is_empty());

view! {
    <AssetGrid is_empty=is_empty>
        <For each=move || assets.get() key=|a| a.id.clone() let:asset>
            <AssetCard asset_id=asset.id name=asset.name />
        </For>
    </AssetGrid>
}

// Mixed content - AssetCard and ImageCard together
view! {
    <AssetGrid is_empty=Signal::derive(|| false)>
        <AssetCard asset_id="..." name="NFT" />
        <ImageCard image_url="https://..." name="Regular Image" />
        <AssetCard asset_id="..." name="Another NFT" />
    </AssetGrid>
}

// With loading state
view! {
    <AssetGrid
        loading=is_loading
        is_empty=is_empty
        empty_message="No crew members found"
    >
        {children}
    </AssetGrid>
}

// Fixed 4-column layout
view! {
    <AssetGrid columns=4 gap="0.5rem" is_empty=is_empty>
        {children}
    </AssetGrid>
}"##}</pre>
            </div>

            // Inline styles for custom card overlay demo
            <style>{r#"
                .custom-grid-card {
                    position: relative;
                }
                .custom-grid-card__overlay {
                    position: absolute;
                    top: 0.25rem;
                    right: 0.25rem;
                }
            "#}</style>
        </div>
    }
}
