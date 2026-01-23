//! Welcome story - landing page for the storybook

use super::helpers::render_feature_card;
use primitives::{create_element, AppendChild};
use web_sys::Element;

pub fn render_welcome() -> Element {
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
