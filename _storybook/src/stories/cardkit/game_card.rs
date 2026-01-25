//! GameCard component story

use crate::stories::helpers::AttributeCard;
use leptos::prelude::*;
use ui_cardkit::{CardSize, GameCard};

#[component]
pub fn GameCardStory() -> impl IntoView {
    let (selected_index, set_selected_index) = signal(Option::<usize>::None);

    view! {
        <div>
            <div class="story-header">
                <h2>"GameCard"</h2>
                <p>"Full-sized card component for hand display and detail views. Provides consistent framing with visual states for selection, highlighting, and disabled states."</p>
            </div>

            // Size Variants section
            <div class="story-section">
                <h3>"Size Variants"</h3>
                <div class="story-canvas" style="background: #0d0d1a; padding: 2rem;">
                    <div style="display: flex; align-items: flex-end; gap: 1rem; flex-wrap: wrap;">
                        <div style="text-align: center;">
                            <GameCard size=CardSize::Sm>
                                <div style="width: 100%; height: 100%; background: linear-gradient(135deg, #1a1a2e, #2a2a4e); display: flex; align-items: center; justify-content: center; color: #888;">
                                    "Sm"
                                </div>
                            </GameCard>
                            <div style="color: #888; font-size: 12px; margin-top: 4px;">"100px"</div>
                        </div>
                        <div style="text-align: center;">
                            <GameCard size=CardSize::Md>
                                <div style="width: 100%; height: 100%; background: linear-gradient(135deg, #1a1a2e, #2a2a4e); display: flex; align-items: center; justify-content: center; color: #888;">
                                    "Md"
                                </div>
                            </GameCard>
                            <div style="color: #888; font-size: 12px; margin-top: 4px;">"140px"</div>
                        </div>
                        <div style="text-align: center;">
                            <GameCard size=CardSize::Lg>
                                <div style="width: 100%; height: 100%; background: linear-gradient(135deg, #1a1a2e, #2a2a4e); display: flex; align-items: center; justify-content: center; color: #888;">
                                    "Lg"
                                </div>
                            </GameCard>
                            <div style="color: #888; font-size: 12px; margin-top: 4px;">"200px"</div>
                        </div>
                    </div>
                </div>
            </div>

            // States section
            <div class="story-section">
                <h3>"Visual States"</h3>
                <div class="story-canvas" style="background: #0d0d1a; padding: 2rem;">
                    <div style="display: flex; gap: 1.5rem; flex-wrap: wrap;">
                        <div style="text-align: center;">
                            <GameCard>
                                <div style="width: 100%; height: 100%; background: linear-gradient(135deg, #1a1a2e, #2a2a4e); display: flex; align-items: center; justify-content: center; color: #e0e0e0;">
                                    "Normal"
                                </div>
                            </GameCard>
                            <div style="color: #888; font-size: 12px; margin-top: 4px;">"Default state"</div>
                        </div>
                        <div style="text-align: center;">
                            <GameCard highlighted=Signal::derive(|| true)>
                                <div style="width: 100%; height: 100%; background: linear-gradient(135deg, #1a1a2e, #2a2a4e); display: flex; align-items: center; justify-content: center; color: #ffd700;">
                                    "Highlighted"
                                </div>
                            </GameCard>
                            <div style="color: #888; font-size: 12px; margin-top: 4px;">"Selected/active"</div>
                        </div>
                        <div style="text-align: center;">
                            <GameCard disabled=Signal::derive(|| true)>
                                <div style="width: 100%; height: 100%; background: linear-gradient(135deg, #1a1a2e, #2a2a4e); display: flex; align-items: center; justify-content: center; color: #888;">
                                    "Disabled"
                                </div>
                            </GameCard>
                            <div style="color: #888; font-size: 12px; margin-top: 4px;">"Can't interact"</div>
                        </div>
                    </div>
                </div>
            </div>

            // Interactive Demo
            <div class="story-section">
                <h3>"Interactive Demo"</h3>
                <p class="story-description">"Click cards to select them. Only one card can be selected at a time."</p>
                <div class="story-canvas" style="background: #0d0d1a; padding: 2rem;">
                    <div style="display: flex; gap: 1rem; flex-wrap: wrap;">
                        {(0..4).map(|i| {
                            let is_selected = Signal::derive(move || selected_index.get() == Some(i));
                            view! {
                                <GameCard
                                    highlighted=is_selected
                                    on_click=move |_| {
                                        set_selected_index.set(if selected_index.get() == Some(i) {
                                            None
                                        } else {
                                            Some(i)
                                        });
                                    }
                                >
                                    <div style="width: 100%; height: 100%; background: linear-gradient(135deg, #1a1a2e, #2a2a4e); display: flex; align-items: center; justify-content: center; color: #e0e0e0;">
                                        {format!("Card {}", i + 1)}
                                    </div>
                                </GameCard>
                            }
                        }).collect_view()}
                    </div>
                    <div style="margin-top: 1rem; color: #888;">
                        {move || match selected_index.get() {
                            Some(i) => format!("Selected: Card {}", i + 1),
                            None => "Click a card to select it".to_string(),
                        }}
                    </div>
                </div>
            </div>

            // Props section
            <div class="story-section">
                <h3>"Props"</h3>
                <div class="story-canvas">
                    <div class="story-grid">
                        <AttributeCard
                            name="size"
                            values="CardSize (Sm|Md|Lg|Xl)"
                            description="Card dimensions: 100px, 140px (default), 200px, 280px. Height is 1.4x width."
                        />
                        <AttributeCard
                            name="highlighted"
                            values="Signal<bool>"
                            description="Adds golden glow effect for selected/active state"
                        />
                        <AttributeCard
                            name="disabled"
                            values="Signal<bool>"
                            description="Greys out card and prevents interaction"
                        />
                        <AttributeCard
                            name="on_click"
                            values="Callback<()>"
                            description="Called when card is clicked (unless disabled)"
                        />
                        <AttributeCard
                            name="children"
                            values="Children"
                            description="Card face content - typically an image with overlays"
                        />
                    </div>
                </div>
            </div>

            // Code example
            <div class="story-section">
                <h3>"Usage"</h3>
                <pre class="code-block">{r##"use ui_cardkit::{GameCard, CardSize};

// Basic card
view! {
    <GameCard size=CardSize::Md>
        <img src="card-art.png" />
    </GameCard>
}

// Selectable card
let (selected, set_selected) = signal(false);
view! {
    <GameCard
        highlighted=selected
        on_click=move |_| set_selected.update(|s| *s = !*s)
    >
        <CardContent />
    </GameCard>
}"##}</pre>
            </div>
        </div>
    }
}
