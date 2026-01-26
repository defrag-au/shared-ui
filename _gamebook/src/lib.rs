//! Game UI Testing Harness
//!
//! A dedicated environment for testing game feel and interactions.
//! Unlike the storybook (component demos), this provides full-screen
//! immersive game prototypes.

mod games;
mod styles;

use games::leviathan_hunt::LeviathanHunt;
use leptos::prelude::*;
use styles::STYLES;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Available game prototypes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Game {
    LeviathanHunt,
}

impl Game {
    fn all() -> &'static [Game] {
        &[Game::LeviathanHunt]
    }

    fn label(&self) -> &'static str {
        match self {
            Game::LeviathanHunt => "Leviathan Hunt",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Game::LeviathanHunt => "Hunt the Leviathan with your fleet of engines",
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let app_element = document()
        .get_element_by_id("app")
        .expect("should find #app element")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("#app should be an HtmlElement");
    mount_to(app_element, App).forget();
}

#[component]
fn App() -> impl IntoView {
    let (current_game, set_current_game) = signal(Option::<Game>::None);

    view! {
        <style>{ui_components::STYLES}</style>
        <style>{ui_cardkit::STYLES}</style>
        <style>{STYLES}</style>

        <div class="gamebook">
            {move || match current_game.get() {
                None => view! {
                    <GameSelector on_select=Callback::new(move |game| set_current_game.set(Some(game))) />
                }.into_any(),
                Some(game) => view! {
                    <GameContainer game=game />
                }.into_any(),
            }}
        </div>
    }
}

#[component]
fn GameSelector(on_select: Callback<Game>) -> impl IntoView {
    view! {
        <div class="game-selector">
            <h1 class="game-selector__title">"Game Prototypes"</h1>
            <p class="game-selector__subtitle">"Select a game to test"</p>

            <div class="game-selector__grid">
                {Game::all().iter().map(|&game| {
                    view! {
                        <button
                            class="game-selector__card"
                            on:click=move |_| on_select.run(game)
                        >
                            <span class="game-selector__card-title">{game.label()}</span>
                            <span class="game-selector__card-desc">{game.description()}</span>
                        </button>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn GameContainer(game: Game) -> impl IntoView {
    view! {
        <div class="game-container">
            <div class="game-container__content game-container__content--stage">
                {match game {
                    Game::LeviathanHunt => view! { <LeviathanHunt /> }.into_any(),
                }}
            </div>
        </div>
    }
}
