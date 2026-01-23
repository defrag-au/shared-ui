//! Connection Status component story

use crate::stories::helpers::render_attribute_card;
use futures_signals::signal::{Mutable, SignalExt};
use primitives::{
    bind_text_content, create_button, create_element, document, on_click, AppendChild,
};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

pub fn render_connection_status_story() -> Element {
    // Register components
    ui_components::define_all();

    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Connection Status"));
    header.append(&h2);

    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "A web component for displaying WebSocket/realtime connection state with click-to-reconnect support.",
    ));
    header.append(&desc);
    container.append(&header);

    // States section
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Connection States"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let inline = create_element("div", &["story-inline"]);

    // Connected
    let connected = document().create_element("connection-status").unwrap();
    connected.set_attribute("status", "connected").unwrap();
    inline.append_child(&connected).unwrap();

    // Connecting
    let connecting = document().create_element("connection-status").unwrap();
    connecting.set_attribute("status", "connecting").unwrap();
    inline.append_child(&connecting).unwrap();

    // Disconnected
    let disconnected = document().create_element("connection-status").unwrap();
    disconnected
        .set_attribute("status", "disconnected")
        .unwrap();
    inline.append_child(&disconnected).unwrap();

    // Error
    let error = document().create_element("connection-status").unwrap();
    error.set_attribute("status", "error").unwrap();
    inline.append_child(&error).unwrap();

    canvas.append(&inline);
    section.append(&canvas);
    container.append(&section);

    // Interactive demo section
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Interactive Demo"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let inline2 = create_element("div", &["story-inline"]);

    let status = Mutable::new("disconnected".to_string());

    // Create buttons to change state
    let btn_connected = create_button("Set Connected", &["demo-btn", "demo-btn--success"]);
    let status_clone = status.clone();
    on_click(&btn_connected.clone().into(), move |_| {
        status_clone.set("connected".to_string());
    });
    inline2.append(&btn_connected);

    let btn_connecting = create_button("Set Connecting", &["demo-btn", "demo-btn--warning"]);
    let status_clone = status.clone();
    on_click(&btn_connecting.clone().into(), move |_| {
        status_clone.set("connecting".to_string());
    });
    inline2.append(&btn_connecting);

    let btn_disconnected = create_button("Set Disconnected", &["demo-btn"]);
    let status_clone = status.clone();
    on_click(&btn_disconnected.clone().into(), move |_| {
        status_clone.set("disconnected".to_string());
    });
    inline2.append(&btn_disconnected);

    canvas2.append(&inline2);

    // The reactive component
    let demo_row = create_element("div", &["story-inline"]);
    demo_row
        .set_attribute("style", "margin-top: 1rem;")
        .unwrap();

    let demo_component: HtmlElement = document()
        .create_element("connection-status")
        .unwrap()
        .unchecked_into();
    demo_component
        .set_attribute("status", "disconnected")
        .unwrap();
    demo_row.append_child(&demo_component).unwrap();

    // Bind status attribute reactively
    {
        let demo_component = demo_component.clone();
        let future = status.signal_cloned().for_each(move |s| {
            demo_component.set_attribute("status", &s).unwrap();
            async {}
        });
        wasm_bindgen_futures::spawn_local(future);
    }

    // Listen for reconnect event
    let reconnect_count = Mutable::new(0u32);
    let reconnect_label =
        create_element("span", &["status-indicator", "status-indicator--connected"]);
    bind_text_content(
        &reconnect_label,
        reconnect_count
            .signal()
            .map(|c| format!("Reconnect clicked: {} times", c)),
    );

    {
        let reconnect_count = reconnect_count.clone();
        let closure =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::CustomEvent| {
                reconnect_count.replace_with(|c| *c + 1);
            }) as Box<dyn Fn(_)>);
        demo_component
            .add_event_listener_with_callback("reconnect", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    demo_row.append(&reconnect_label);
    canvas2.append(&demo_row);
    section2.append(&canvas2);
    container.append(&section2);

    // Attributes section
    let section3 = create_element("div", &["story-section"]);
    let h3_3 = create_element("h3", &[]);
    h3_3.set_text_content(Some("Attributes"));
    section3.append(&h3_3);

    let canvas3 = create_element("div", &["story-canvas"]);
    let grid3 = create_element("div", &["story-grid"]);

    let attr1 = render_attribute_card(
        "status",
        "connected | connecting | disconnected | error",
        "Connection state to display",
    );
    grid3.append(&attr1);

    let attr2 = render_attribute_card(
        "show-text",
        "true | false",
        "Whether to show status text (default: true)",
    );
    grid3.append(&attr2);

    let attr3 = render_attribute_card(
        "clickable",
        "true | false",
        "Override auto-clickable behavior",
    );
    grid3.append(&attr3);

    canvas3.append(&grid3);
    section3.append(&canvas3);
    container.append(&section3);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Usage"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r#"// Register components at app startup
ui_components::define_all();

// Use in HTML
<connection-status status="connected"></connection-status>
<connection-status status="disconnected"></connection-status>

// Listen for reconnect event (Leptos example)
<connection-status
    attr:status=status_str
    on:reconnect=move |_| { reconnect(); }
/>

// Vanilla JS
const el = document.querySelector('connection-status');
el.addEventListener('reconnect', () => {
    console.log('User clicked to reconnect');
});"#,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}
