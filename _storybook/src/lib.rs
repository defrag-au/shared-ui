//! Shared UI Storybook
//!
//! Development-only showcase for shared-ui components and primitives.

use futures_signals::signal::{Mutable, SignalExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, HtmlInputElement};

use primitives::{
    bind_class, bind_text_content, bind_visible, create_button, create_element,
    create_html_element, create_input, document, on_click, on_input, text_signal, AppendChild,
    ClearChildren, SetAttr,
};
use wallet_core::{ConnectionState, Network, WalletProvider};

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
    // Wallet Core
    WalletProviders,
    ConnectionStates,
}

impl Story {
    fn all() -> &'static [Story] {
        &[
            Story::Welcome,
            Story::ReactiveText,
            Story::ReactiveBindings,
            Story::DomHelpers,
            Story::EventHandlers,
            Story::WalletProviders,
            Story::ConnectionStates,
        ]
    }

    fn label(&self) -> &'static str {
        match self {
            Story::Welcome => "Welcome",
            Story::ReactiveText => "Reactive Text",
            Story::ReactiveBindings => "Reactive Bindings",
            Story::DomHelpers => "DOM Helpers",
            Story::EventHandlers => "Event Handlers",
            Story::WalletProviders => "Wallet Providers",
            Story::ConnectionStates => "Connection States",
        }
    }

    fn category(&self) -> &'static str {
        match self {
            Story::Welcome => "Getting Started",
            Story::ReactiveText
            | Story::ReactiveBindings
            | Story::DomHelpers
            | Story::EventHandlers => "Primitives",
            Story::WalletProviders | Story::ConnectionStates => "Wallet Core",
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
    main.attr("id", "story-content");
    container.append(&main);

    root.append(&container);

    // Initial render
    render_story(app.current_story.get());
}

fn render_sidebar(app: &App) -> Element {
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
        Story::Welcome => render_welcome(),
        Story::ReactiveText => render_reactive_text_story(),
        Story::ReactiveBindings => render_reactive_bindings_story(),
        Story::DomHelpers => render_dom_helpers_story(),
        Story::EventHandlers => render_event_handlers_story(),
        Story::WalletProviders => render_wallet_providers_story(),
        Story::ConnectionStates => render_connection_states_story(),
    };

    main.append(&content);
}

// ============================================================================
// Welcome Story
// ============================================================================

fn render_welcome() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Welcome to Shared UI"));
    header.append(&h2);

    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "A collection of reusable Rust/WASM UI primitives and components for Cardano applications.",
    ));
    header.append(&desc);
    container.append(&header);

    // Features section
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Included Crates"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &["story-grid"]);

    // Primitives card
    let card1 = render_feature_card(
        "primitives",
        "Reactive DOM bindings, element helpers, and event utilities using futures-signals.",
    );
    grid.append(&card1);

    // Wallet Core card
    let card2 = render_feature_card(
        "wallet-core",
        "CIP-30 wallet integration for Cardano wallets (Nami, Eternl, Lace, etc.).",
    );
    grid.append(&card2);

    // Components card
    let card3 = render_feature_card(
        "components",
        "Reusable web components built on primitives (coming soon).",
    );
    grid.append(&card3);

    canvas.append(&grid);
    section.append(&canvas);
    container.append(&section);

    container
}

fn render_feature_card(title: &str, description: &str) -> Element {
    let card = create_element("div", &["wallet-card"]);

    let header = create_element("div", &["wallet-card__header"]);
    let name = create_element("span", &["wallet-card__name"]);
    name.set_text_content(Some(title));
    header.append(&name);
    card.append(&header);

    let body = create_element("div", &["wallet-card__body"]);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(description));
    body.append(&desc);
    card.append(&body);

    card
}

// ============================================================================
// Reactive Text Story
// ============================================================================

fn render_reactive_text_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Reactive Text"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Create text nodes that automatically update when their underlying signal changes.",
    ));
    header.append(&desc);
    container.append(&header);

    // Demo section
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Live Demo"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let demo = create_element("div", &["signal-demo"]);

    // Create reactive state
    let name = Mutable::new("World".to_string());

    // Input
    let input: HtmlInputElement = create_input("text", &["demo-input"]);
    input.set_value("World");
    input.set_placeholder("Enter a name...");

    let name_clone = name.clone();
    on_input(&input, move |_e| {
        let input: HtmlInputElement = document()
            .query_selector(".demo-input")
            .unwrap()
            .unwrap()
            .unchecked_into();
        name_clone.set(input.value());
    });
    demo.append(&input);

    // Output with reactive text
    let output = create_element("div", &["signal-demo__output"]);
    output.append_text("Hello, ");
    let text_node = text_signal(name.signal_cloned());
    output.append_child(&text_node).unwrap();
    output.append_text("!");
    demo.append(&output);

    canvas.append(&demo);
    section.append(&canvas);
    container.append(&section);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Usage"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r#"use futures_signals::signal::Mutable;
use primitives::text_signal;

let name = Mutable::new("World".to_string());

// Create a text node bound to the signal
let text_node = text_signal(name.signal_cloned());
parent.append_child(&text_node);

// Updates automatically when signal changes
name.set("Rust".to_string());"#,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}

// ============================================================================
// Reactive Bindings Story
// ============================================================================

fn render_reactive_bindings_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Reactive Bindings"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Bind signals to element attributes, classes, and visibility.",
    ));
    header.append(&desc);
    container.append(&header);

    // Class binding demo
    let section1 = create_element("div", &["story-section"]);
    let h3_1 = create_element("h3", &[]);
    h3_1.set_text_content(Some("Class Binding"));
    section1.append(&h3_1);

    let canvas1 = create_element("div", &["story-canvas"]);
    let inline1 = create_element("div", &["story-inline"]);

    let is_active = Mutable::new(false);

    let toggle_btn = create_button("Toggle Active", &["demo-btn", "demo-btn--primary"]);
    let is_active_clone = is_active.clone();
    on_click(&toggle_btn.clone().into(), move |_| {
        is_active_clone.replace_with(|v| !*v);
    });
    inline1.append(&toggle_btn);

    let status = create_element(
        "span",
        &["status-indicator", "status-indicator--disconnected"],
    );
    status.set_text_content(Some("Inactive"));

    // Bind the class
    bind_class(&status, "status-indicator--connected", is_active.signal());
    bind_class(
        &status,
        "status-indicator--disconnected",
        is_active.signal().map(|v| !v),
    );

    // Bind text content
    bind_text_content(
        &status,
        is_active
            .signal()
            .map(|v| if v { "Active" } else { "Inactive" }.to_string()),
    );

    inline1.append(&status);
    canvas1.append(&inline1);
    section1.append(&canvas1);
    container.append(&section1);

    // Visibility binding demo
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Visibility Binding"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let inline2 = create_element("div", &["story-inline"]);

    let is_visible = Mutable::new(true);

    let vis_btn = create_button("Toggle Visibility", &["demo-btn", "demo-btn--warning"]);
    let is_visible_clone = is_visible.clone();
    on_click(&vis_btn.clone().into(), move |_| {
        is_visible_clone.replace_with(|v| !*v);
    });
    inline2.append(&vis_btn);

    let hidden_box: HtmlElement = create_html_element("div", &["wallet-card"]);
    hidden_box.set_text_content(Some("I can be hidden!"));
    hidden_box.style().set_property("padding", "1rem").unwrap();

    bind_visible(&hidden_box, is_visible.signal());

    inline2.append(&hidden_box);
    canvas2.append(&inline2);
    section2.append(&canvas2);
    container.append(&section2);

    container
}

// ============================================================================
// DOM Helpers Story
// ============================================================================

fn render_dom_helpers_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("DOM Helpers"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Utility functions for creating and manipulating DOM elements.",
    ));
    header.append(&desc);
    container.append(&header);

    // Element creation
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Element Creation"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let inline = create_element("div", &["story-inline"]);

    // Buttons
    let btn1 = create_button("Primary", &["demo-btn", "demo-btn--primary"]);
    inline.append(&btn1);

    let btn2 = create_button("Success", &["demo-btn", "demo-btn--success"]);
    inline.append(&btn2);

    let btn3 = create_button("Warning", &["demo-btn", "demo-btn--warning"]);
    inline.append(&btn3);

    let btn4 = create_button("Danger", &["demo-btn", "demo-btn--danger"]);
    inline.append(&btn4);

    canvas.append(&inline);
    section.append(&canvas);
    container.append(&section);

    // Inputs
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Input Elements"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let inline2 = create_element("div", &["story-inline"]);

    let text_input = create_input("text", &["demo-input"]);
    text_input.set_placeholder("Text input");
    inline2.append(&text_input);

    let password_input = create_input("password", &["demo-input"]);
    password_input.set_placeholder("Password input");
    inline2.append(&password_input);

    canvas2.append(&inline2);
    section2.append(&canvas2);
    container.append(&section2);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Usage"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r#"use primitives::{create_element, create_button, create_input};

// Create element with classes
let div = create_element("div", &["container", "active"]);

// Create button
let btn = create_button("Click me", &["btn", "btn--primary"]);

// Create input
let input = create_input("text", &["form-input"]);
input.set_placeholder("Enter text...");"#,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}

// ============================================================================
// Event Handlers Story
// ============================================================================

fn render_event_handlers_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Event Handlers"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Attach event listeners to elements with type-safe handlers.",
    ));
    header.append(&desc);
    container.append(&header);

    // Click events
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Click Events"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);

    let click_count = Mutable::new(0u32);

    let inline = create_element("div", &["story-inline"]);

    let click_btn = create_button("Click me!", &["demo-btn", "demo-btn--primary"]);
    let click_count_clone = click_count.clone();
    on_click(&click_btn.clone().into(), move |_| {
        click_count_clone.replace_with(|c| *c + 1);
    });
    inline.append(&click_btn);

    let count_display =
        create_element("span", &["status-indicator", "status-indicator--connected"]);
    bind_text_content(
        &count_display,
        click_count
            .signal()
            .map(|c| format!("Clicked: {} times", c)),
    );
    inline.append(&count_display);

    canvas.append(&inline);
    section.append(&canvas);
    container.append(&section);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Usage"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r#"use primitives::{on_click, on_input, on_change};

// Click handler
on_click(&button, |event: MouseEvent| {
    log!("Button clicked!");
});

// Input handler
on_input(&input, |event: Event| {
    let value = input.value();
    log!("Input changed: {}", value);
});

// Change handler (fires on blur)
on_change(&select, |event: Event| {
    log!("Selection changed");
});"#,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}

// ============================================================================
// Wallet Providers Story
// ============================================================================

fn render_wallet_providers_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Wallet Providers"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Supported Cardano wallet providers for CIP-30 integration.",
    ));
    header.append(&desc);
    container.append(&header);

    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Supported Wallets"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &["story-grid"]);

    for provider in WalletProvider::all() {
        let card = create_element("div", &["wallet-card"]);

        let header = create_element("div", &["wallet-card__header"]);
        let icon = create_element("div", &["wallet-card__icon"]);
        icon.set_text_content(Some(wallet_icon(*provider)));
        header.append(&icon);

        let name = create_element("span", &["wallet-card__name"]);
        name.set_text_content(Some(provider.display_name()));
        header.append(&name);
        card.append(&header);

        let body = create_element("div", &["wallet-card__body"]);

        let row = create_element("div", &["wallet-card__row"]);
        let label = create_element("span", &["wallet-card__label"]);
        label.set_text_content(Some("API Name"));
        row.append(&label);
        let value = create_element("span", &["wallet-card__value"]);
        value.set_text_content(Some(provider.api_name()));
        row.append(&value);
        body.append(&row);

        card.append(&body);
        grid.append(&card);
    }

    canvas.append(&grid);
    section.append(&canvas);
    container.append(&section);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Usage"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r#"use wallet_core::WalletProvider;

// Get all providers
for provider in WalletProvider::all() {
    println!("{}: {}", provider.display_name(), provider.api_name());
}

// Check specific provider
let nami = WalletProvider::Nami;
assert_eq!(nami.api_name(), "nami");
assert_eq!(nami.display_name(), "Nami");"#,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}

fn wallet_icon(provider: WalletProvider) -> &'static str {
    match provider {
        WalletProvider::Nami => "N",
        WalletProvider::Eternl => "E",
        WalletProvider::Lace => "L",
        WalletProvider::Flint => "F",
        WalletProvider::Typhon => "T",
        WalletProvider::Vespr => "V",
        WalletProvider::NuFi => "Nu",
        WalletProvider::Gero => "G",
        WalletProvider::Yoroi => "Y",
    }
}

// ============================================================================
// Connection States Story
// ============================================================================

fn render_connection_states_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Connection States"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some("Visual representation of wallet connection states."));
    header.append(&desc);
    container.append(&header);

    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("State Indicators"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let inline = create_element("div", &["story-inline"]);

    // Disconnected
    let disc = create_element(
        "span",
        &["status-indicator", "status-indicator--disconnected"],
    );
    disc.set_text_content(Some("Disconnected"));
    inline.append(&disc);

    // Connecting
    let connecting = create_element(
        "span",
        &["status-indicator", "status-indicator--connecting"],
    );
    connecting.set_text_content(Some("Connecting..."));
    inline.append(&connecting);

    // Connected
    let connected = create_element("span", &["status-indicator", "status-indicator--connected"]);
    connected.set_text_content(Some("Connected"));
    inline.append(&connected);

    canvas.append(&inline);
    section.append(&canvas);
    container.append(&section);

    // Connection state cards
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Connection State Examples"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &["story-grid"]);

    // Disconnected state
    let state1 = ConnectionState::Disconnected;
    grid.append(&render_connection_card(&state1));

    // Connecting state
    let state2 = ConnectionState::Connecting;
    grid.append(&render_connection_card(&state2));

    // Connected state
    let state3 = ConnectionState::Connected {
        provider: WalletProvider::Eternl,
        address: "addr1qx...abc123".to_string(),
        network: Network::Mainnet,
    };
    grid.append(&render_connection_card(&state3));

    // Error state
    let state4 = ConnectionState::Error("User rejected connection".to_string());
    grid.append(&render_connection_card(&state4));

    canvas2.append(&grid);
    section2.append(&canvas2);
    container.append(&section2);

    container
}

fn render_connection_card(state: &ConnectionState) -> Element {
    let card = create_element("div", &["wallet-card"]);

    let header = create_element("div", &["wallet-card__header"]);

    let (status_class, status_text) = match state {
        ConnectionState::Disconnected => ("status-indicator--disconnected", "Disconnected"),
        ConnectionState::Connecting => ("status-indicator--connecting", "Connecting"),
        ConnectionState::Connected { .. } => ("status-indicator--connected", "Connected"),
        ConnectionState::Error(_) => ("status-indicator--disconnected", "Error"),
    };

    let status = create_element("span", &["status-indicator", status_class]);
    status.set_text_content(Some(status_text));
    header.append(&status);
    card.append(&header);

    let body = create_element("div", &["wallet-card__body"]);

    match state {
        ConnectionState::Connected {
            provider,
            address,
            network,
        } => {
            // Provider row
            let row1 = create_element("div", &["wallet-card__row"]);
            let label1 = create_element("span", &["wallet-card__label"]);
            label1.set_text_content(Some("Provider"));
            row1.append(&label1);
            let value1 = create_element("span", &["wallet-card__value"]);
            value1.set_text_content(Some(provider.display_name()));
            row1.append(&value1);
            body.append(&row1);

            // Address row
            let row2 = create_element("div", &["wallet-card__row"]);
            let label2 = create_element("span", &["wallet-card__label"]);
            label2.set_text_content(Some("Address"));
            row2.append(&label2);
            let value2 = create_element("span", &["wallet-card__value"]);
            value2.set_text_content(Some(address));
            row2.append(&value2);
            body.append(&row2);

            // Network row
            let row3 = create_element("div", &["wallet-card__row"]);
            let label3 = create_element("span", &["wallet-card__label"]);
            label3.set_text_content(Some("Network"));
            row3.append(&label3);
            let value3 = create_element("span", &["wallet-card__value"]);
            value3.set_text_content(Some(match network {
                Network::Mainnet => "Mainnet",
                Network::Preprod => "Preprod",
                Network::Preview => "Preview",
            }));
            row3.append(&value3);
            body.append(&row3);
        }
        ConnectionState::Error(msg) => {
            let row = create_element("div", &["wallet-card__row"]);
            let label = create_element("span", &["wallet-card__label"]);
            label.set_text_content(Some("Error"));
            row.append(&label);
            let value = create_element("span", &["wallet-card__value"]);
            value.set_text_content(Some(msg));
            row.append(&value);
            body.append(&row);
        }
        _ => {
            let msg = create_element("p", &[]);
            msg.set_text_content(Some("No additional details"));
            body.append(&msg);
        }
    }

    card.append(&body);
    card
}
