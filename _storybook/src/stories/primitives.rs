//! Primitives stories - reactive bindings, DOM helpers, event handlers

use futures_signals::signal::{Mutable, SignalExt};
use primitives::{
    bind_class, bind_text_content, bind_visible, create_button, create_element,
    create_html_element, create_input, document, on_click, on_input, text_signal, AppendChild,
};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, HtmlInputElement};

// ============================================================================
// Reactive Text Story
// ============================================================================

pub fn render_reactive_text_story() -> Element {
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

pub fn render_reactive_bindings_story() -> Element {
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

pub fn render_dom_helpers_story() -> Element {
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

pub fn render_event_handlers_story() -> Element {
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
