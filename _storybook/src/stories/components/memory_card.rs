//! Memory Card component story

use crate::stories::helpers::render_attribute_card;
use futures_signals::signal::{Mutable, SignalExt};
use primitives::{
    bind_text_content, create_button, create_element, document, on_click, AppendChild,
};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

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
