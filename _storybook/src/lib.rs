//! Shared UI Storybook
//!
//! Development-only showcase for shared-ui components using Leptos.

mod stories;

use leptos::*;
use wasm_bindgen::prelude::*;

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
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (current_story, set_current_story) = create_signal(Story::Welcome);

    view! {
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
                                            class:active=is_active
                                            on:click=move |_| set_current_story.set(story)
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
    move || match story.get() {
        Story::Welcome => stories::WelcomeStory().into_view(),
        Story::ImageCardComponent => stories::ImageCardStory().into_view(),
        Story::AssetCardComponent => stories::AssetCardStory().into_view(),
        Story::AssetCacheComponent => stories::AssetCacheStory().into_view(),
        Story::ConnectionStatusComponent => stories::ConnectionStatusStory().into_view(),
        Story::MemoryCardComponent => stories::MemoryCardStory().into_view(),
        Story::WalletProviders => stories::WalletProvidersStory().into_view(),
        Story::ConnectionStates => stories::ConnectionStatesStory().into_view(),
        Story::FlowOverview => stories::FlowOverviewStory().into_view(),
        Story::FlowState => stories::FlowStateStory().into_view(),
        Story::FlowOperations => stories::FlowOperationsStory().into_view(),
        Story::LoadingStates => stories::LoadingStatesStory().into_view(),
        Story::LoaderConfig => stories::LoaderConfigStory().into_view(),
        Story::ToastTypes => stories::ToastTypesStory().into_view(),
        Story::ToastUsage => stories::ToastUsageStory().into_view(),
    }
}
