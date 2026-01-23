//! Shared UI Storybook
//!
//! Development-only showcase for shared-ui components using Leptos.

mod stories;

use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// ============================================================================
// Story Enum
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Story {
    Welcome,
    // Components
    ImageCardComponent,
    AssetCardComponent,
    AssetCacheComponent,
    ConnectionStatusComponent,
    MemoryCardComponent,
    // Wallet Core
    WalletProviders,
    ConnectionStates,
    // UI Flow
    FlowOverview,
    FlowState,
    FlowOperations,
    // UI Loader
    LoadingStates,
    LoaderConfig,
    // UI Toast
    ToastTypes,
    ToastUsage,
}

impl Story {
    fn all() -> &'static [Story] {
        &[
            Story::Welcome,
            Story::ImageCardComponent,
            Story::AssetCardComponent,
            Story::AssetCacheComponent,
            Story::ConnectionStatusComponent,
            Story::MemoryCardComponent,
            Story::WalletProviders,
            Story::ConnectionStates,
            Story::FlowOverview,
            Story::FlowState,
            Story::FlowOperations,
            Story::LoadingStates,
            Story::LoaderConfig,
            Story::ToastTypes,
            Story::ToastUsage,
        ]
    }

    fn label(&self) -> &'static str {
        match self {
            Story::Welcome => "Welcome",
            Story::ImageCardComponent => "Image Card",
            Story::AssetCardComponent => "Asset Card",
            Story::AssetCacheComponent => "Asset Cache",
            Story::ConnectionStatusComponent => "Connection Status",
            Story::MemoryCardComponent => "Memory Card",
            Story::WalletProviders => "Wallet Providers",
            Story::ConnectionStates => "Connection States",
            Story::FlowOverview => "Overview",
            Story::FlowState => "FlowState Trait",
            Story::FlowOperations => "Operations",
            Story::LoadingStates => "Loading States",
            Story::LoaderConfig => "Loader Config",
            Story::ToastTypes => "Toast Types",
            Story::ToastUsage => "Toast Usage",
        }
    }

    fn category(&self) -> &'static str {
        match self {
            Story::Welcome => "Getting Started",
            Story::ImageCardComponent
            | Story::AssetCardComponent
            | Story::AssetCacheComponent
            | Story::ConnectionStatusComponent
            | Story::MemoryCardComponent => "Components",
            Story::WalletProviders | Story::ConnectionStates => "Wallet Core",
            Story::FlowOverview | Story::FlowState | Story::FlowOperations => "UI Flow",
            Story::LoadingStates | Story::LoaderConfig => "UI Loader",
            Story::ToastTypes | Story::ToastUsage => "UI Toast",
        }
    }
}

// ============================================================================
// Main Entry
// ============================================================================

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    // Initialize tracing to browser console
    tracing_wasm::set_as_global_default();

    // Mount to #app element (not body) to work with the index.html structure
    let app_element = document()
        .get_element_by_id("app")
        .expect("should find #app element")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("#app should be an HtmlElement");
    mount_to(app_element, App);
}

#[component]
fn App() -> impl IntoView {
    let (current_story, set_current_story) = create_signal(Story::Welcome);

    view! {
        // Include ui-components styles
        <style>{ui_components::STYLES}</style>
        <div class="storybook">
            <Sidebar current_story=current_story set_current_story=set_current_story />
            <main class="storybook-main">
                <StoryContent story=current_story />
            </main>
        </div>
    }
}

#[component]
fn Sidebar(
    current_story: ReadSignal<Story>,
    set_current_story: WriteSignal<Story>,
) -> impl IntoView {
    // Group stories by category
    let categories: Vec<(&'static str, Vec<Story>)> = {
        let mut cats: Vec<(&str, Vec<Story>)> = Vec::new();
        for story in Story::all() {
            let cat = story.category();
            if let Some((_, stories)) = cats.iter_mut().find(|(c, _)| *c == cat) {
                stories.push(*story);
            } else {
                cats.push((cat, vec![*story]));
            }
        }
        cats
    };

    view! {
        <aside class="storybook-sidebar">
            <h1>"Shared UI"</h1>
            {categories.into_iter().map(|(category, stories)| {
                view! {
                    <div class="nav-section">
                        <h2>{category}</h2>
                        <ul>
                            {stories.into_iter().map(|story| {
                                let is_active = move || current_story.get() == story;
                                view! {
                                    <li>
                                        <a
                                            href="#"
                                            class:active=is_active
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                tracing::info!("Clicked story: {:?}", story);
                                                set_current_story.set(story);
                                            }
                                        >
                                            {story.label()}
                                        </a>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    </div>
                }
            }).collect_view()}
        </aside>
    }
}

#[component]
fn StoryContent(story: ReadSignal<Story>) -> impl IntoView {
    view! {
        <Show when=move || story.get() == Story::Welcome fallback=|| ()>
            <stories::WelcomeStory />
        </Show>
        <Show when=move || story.get() == Story::ImageCardComponent fallback=|| ()>
            <stories::ImageCardStory />
        </Show>
        <Show when=move || story.get() == Story::AssetCardComponent fallback=|| ()>
            <stories::AssetCardStory />
        </Show>
        <Show when=move || story.get() == Story::AssetCacheComponent fallback=|| ()>
            <stories::AssetCacheStory />
        </Show>
        <Show when=move || story.get() == Story::ConnectionStatusComponent fallback=|| ()>
            <stories::ConnectionStatusStory />
        </Show>
        <Show when=move || story.get() == Story::MemoryCardComponent fallback=|| ()>
            <stories::MemoryCardStory />
        </Show>
        <Show when=move || story.get() == Story::WalletProviders fallback=|| ()>
            <stories::WalletProvidersStory />
        </Show>
        <Show when=move || story.get() == Story::ConnectionStates fallback=|| ()>
            <stories::ConnectionStatesStory />
        </Show>
        <Show when=move || story.get() == Story::FlowOverview fallback=|| ()>
            <stories::FlowOverviewStory />
        </Show>
        <Show when=move || story.get() == Story::FlowState fallback=|| ()>
            <stories::FlowStateStory />
        </Show>
        <Show when=move || story.get() == Story::FlowOperations fallback=|| ()>
            <stories::FlowOperationsStory />
        </Show>
        <Show when=move || story.get() == Story::LoadingStates fallback=|| ()>
            <stories::LoadingStatesStory />
        </Show>
        <Show when=move || story.get() == Story::LoaderConfig fallback=|| ()>
            <stories::LoaderConfigStory />
        </Show>
        <Show when=move || story.get() == Story::ToastTypes fallback=|| ()>
            <stories::ToastTypesStory />
        </Show>
        <Show when=move || story.get() == Story::ToastUsage fallback=|| ()>
            <stories::ToastUsageStory />
        </Show>
    }
}
