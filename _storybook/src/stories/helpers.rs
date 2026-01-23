//! Shared helper functions for rendering story cards

use primitives::{create_element, AppendChild};
use web_sys::Element;

/// Render a feature card for the welcome page
pub fn render_feature_card(title: &str, description: &str) -> Element {
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

/// Render an attribute documentation card
pub fn render_attribute_card(name: &str, values: &str, description: &str) -> Element {
    let card = create_element("div", &["wallet-card"]);

    let header = create_element("div", &["wallet-card__header"]);
    let name_span = create_element("span", &["wallet-card__name"]);
    name_span.set_text_content(Some(name));
    header.append(&name_span);
    card.append(&header);

    let body = create_element("div", &["wallet-card__body"]);

    let row = create_element("div", &["wallet-card__row"]);
    let label = create_element("span", &["wallet-card__label"]);
    label.set_text_content(Some("Values"));
    row.append(&label);
    let value = create_element("span", &["wallet-card__value"]);
    value.set_text_content(Some(values));
    row.append(&value);
    body.append(&row);

    let desc = create_element("p", &[]);
    desc.set_text_content(Some(description));
    desc.set_attribute(
        "style",
        "margin-top: 0.5rem; font-size: 0.9em; color: #8b8fa3;",
    )
    .unwrap();
    body.append(&desc);

    card.append(&body);
    card
}

/// Render a loader step card (numbered)
pub fn render_loader_step_card(step: &str, title: &str, description: &str) -> Element {
    let card = create_element("div", &["wallet-card"]);

    let header = create_element("div", &["wallet-card__header"]);
    let icon = create_element("div", &["wallet-card__icon"]);
    icon.set_text_content(Some(step));
    header.append(&icon);

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

/// Render a loader error card
pub fn render_loader_error_card(error_type: &str, description: &str) -> Element {
    let card = create_element("div", &["wallet-card"]);

    let header = create_element("div", &["wallet-card__header"]);
    let status = create_element(
        "span",
        &["status-indicator", "status-indicator--disconnected"],
    );
    status.set_text_content(Some(error_type));
    header.append(&status);
    card.append(&header);

    let body = create_element("div", &["wallet-card__body"]);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(description));
    body.append(&desc);
    card.append(&body);

    card
}

/// Render a config option card
pub fn render_config_option_card(
    name: &str,
    type_name: &str,
    default: &str,
    description: &str,
) -> Element {
    let card = create_element("div", &["wallet-card"]);

    let header = create_element("div", &["wallet-card__header"]);
    let name_span = create_element("span", &["wallet-card__name"]);
    name_span.set_text_content(Some(name));
    header.append(&name_span);
    card.append(&header);

    let body = create_element("div", &["wallet-card__body"]);

    // Type row
    let row1 = create_element("div", &["wallet-card__row"]);
    let label1 = create_element("span", &["wallet-card__label"]);
    label1.set_text_content(Some("Type"));
    row1.append(&label1);
    let value1 = create_element("span", &["wallet-card__value"]);
    value1.set_text_content(Some(type_name));
    row1.append(&value1);
    body.append(&row1);

    // Default row
    let row2 = create_element("div", &["wallet-card__row"]);
    let label2 = create_element("span", &["wallet-card__label"]);
    label2.set_text_content(Some("Default"));
    row2.append(&label2);
    let value2 = create_element("span", &["wallet-card__value"]);
    value2.set_text_content(Some(default));
    row2.append(&value2);
    body.append(&row2);

    // Description
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(description));
    desc.set_attribute(
        "style",
        "margin-top: 0.5rem; font-size: 0.9em; color: #8b8fa3;",
    )
    .unwrap();
    body.append(&desc);

    card.append(&body);
    card
}

/// Render a trait method card
pub fn render_trait_method_card(signature: &str, returns: &str, description: &str) -> Element {
    let card = create_element("div", &["wallet-card"]);

    let header = create_element("div", &["wallet-card__header"]);
    let name_span = create_element("span", &["wallet-card__name"]);
    name_span.set_text_content(Some(signature));
    header.append(&name_span);
    card.append(&header);

    let body = create_element("div", &["wallet-card__body"]);

    // Returns row
    let row = create_element("div", &["wallet-card__row"]);
    let label = create_element("span", &["wallet-card__label"]);
    label.set_text_content(Some("Returns"));
    row.append(&label);
    let value = create_element("span", &["wallet-card__value"]);
    value.set_text_content(Some(returns));
    row.append(&value);
    body.append(&row);

    // Description
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(description));
    desc.set_attribute(
        "style",
        "margin-top: 0.5rem; font-size: 0.9em; color: #8b8fa3;",
    )
    .unwrap();
    body.append(&desc);

    card.append(&body);
    card
}

/// Render a toast kind card
pub fn render_toast_kind_card(name: &str, css_class: &str, icon: &str, example: &str) -> Element {
    let card = create_element("div", &["wallet-card"]);

    let header = create_element("div", &["wallet-card__header"]);
    let icon_el = create_element("div", &["wallet-card__icon"]);
    icon_el.set_text_content(Some(icon));
    header.append(&icon_el);

    let name_span = create_element("span", &["wallet-card__name"]);
    name_span.set_text_content(Some(name));
    header.append(&name_span);
    card.append(&header);

    let body = create_element("div", &["wallet-card__body"]);

    // CSS class row
    let row1 = create_element("div", &["wallet-card__row"]);
    let label1 = create_element("span", &["wallet-card__label"]);
    label1.set_text_content(Some("CSS Class"));
    row1.append(&label1);
    let value1 = create_element("span", &["wallet-card__value"]);
    value1.set_text_content(Some(css_class));
    row1.append(&value1);
    body.append(&row1);

    // Example row
    let row2 = create_element("div", &["wallet-card__row"]);
    let label2 = create_element("span", &["wallet-card__label"]);
    label2.set_text_content(Some("Example"));
    row2.append(&label2);
    let value2 = create_element("span", &["wallet-card__value"]);
    value2.set_text_content(Some(example));
    row2.append(&value2);
    body.append(&row2);

    card.append(&body);
    card
}

/// Render a toast function card
pub fn render_toast_fn_card(signature: &str, description: &str) -> Element {
    let card = create_element("div", &["wallet-card"]);

    let header = create_element("div", &["wallet-card__header"]);
    let name_span = create_element("span", &["wallet-card__name"]);
    name_span.set_text_content(Some(signature));
    header.append(&name_span);
    card.append(&header);

    let body = create_element("div", &["wallet-card__body"]);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(description));
    body.append(&desc);
    card.append(&body);

    card
}

/// Render a flow concept card
pub fn render_flow_concept_card(title: &str, description: &str) -> Element {
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
