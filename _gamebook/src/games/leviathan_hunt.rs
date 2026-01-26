//! Leviathan Hunt - Game Prototype
//!
//! Hunt the Leviathan with your fleet of engines.

use leptos::prelude::*;
use ui_cardkit::{HealthBar, MonsterStage};

/// Card rarity for styling
#[derive(Clone, Copy)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl Rarity {
    fn class(&self) -> &'static str {
        match self {
            Rarity::Common => "common",
            Rarity::Uncommon => "uncommon",
            Rarity::Rare => "rare",
            Rarity::Epic => "epic",
        }
    }
}

/// A deployed card in the play area
#[derive(Clone)]
struct DeployedCard {
    name: &'static str,
    power: u32,
    charges: (u32, u32), // current / max
    rarity: Rarity,
    image: &'static str,
}

/// A card in the player's hand
#[derive(Clone)]
struct HandCard {
    name: &'static str,
    power: u32,
    cost: u32,
    description: &'static str,
    rarity: Rarity,
    image: &'static str,
}

/// Combat log entry
#[derive(Clone)]
struct LogEntry {
    title: &'static str,
    subtitle: &'static str,
    time: &'static str,
    image: Option<&'static str>,
}

/// Leviathan Hunt game prototype
#[component]
pub fn LeviathanHunt() -> impl IntoView {
    // Game state
    let (monster_health, set_monster_health) = signal(42u32);
    let monster_max_health = 60u32;
    let (currency, _set_currency) = signal(1200u32);

    // Selected card in hand
    let (selected_index, set_selected_index) = signal(Option::<usize>::None);

    // Bottom sheet state
    let (sheet_open, set_sheet_open) = signal(false);

    // Collapsible sections (mobile)
    let (engines_collapsed, set_engines_collapsed) = signal(true);
    let (hand_collapsed, _set_hand_collapsed) = signal(false);

    // Demo deployed cards
    let deployed_cards = vec![
        DeployedCard {
            name: "Rusted Harpoon",
            power: 2,
            charges: (7, 7),
            rarity: Rarity::Common,
            image: "https://images.unsplash.com/photo-1590479773265-7464e5d48118?w=300",
        },
        DeployedCard {
            name: "Clockwork Ram",
            power: 4,
            charges: (3, 3),
            rarity: Rarity::Uncommon,
            image: "https://images.unsplash.com/photo-1518709268805-4e9042af9f23?w=300",
        },
        DeployedCard {
            name: "Cursed Net Array",
            power: 1,
            charges: (2, 3),
            rarity: Rarity::Rare,
            image: "https://images.unsplash.com/photo-1534670007418-fbb7f6cf32c3?w=300",
        },
        DeployedCard {
            name: "Depth Charge Buoy",
            power: 5,
            charges: (3, 3),
            rarity: Rarity::Epic,
            image: "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=300",
        },
    ];

    // Demo hand cards
    let hand_cards = vec![
        HandCard {
            name: "Cut the Line",
            power: 3,
            cost: 3,
            description: "Destroy target Harpoon",
            rarity: Rarity::Common,
            image: "https://images.unsplash.com/photo-1590479773265-7464e5d48118?w=300",
        },
        HandCard {
            name: "Overclock",
            power: 3,
            cost: 3,
            description: "Double this engine's damage. Destroy it after.",
            rarity: Rarity::Common,
            image: "https://images.unsplash.com/photo-1518709268805-4e9042af9f23?w=300",
        },
        HandCard {
            name: "Aim for Pulley's",
            power: 3,
            cost: 3,
            description: "Target Leviathan's Remaining Tentacle. Double damage.",
            rarity: Rarity::Rare,
            image: "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=300",
        },
    ];

    // Combat log entries
    let log_entries = vec![
        LogEntry {
            title: "Depth Charge Buoy",
            subtitle: "DAMAGED",
            time: "1 minute ago",
            image: Some("https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=100"),
        },
        LogEntry {
            title: "Rusted Harpoon",
            subtitle: "DAMAGED",
            time: "1 minute ago",
            image: Some("https://images.unsplash.com/photo-1590479773265-7464e5d48118?w=100"),
        },
        LogEntry {
            title: "Leviathan",
            subtitle: "",
            time: "1 minute ago",
            image: None,
        },
    ];

    // Player avatars
    let player_avatars = [
        "https://i.pravatar.cc/100?img=1",
        "https://i.pravatar.cc/100?img=2",
        "https://i.pravatar.cc/100?img=3",
        "https://i.pravatar.cc/100?img=4",
    ];

    // Calculate total auto damage
    let auto_damage: u32 = deployed_cards.iter().map(|c| c.power).sum();

    view! {
        // Monster as background
        <MonsterStage
            monster_image=Signal::derive(|| "https://images.unsplash.com/photo-1518709268805-4e9042af9f23?w=1600".to_string())
            monster_name=Signal::derive(|| "Leviathan".to_string())
            health_percent=Signal::derive(move || monster_health.get() as f32 / monster_max_health as f32)
        />

        // Game UI overlay
        <div class="game-ui">
            // Top bar
            <div class="top-bar">
                // Left: Your avatar (mobile) or currency (desktop)
                <div class="currency">
                    <div class="currency__icon"></div>
                    <span class="currency__value">{move || currency.get()}</span>
                </div>
                // Mobile: show your avatar on left
                <div class="player-avatar player-avatar--you" style="display: none;">
                    <img src=player_avatars[0] alt="You" />
                </div>

                <div class="player-bar">
                    {player_avatars.iter().enumerate().map(|(i, url)| {
                        let class = if i == 0 { "player-avatar player-avatar--you" } else { "player-avatar" };
                        view! {
                            <div class=class>
                                <img src=*url alt="Player" />
                            </div>
                        }
                    }).collect_view()}
                    <div class="player-stat">
                        <span class="player-stat__value">"2/3"</span>
                    </div>
                    <div class="player-stat">
                        <span class="player-stat__value">"41"</span>
                    </div>
                </div>
            </div>

            // Header: title + health
            <div class="game-header">
                <h1 class="monster-title">"Leviathan"</h1>
                <div class="health-bar-container">
                    <HealthBar
                        current=monster_health
                        max=Signal::derive(move || monster_max_health)
                        show_value=false
                    />
                    <span class="health-text">
                        {move || format!("HP {} / {}", monster_health.get(), monster_max_health)}
                    </span>
                </div>
            </div>

            // Engines section (collapsible on mobile)
            <div class=move || {
                if engines_collapsed.get() {
                    "collapsible-section collapsible-section--collapsed"
                } else {
                    "collapsible-section"
                }
            }>
                <div class="collapsible-section__header">
                    <span class="collapsible-section__title">"Engines in Play"</span>
                    <span class="collapsible-section__meta">{format!("AUTO: {auto_damage}")}</span>
                </div>
                <div class="collapsible-section__content">
                    <div class="play-area">
                        <div class="deployed-cards">
                            {deployed_cards.iter().map(|card| {
                                let rarity_class = format!("deployed-card deployed-card--{}", card.rarity.class());
                                let charges_text = format!("{}/{}", card.charges.0, card.charges.1);
                                view! {
                                    <div class=rarity_class>
                                        <img class="deployed-card__image" src=card.image alt=card.name />
                                        <span class="deployed-card__power">{card.power}</span>
                                        <div class="deployed-card__footer">
                                            <span class="deployed-card__name">{card.name}</span>
                                            <span class="deployed-card__charges">{charges_text}</span>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
                <button
                    class="collapsible-section__toggle"
                    on:click=move |_| set_engines_collapsed.update(|v| *v = !*v)
                >
                    {move || if engines_collapsed.get() { "More" } else { "Less" }}
                </button>
            </div>

            // State banner
            <div class="state-banner">
                <span class="state-banner__text">"Full hand! Play a card!"</span>
            </div>

            // Hand section (collapsible on mobile)
            <div class=move || {
                if hand_collapsed.get() {
                    "collapsible-section collapsible-section--collapsed"
                } else {
                    "collapsible-section"
                }
            }>
                <div class="collapsible-section__content">
                    <div class="hand-area">
                        <div class="hand-area__cards">
                            {hand_cards.iter().enumerate().map(|(i, card)| {
                                let is_selected = move || selected_index.get() == Some(i);
                                let base_class = format!("hand-card hand-card--{}", card.rarity.class());
                                let class = move || {
                                    if is_selected() {
                                        format!("{} hand-card--selected", base_class)
                                    } else {
                                        base_class.clone()
                                    }
                                };
                                let image = card.image;
                                let name = card.name;
                                let power = card.power;
                                let cost = card.cost;
                                let description = card.description;

                                view! {
                                    <div
                                        class=class
                                        on:click=move |_| {
                                            if selected_index.get() == Some(i) {
                                                set_monster_health.update(|h| *h = h.saturating_sub(power));
                                                set_selected_index.set(None);
                                            } else {
                                                set_selected_index.set(Some(i));
                                            }
                                        }
                                    >
                                        <span class="hand-card__power">{power}</span>
                                        <img class="hand-card__image" src=image alt=name />
                                        <span class="hand-card__cost">{cost}</span>
                                        <div class="hand-card__footer">
                                            <span class="hand-card__name">{name}</span>
                                            <span class="hand-card__desc">{description}</span>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
                <button
                    class="collapsible-section__toggle"
                    on:click=move |_| set_sheet_open.set(true)
                >
                    "More"
                </button>
            </div>
        </div>

        // Bottom sheet (MORE panel)
        <div class=move || {
            if sheet_open.get() {
                "bottom-sheet bottom-sheet--open"
            } else {
                "bottom-sheet"
            }
        }>
            <div
                class="bottom-sheet__backdrop"
                on:click=move |_| set_sheet_open.set(false)
            ></div>
            <div class="bottom-sheet__panel">
                <div class="bottom-sheet__header">
                    <span class="bottom-sheet__title">"More"</span>
                    <button
                        class="bottom-sheet__close"
                        on:click=move |_| set_sheet_open.set(false)
                    >
                        "×"
                    </button>
                </div>
                <div class="bottom-sheet__content">
                    // Stats row
                    <div class="stats-row">
                        <div class="stats-row__item">
                            <div class="currency__icon stats-row__icon"></div>
                            <span class="stats-row__value">{move || format!("{}", currency.get())}</span>
                        </div>
                        <div class="stats-row__item">
                            <span class="stats-row__value">"0"</span>
                        </div>
                        <div class="stats-row__item">
                            <span class="stats-row__value">"17"</span>
                        </div>
                    </div>

                    // Combat log
                    <div class="combat-log">
                        <div class="combat-log__title">
                            "Combat Log"
                        </div>
                        <div class="combat-log__list">
                            {log_entries.iter().map(|entry| {
                                view! {
                                    <div class="log-entry">
                                        <div class="log-entry__icon">
                                            {entry.image.map(|src| view! {
                                                <img src=src alt="" />
                                            })}
                                        </div>
                                        <div class="log-entry__content">
                                            <div class="log-entry__title">{entry.title}</div>
                                            <div class="log-entry__subtitle">{entry.subtitle}</div>
                                        </div>
                                        <div class="log-entry__time">{entry.time}</div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
                <div class="bottom-sheet__footer">
                    <button
                        class="bottom-sheet__close-btn"
                        on:click=move |_| set_sheet_open.set(false)
                    >
                        "Close"
                    </button>
                </div>
            </div>
        </div>
    }
}
