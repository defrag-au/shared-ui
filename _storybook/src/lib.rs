//! Shared UI Storybook
//!
//! Development-only showcase for shared-ui components and primitives.

mod stories;

use futures_signals::signal::{Mutable, SignalExt};
use wasm_bindgen::prelude::*;

use primitives::{bind_class, create_element, document, on_click, AppendChild, ClearChildren};

// ============================================================================
// Story Enum
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Story {
    Welcome,
    // Primitives
    ReactiveText,
    ReactiveBindings,
    DomHelpers,
    EventHandlers,
    // Components
    ImageCardComponent,
    AssetCardComponent,
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
            Story::ReactiveText,
            Story::ReactiveBindings,
            Story::DomHelpers,
            Story::EventHandlers,
            Story::ImageCardComponent,
            Story::AssetCardComponent,
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
            Story::ReactiveText => "Reactive Text",
            Story::ReactiveBindings => "Reactive Bindings",
            Story::DomHelpers => "DOM Helpers",
            Story::EventHandlers => "Event Handlers",
            Story::ImageCardComponent => "Image Card",
            Story::AssetCardComponent => "Asset Card",
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
            Story::ReactiveText
            | Story::ReactiveBindings
            | Story::DomHelpers
            | Story::EventHandlers => "Primitives",
            Story::ImageCardComponent
            | Story::AssetCardComponent
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
// App State
// ============================================================================

struct App {
    current_story: Mutable<Story>,
}

impl App {
    fn new() -> Self {
        Self {
            current_story: Mutable::new(Story::Welcome),
        }
    }
}

// ============================================================================
// Main Entry
// ============================================================================

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let app = App::new();
    render_app(&app);
}

fn render_app(app: &App) {
    let root = document().get_element_by_id("app").unwrap();
    root.clear_children();

    let container = create_element("div", &["storybook"]);

    // Sidebar
    let sidebar = render_sidebar(app);
    container.append(&sidebar);

    // Main content area
    let main = create_element("main", &["storybook-main"]);
    main.set_attribute("id", "story-content").unwrap();
    container.append(&main);

    root.append(&container);

    // Initial render
    render_story(app.current_story.get());
}

fn render_sidebar(app: &App) -> web_sys::Element {
    let sidebar = create_element("aside", &["storybook-sidebar"]);

    let title = create_element("h1", &[]);
    title.set_text_content(Some("Shared UI"));
    sidebar.append(&title);

    // Group stories by category
    let mut current_category = "";

    for story in Story::all() {
        if story.category() != current_category {
            current_category = story.category();

            let section = create_element("div", &["nav-section"]);
            let heading = create_element("h2", &[]);
            heading.set_text_content(Some(current_category));
            section.append(&heading);

            let list = create_element("ul", &[]);
            section.append(&list);
            sidebar.append(&section);
        }

        // Find the last ul in sidebar
        let lists = sidebar.query_selector_all("ul").unwrap();
        if let Some(list) = lists.item(lists.length() - 1) {
            let li = create_element("li", &[]);
            let link = create_element("a", &[]);
            link.set_text_content(Some(story.label()));

            // Highlight active story
            let story_value = *story;
            bind_class(
                &link,
                "active",
                app.current_story
                    .signal()
                    .map(move |current| current == story_value),
            );

            // Click handler
            let current_story = app.current_story.clone();
            on_click(&link, move |_| {
                current_story.set(story_value);
                render_story(story_value);
            });

            li.append(&link);
            list.append_child(&li).unwrap();
        }
    }

    sidebar
}

fn render_story(story: Story) {
    let main = document().get_element_by_id("story-content").unwrap();
    main.clear_children();

    let content = match story {
        Story::Welcome => stories::render_welcome(),
        Story::ReactiveText => stories::render_reactive_text_story(),
        Story::ReactiveBindings => stories::render_reactive_bindings_story(),
        Story::DomHelpers => stories::render_dom_helpers_story(),
        Story::EventHandlers => stories::render_event_handlers_story(),
        Story::ImageCardComponent => stories::render_image_card_story(),
        Story::AssetCardComponent => stories::render_asset_card_story(),
        Story::ConnectionStatusComponent => stories::render_connection_status_story(),
        Story::MemoryCardComponent => stories::render_memory_card_story(),
        Story::WalletProviders => stories::render_wallet_providers_story(),
        Story::ConnectionStates => stories::render_connection_states_story(),
        Story::FlowOverview => stories::render_flow_overview_story(),
        Story::FlowState => stories::render_flow_state_story(),
        Story::FlowOperations => stories::render_flow_operations_story(),
        Story::LoadingStates => stories::render_loading_states_story(),
        Story::LoaderConfig => stories::render_loader_config_story(),
        Story::ToastTypes => stories::render_toast_types_story(),
        Story::ToastUsage => stories::render_toast_usage_story(),
    };

    main.append(&content);
}
