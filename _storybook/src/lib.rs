//! Shared UI Storybook
//!
//! Development-only showcase for shared-ui components using Leptos.

pub mod api;
mod stories;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// ============================================================================
// Story Enum
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Story {
    Welcome,
    // Components - Layout
    CardComponent,
    ModalComponent,
    TabsComponent,
    HeaderComponent,
    // Components - Data Display
    ImageCardComponent,
    AssetCardComponent,
    AssetGridComponent,
    AssetDetailCardComponent,
    AssetCacheComponent,
    ConnectionStatusComponent,
    MemoryCardComponent,
    StatPillComponent,
    BadgeComponent,
    EmptyStateComponent,
    ProgressBarComponent,
    // Components - Forms
    ButtonComponent,
    ButtonGroupComponent,
    SelectComponent,
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
            // Layout
            Story::CardComponent,
            Story::ModalComponent,
            Story::TabsComponent,
            Story::HeaderComponent,
            // Data Display
            Story::ImageCardComponent,
            Story::AssetCardComponent,
            Story::AssetGridComponent,
            Story::AssetDetailCardComponent,
            Story::AssetCacheComponent,
            Story::ConnectionStatusComponent,
            Story::MemoryCardComponent,
            Story::StatPillComponent,
            Story::BadgeComponent,
            Story::EmptyStateComponent,
            Story::ProgressBarComponent,
            // Forms
            Story::ButtonComponent,
            Story::ButtonGroupComponent,
            Story::SelectComponent,
            // Wallet
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
            // Layout
            Story::CardComponent => "Card",
            Story::ModalComponent => "Modal",
            Story::TabsComponent => "Tabs",
            Story::HeaderComponent => "Page Header",
            // Data Display
            Story::ImageCardComponent => "Image Card",
            Story::AssetCardComponent => "Asset Card",
            Story::AssetGridComponent => "Asset Grid",
            Story::AssetDetailCardComponent => "Asset Detail Card",
            Story::AssetCacheComponent => "Asset Cache",
            Story::ConnectionStatusComponent => "Connection Status",
            Story::MemoryCardComponent => "Memory Card",
            Story::StatPillComponent => "Stat Pill",
            Story::BadgeComponent => "Badge",
            Story::EmptyStateComponent => "Empty State",
            Story::ProgressBarComponent => "Progress Bar",
            // Forms
            Story::ButtonComponent => "Button",
            Story::ButtonGroupComponent => "Button Group",
            Story::SelectComponent => "Select",
            // Wallet
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
            // Layout components
            Story::CardComponent
            | Story::ModalComponent
            | Story::TabsComponent
            | Story::HeaderComponent => "Layout",
            // Data display components
            Story::ImageCardComponent
            | Story::AssetCardComponent
            | Story::AssetGridComponent
            | Story::AssetDetailCardComponent
            | Story::AssetCacheComponent
            | Story::ConnectionStatusComponent
            | Story::MemoryCardComponent
            | Story::StatPillComponent
            | Story::BadgeComponent
            | Story::EmptyStateComponent
            | Story::ProgressBarComponent => "Data Display",
            // Form components
            Story::ButtonComponent | Story::ButtonGroupComponent | Story::SelectComponent => {
                "Forms"
            }
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
    mount_to(app_element, App).forget();
}

#[component]
fn App() -> impl IntoView {
    let (current_story, set_current_story) = signal(Story::Welcome);

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
    use leptos::prelude::CollectView;
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
        // Layout
        <Show when=move || story.get() == Story::CardComponent fallback=|| ()>
            <stories::CardStory />
        </Show>
        <Show when=move || story.get() == Story::ModalComponent fallback=|| ()>
            <stories::ModalStory />
        </Show>
        <Show when=move || story.get() == Story::TabsComponent fallback=|| ()>
            <stories::TabsStory />
        </Show>
        <Show when=move || story.get() == Story::HeaderComponent fallback=|| ()>
            <stories::HeaderStory />
        </Show>
        // Data Display
        <Show when=move || story.get() == Story::ImageCardComponent fallback=|| ()>
            <stories::ImageCardStory />
        </Show>
        <Show when=move || story.get() == Story::AssetCardComponent fallback=|| ()>
            <stories::AssetCardStory />
        </Show>
        <Show when=move || story.get() == Story::AssetGridComponent fallback=|| ()>
            <stories::AssetGridStory />
        </Show>
        <Show when=move || story.get() == Story::AssetDetailCardComponent fallback=|| ()>
            <stories::AssetDetailCardStory />
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
        <Show when=move || story.get() == Story::StatPillComponent fallback=|| ()>
            <stories::StatPillStory />
        </Show>
        <Show when=move || story.get() == Story::BadgeComponent fallback=|| ()>
            <stories::BadgeStory />
        </Show>
        <Show when=move || story.get() == Story::EmptyStateComponent fallback=|| ()>
            <stories::EmptyStateStory />
        </Show>
        <Show when=move || story.get() == Story::ProgressBarComponent fallback=|| ()>
            <stories::ProgressBarStory />
        </Show>
        // Forms
        <Show when=move || story.get() == Story::ButtonComponent fallback=|| ()>
            <stories::ButtonStory />
        </Show>
        <Show when=move || story.get() == Story::ButtonGroupComponent fallback=|| ()>
            <stories::ButtonGroupStory />
        </Show>
        <Show when=move || story.get() == Story::SelectComponent fallback=|| ()>
            <stories::SelectStory />
        </Show>
        // Wallet
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
