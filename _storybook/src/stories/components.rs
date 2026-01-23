//! Component stories - image-card, asset-card, connection-status, memory-card

use super::helpers::render_attribute_card;
use futures_signals::signal::{Mutable, SignalExt};
use primitives::{
    bind_text_content, create_button, create_element, document, on_click, AppendChild,
};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

// ============================================================================
// Image Card Component Story
// ============================================================================

pub fn render_image_card_story() -> Element {
    // Register components
    ui_components::define_all();

    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Image Card"));
    header.append(&h2);

    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "A basic card component for displaying images with optional name overlay and accent color. Foundation for other card components.",
    ));
    header.append(&desc);
    container.append(&header);

    // Basic Examples section
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Examples"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &[]);
    grid.set_attribute(
        "style",
        "display: grid; grid-template-columns: repeat(4, 120px); gap: 1rem;",
    )
    .unwrap();

    // Basic card
    let card1 = document().create_element("image-card").unwrap();
    card1.set_attribute("image-url", "https://iiif.hodlcroft.com/iiif/3/b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6:506972617465313839/full/400,/0/default.jpg").unwrap();
    card1.set_attribute("name", "Basic Card").unwrap();
    grid.append_child(&card1).unwrap();

    // Card with name shown
    let card2 = document().create_element("image-card").unwrap();
    card2.set_attribute("image-url", "https://iiif.hodlcroft.com/iiif/3/b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6:506972617465323030/full/400,/0/default.jpg").unwrap();
    card2.set_attribute("name", "With Name").unwrap();
    card2.set_attribute("show-name", "").unwrap();
    grid.append_child(&card2).unwrap();

    // Card with accent color
    let card3 = document().create_element("image-card").unwrap();
    card3.set_attribute("image-url", "https://iiif.hodlcroft.com/iiif/3/b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6:506972617465333333/full/400,/0/default.jpg").unwrap();
    card3.set_attribute("name", "Gold Accent").unwrap();
    card3.set_attribute("show-name", "").unwrap();
    card3.set_attribute("accent-color", "#FFD700").unwrap();
    grid.append_child(&card3).unwrap();

    // Static card
    let card4 = document().create_element("image-card").unwrap();
    card4.set_attribute("image-url", "https://iiif.hodlcroft.com/iiif/3/b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6:506972617465343434/full/400,/0/default.jpg").unwrap();
    card4.set_attribute("name", "Static").unwrap();
    card4.set_attribute("show-name", "").unwrap();
    card4.set_attribute("static", "").unwrap();
    grid.append_child(&card4).unwrap();

    canvas.append(&grid);
    section.append(&canvas);
    container.append(&section);

    // Size Variants section
    let size_section = create_element("div", &["story-section"]);
    let size_h3 = create_element("h3", &[]);
    size_h3.set_text_content(Some("Size Variants"));
    size_section.append(&size_h3);

    let size_canvas = create_element("div", &["story-canvas"]);
    let size_row = create_element("div", &[]);
    size_row
        .set_attribute(
            "style",
            "display: flex; align-items: flex-end; gap: 1rem; flex-wrap: wrap;",
        )
        .unwrap();

    // xs (80px)
    let xs_card = document().create_element("image-card").unwrap();
    xs_card.set_attribute("image-url", "https://iiif.hodlcroft.com/iiif/3/b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6:506972617465313839/full/400,/0/default.jpg").unwrap();
    xs_card.set_attribute("name", "xs (80px)").unwrap();
    xs_card.set_attribute("size", "xs").unwrap();
    xs_card.set_attribute("show-name", "").unwrap();
    size_row.append_child(&xs_card).unwrap();

    // sm (120px)
    let sm_card = document().create_element("image-card").unwrap();
    sm_card.set_attribute("image-url", "https://iiif.hodlcroft.com/iiif/3/b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6:506972617465323030/full/400,/0/default.jpg").unwrap();
    sm_card.set_attribute("name", "sm (120px)").unwrap();
    sm_card.set_attribute("size", "sm").unwrap();
    sm_card.set_attribute("show-name", "").unwrap();
    size_row.append_child(&sm_card).unwrap();

    // md (240px)
    let md_card = document().create_element("image-card").unwrap();
    md_card.set_attribute("image-url", "https://iiif.hodlcroft.com/iiif/3/b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6:506972617465333333/full/400,/0/default.jpg").unwrap();
    md_card.set_attribute("name", "md (240px)").unwrap();
    md_card.set_attribute("size", "md").unwrap();
    md_card.set_attribute("show-name", "").unwrap();
    size_row.append_child(&md_card).unwrap();

    size_canvas.append(&size_row);

    // Second row for larger sizes
    let size_row2 = create_element("div", &[]);
    size_row2
        .set_attribute(
            "style",
            "display: flex; align-items: flex-end; gap: 1rem; margin-top: 1rem; flex-wrap: wrap;",
        )
        .unwrap();

    // lg (400px)
    let lg_card = document().create_element("image-card").unwrap();
    lg_card.set_attribute("image-url", "https://iiif.hodlcroft.com/iiif/3/b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6:506972617465343434/full/400,/0/default.jpg").unwrap();
    lg_card.set_attribute("name", "lg (400px)").unwrap();
    lg_card.set_attribute("size", "lg").unwrap();
    lg_card.set_attribute("show-name", "").unwrap();
    size_row2.append_child(&lg_card).unwrap();

    size_canvas.append(&size_row2);
    size_section.append(&size_canvas);
    container.append(&size_section);

    // Attributes section
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Attributes"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let grid2 = create_element("div", &["story-grid"]);

    let attr1 = render_attribute_card("image-url", "URL string", "URL of the image to display");
    grid2.append(&attr1);

    let attr2 = render_attribute_card("name", "string", "Name shown in overlay and as tooltip");
    grid2.append(&attr2);

    let attr3 = render_attribute_card(
        "size",
        "xs | sm | md | lg | xl",
        "Card size: 80px, 120px (default), 240px, 400px, 800px",
    );
    grid2.append(&attr3);

    let attr4 = render_attribute_card("show-name", "boolean", "If present, shows the name overlay");
    grid2.append(&attr4);

    let attr5 = render_attribute_card(
        "accent-color",
        "CSS color",
        "Color for the accent bar at top",
    );
    grid2.append(&attr5);

    let attr6 = render_attribute_card(
        "static",
        "boolean",
        "If present, disables hover effects and clicks",
    );
    grid2.append(&attr6);

    canvas2.append(&grid2);
    section2.append(&canvas2);
    container.append(&section2);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Usage"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r##"// Register components at app startup
ui_components::define_all();

// Basic image card (default sm size)
<image-card
    image-url="https://example.com/image.png"
    name="My Image"
></image-card>

// Medium size with name overlay
<image-card
    image-url="https://..."
    name="Featured"
    size="md"
    show-name
></image-card>

// Large card with accent color
<image-card
    image-url="https://..."
    name="Hero"
    size="lg"
    show-name
    accent-color="#FFD700"
></image-card>

// Listen for clicks
<image-card
    attr:image-url=url
    attr:name=name
    attr:size="md"
    on:card-click=move |_| { handle_click(); }
/>"##,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}

// ============================================================================
// Asset Card Component Story
// ============================================================================

pub fn render_asset_card_story() -> Element {
    // Register components
    ui_components::define_all();

    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Asset Card"));
    header.append(&h2);

    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Cardano NFT asset card with automatic IIIF URL generation. Wraps <image-card> and derives image URLs from asset IDs. IIIF image resolution is automatically selected based on card size.",
    ));
    header.append(&desc);
    container.append(&header);

    // Size Variants section
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Size Variants"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let size_row = create_element("div", &[]);
    size_row
        .set_attribute(
            "style",
            "display: flex; align-items: flex-end; gap: 1rem; flex-wrap: wrap;",
        )
        .unwrap();

    // xs (80px) - uses 400px IIIF
    let xs_card = document().create_element("asset-card").unwrap();
    xs_card
        .set_attribute(
            "asset-id",
            "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839",
        )
        .unwrap();
    xs_card.set_attribute("name", "xs (80px)").unwrap();
    xs_card.set_attribute("size", "xs").unwrap();
    xs_card.set_attribute("show-name", "").unwrap();
    size_row.append_child(&xs_card).unwrap();

    // sm (120px) - uses 400px IIIF
    let sm_card = document().create_element("asset-card").unwrap();
    sm_card
        .set_attribute(
            "asset-id",
            "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465323030",
        )
        .unwrap();
    sm_card.set_attribute("name", "sm (120px)").unwrap();
    sm_card.set_attribute("size", "sm").unwrap();
    sm_card.set_attribute("show-name", "").unwrap();
    size_row.append_child(&sm_card).unwrap();

    // md (240px) - uses 400px IIIF
    let md_card = document().create_element("asset-card").unwrap();
    md_card
        .set_attribute(
            "asset-id",
            "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465333333",
        )
        .unwrap();
    md_card.set_attribute("name", "md (240px)").unwrap();
    md_card.set_attribute("size", "md").unwrap();
    md_card.set_attribute("show-name", "").unwrap();
    size_row.append_child(&md_card).unwrap();

    canvas.append(&size_row);

    // Second row for larger sizes
    let size_row2 = create_element("div", &[]);
    size_row2
        .set_attribute(
            "style",
            "display: flex; align-items: flex-end; gap: 1rem; margin-top: 1rem; flex-wrap: wrap;",
        )
        .unwrap();

    // lg (400px) - uses 400px IIIF
    let lg_card = document().create_element("asset-card").unwrap();
    lg_card
        .set_attribute(
            "asset-id",
            "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465343434",
        )
        .unwrap();
    lg_card
        .set_attribute("name", "lg (400px) - 400px IIIF")
        .unwrap();
    lg_card.set_attribute("size", "lg").unwrap();
    lg_card.set_attribute("show-name", "").unwrap();
    size_row2.append_child(&lg_card).unwrap();

    canvas.append(&size_row2);
    section.append(&canvas);
    container.append(&section);

    // IIIF Info section
    let iiif_section = create_element("div", &["story-section"]);
    let iiif_h3 = create_element("h3", &[]);
    iiif_h3.set_text_content(Some("IIIF Image Selection"));
    iiif_section.append(&iiif_h3);

    let iiif_canvas = create_element("div", &["story-canvas"]);
    let iiif_grid = create_element("div", &["story-grid"]);

    let iiif1 = render_attribute_card(
        "xs, sm, md, lg",
        "400px IIIF",
        "Card sizes up to 400px use the cached 400px IIIF image for fast loading",
    );
    iiif_grid.append(&iiif1);

    let iiif2 = render_attribute_card(
        "xl",
        "1686px IIIF",
        "Card sizes above 400px use the high-resolution 1686px IIIF image",
    );
    iiif_grid.append(&iiif2);

    iiif_canvas.append(&iiif_grid);
    iiif_section.append(&iiif_canvas);
    container.append(&iiif_section);

    // Interactive Demo section
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Interactive Demo"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);

    let demo_row = create_element("div", &[]);
    demo_row
        .set_attribute("style", "display: flex; align-items: center; gap: 1rem;")
        .unwrap();

    // Interactive card
    let asset_id = "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465353535";
    let demo_card: HtmlElement = document()
        .create_element("asset-card")
        .unwrap()
        .unchecked_into();
    demo_card.set_attribute("asset-id", asset_id).unwrap();
    demo_card.set_attribute("name", "Click me!").unwrap();
    demo_card.set_attribute("show-name", "").unwrap();
    demo_card.style().set_property("width", "140px").unwrap();
    demo_row.append_child(&demo_card).unwrap();

    // Click counter
    let click_count = Mutable::new(0u32);
    let last_asset_id = Mutable::new(String::new());

    let status_label = create_element("span", &["status-indicator", "status-indicator--connected"]);
    {
        let click_count = click_count.clone();
        bind_text_content(
            &status_label,
            click_count.signal().map(move |c| {
                if c == 0 {
                    "Click the card!".to_string()
                } else {
                    format!("Clicks: {c}")
                }
            }),
        );
    }
    demo_row.append(&status_label);

    let asset_label = create_element("span", &["status-indicator"]);
    bind_text_content(
        &asset_label,
        last_asset_id.signal_cloned().map(|id| {
            if id.is_empty() {
                String::new()
            } else {
                // Show truncated asset ID
                let truncated = if id.len() > 20 {
                    format!("{}...{}", &id[..10], &id[id.len() - 10..])
                } else {
                    id
                };
                format!("ID: {truncated}")
            }
        }),
    );
    demo_row.append(&asset_label);

    // Listen for card-click events
    {
        let click_count = click_count.clone();
        let last_asset_id = last_asset_id.clone();
        let closure =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::CustomEvent| {
                click_count.replace_with(|c| *c + 1);
                if let Some(detail) = e.detail().as_string() {
                    last_asset_id.set(detail);
                }
            }) as Box<dyn Fn(_)>);
        demo_card
            .add_event_listener_with_callback("card-click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

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
        "asset-id",
        "string",
        "Cardano asset ID (policy_id + asset_name hex). Used for IIIF lookup and click events.",
    );
    grid3.append(&attr1);

    let attr2 = render_attribute_card(
        "size",
        "xs | sm | md | lg | xl",
        "Card size: 80px, 120px (default), 240px, 400px, 800px. IIIF resolution auto-selected.",
    );
    grid3.append(&attr2);

    let attr4 = render_attribute_card(
        "name",
        "string",
        "Asset name (used for title tooltip and overlay)",
    );
    grid3.append(&attr4);

    let attr5 = render_attribute_card(
        "show-name",
        "boolean (present/absent)",
        "Whether to show the name overlay at the bottom",
    );
    grid3.append(&attr5);

    let attr6 = render_attribute_card(
        "accent-color",
        "CSS color string",
        "Color for the accent bar at the top (e.g., tier color)",
    );
    grid3.append(&attr6);

    let attr7 = render_attribute_card(
        "static",
        "boolean (present/absent)",
        "If present, disables hover effects and click events",
    );
    grid3.append(&attr7);

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
        r##"// Register components at app startup
ui_components::define_all();

// Small card (default) - uses 400px IIIF image
<asset-card
    asset-id="{policy_id}{asset_name_hex}"
    name="Pirate #189"
    show-name
></asset-card>

// Medium card - still uses 400px IIIF (cached, fast)
<asset-card
    asset-id="{policy_id}{asset_name_hex}"
    size="md"
    name="Featured Pirate"
    show-name
></asset-card>

// Extra large card - auto-selects 1686px IIIF
<asset-card
    asset-id="{policy_id}{asset_name_hex}"
    size="xl"
    name="Hero Display"
    show-name
></asset-card>

// With accent color for tier indication
<asset-card
    asset-id="{asset_id}"
    size="lg"
    name="Gold Tier Asset"
    show-name
    accent-color="#FFD700"
></asset-card>

// Listen for clicks (Leptos example)
<asset-card
    attr:asset-id=asset_id
    attr:name=name
    attr:size="md"
    on:card-click=move |e: web_sys::CustomEvent| {
        if let Some(id) = e.detail().as_string() {
            handle_asset_click(id);
        }
    }
/>"##,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}

// ============================================================================
// Connection Status Component Story
// ============================================================================

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

// ============================================================================
// Memory Card Component Story
// ============================================================================

pub fn render_memory_card_story() -> Element {
    // Register components
    ui_components::define_all();

    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Memory Card"));
    header.append(&h2);

    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "A flippable card component for the memory matching game. Wraps <asset-card> internally. Shows card back when face-down, asset image when flipped.",
    ));
    header.append(&desc);
    container.append(&header);

    // Card States section
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Card States"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &[]);
    grid.set_attribute(
        "style",
        "display: grid; grid-template-columns: repeat(4, 100px); gap: 1rem;",
    )
    .unwrap();

    // Face down card
    let card1 = document().create_element("memory-card").unwrap();
    card1
        .set_attribute(
            "asset-id",
            "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465313839",
        )
        .unwrap();
    card1.set_attribute("name", "Face Down").unwrap();
    grid.append_child(&card1).unwrap();

    // Flipped card
    let card2 = document().create_element("memory-card").unwrap();
    card2
        .set_attribute(
            "asset-id",
            "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465323030",
        )
        .unwrap();
    card2.set_attribute("name", "Flipped Card").unwrap();
    card2.set_attribute("flipped", "").unwrap();
    grid.append_child(&card2).unwrap();

    // Matched card
    let card3 = document().create_element("memory-card").unwrap();
    card3
        .set_attribute(
            "asset-id",
            "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465333333",
        )
        .unwrap();
    card3.set_attribute("name", "Matched Card").unwrap();
    card3.set_attribute("flipped", "").unwrap();
    card3.set_attribute("matched", "").unwrap();
    card3.set_attribute("matched-by", "Player1").unwrap();
    grid.append_child(&card3).unwrap();

    // Disabled card
    let card4 = document().create_element("memory-card").unwrap();
    card4
        .set_attribute(
            "asset-id",
            "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465343434",
        )
        .unwrap();
    card4.set_attribute("name", "Disabled").unwrap();
    card4.set_attribute("disabled", "").unwrap();
    grid.append_child(&card4).unwrap();

    canvas.append(&grid);
    section.append(&canvas);
    container.append(&section);

    // Interactive Demo section
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Interactive Demo"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);

    let demo_row = create_element("div", &[]);
    demo_row
        .set_attribute("style", "display: flex; align-items: center; gap: 1rem;")
        .unwrap();

    // Interactive card
    let demo_card: HtmlElement = document()
        .create_element("memory-card")
        .unwrap()
        .unchecked_into();
    demo_card
        .set_attribute(
            "asset-id",
            "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6506972617465353535",
        )
        .unwrap();
    demo_card.set_attribute("name", "Click to flip!").unwrap();
    demo_card.style().set_property("width", "120px").unwrap();
    demo_row.append_child(&demo_card).unwrap();

    // Click counter
    let click_count = Mutable::new(0u32);
    let is_flipped = Mutable::new(false);

    let status_label = create_element("span", &["status-indicator", "status-indicator--connected"]);
    bind_text_content(
        &status_label,
        click_count.signal().map(|c| format!("Clicks: {}", c)),
    );
    demo_row.append(&status_label);

    // Toggle button
    let toggle_btn = create_button("Toggle Flip", &["demo-btn", "demo-btn--primary"]);
    let demo_card_toggle = demo_card.clone();
    let is_flipped_toggle = is_flipped.clone();
    on_click(&toggle_btn.clone().into(), move |_| {
        let new_state = !is_flipped_toggle.get();
        is_flipped_toggle.set(new_state);
        if new_state {
            demo_card_toggle.set_attribute("flipped", "").unwrap();
        } else {
            demo_card_toggle.remove_attribute("flipped").unwrap();
        }
    });
    demo_row.append(&toggle_btn);

    // Listen for card-click events
    {
        let click_count = click_count.clone();
        let is_flipped = is_flipped.clone();
        let demo_card_click = demo_card.clone();
        let closure =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::CustomEvent| {
                click_count.replace_with(|c| *c + 1);
                let new_state = !is_flipped.get();
                is_flipped.set(new_state);
                if new_state {
                    demo_card_click.set_attribute("flipped", "").unwrap();
                } else {
                    demo_card_click.remove_attribute("flipped").unwrap();
                }
            }) as Box<dyn Fn(_)>);
        demo_card
            .add_event_listener_with_callback("card-click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

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
        "asset-id",
        "string",
        "Cardano asset ID (policy_id + asset_name hex) for IIIF image lookup",
    );
    grid3.append(&attr1);

    let attr2 = render_attribute_card(
        "name",
        "string",
        "Asset name shown as overlay on flipped card",
    );
    grid3.append(&attr2);

    let attr3 = render_attribute_card(
        "flipped",
        "boolean (present/absent)",
        "Whether the card is face-up",
    );
    grid3.append(&attr3);

    let attr4 = render_attribute_card(
        "matched",
        "boolean (present/absent)",
        "Whether the card has been matched (shows glow)",
    );
    grid3.append(&attr4);

    let attr5 = render_attribute_card(
        "matched-by",
        "string",
        "Name of player who matched this card",
    );
    grid3.append(&attr5);

    let attr6 = render_attribute_card(
        "disabled",
        "boolean (present/absent)",
        "Whether the card can be clicked",
    );
    grid3.append(&attr6);

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

// Basic card (face down)
<memory-card
    asset-id="{policy_id}{asset_name_hex}"
    name="Asset Name"
></memory-card>

// Flipped card (face up)
<memory-card
    asset-id="{policy_id}{asset_name_hex}"
    name="Asset Name"
    flipped
></memory-card>

// Matched card with player attribution
<memory-card
    asset-id="{policy_id}{asset_name_hex}"
    name="Asset Name"
    flipped
    matched
    matched-by="Player1"
></memory-card>

// Listen for clicks (Leptos)
<memory-card
    attr:asset-id=asset_id
    attr:name=name
    on:card-click=move |_| { handle_flip(index); }
/>"#,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}
