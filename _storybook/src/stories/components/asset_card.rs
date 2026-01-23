//! Asset Card component story

use crate::stories::helpers::render_attribute_card;
use futures_signals::signal::{Mutable, SignalExt};
use primitives::{bind_text_content, create_element, document, AppendChild};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

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
