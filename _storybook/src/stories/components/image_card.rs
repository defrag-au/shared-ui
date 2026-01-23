//! Image Card component story

use crate::stories::helpers::render_attribute_card;
use primitives::{create_element, document, AppendChild};
use web_sys::Element;

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
