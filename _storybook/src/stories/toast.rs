//! UI Toast stories - toast types and usage

use super::helpers::{render_toast_fn_card, render_toast_kind_card, render_trait_method_card};
use primitives::{create_element, AppendChild};
use web_sys::Element;

// ============================================================================
// Toast Types Story
// ============================================================================

pub fn render_toast_types_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Toast Types"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "The ui-toast crate provides four toast types for different notification severities.",
    ));
    header.append(&desc);
    container.append(&header);

    // Toast kinds section
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("ToastKind Variants"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &["story-grid"]);

    // Success
    let card1 = render_toast_kind_card(
        "Success",
        "toast--success",
        "\u{2713}",
        "Operation completed successfully",
    );
    grid.append(&card1);

    // Warning
    let card2 = render_toast_kind_card(
        "Warning",
        "toast--warning",
        "\u{26A0}",
        "Something needs attention",
    );
    grid.append(&card2);

    // Error
    let card3 = render_toast_kind_card("Error", "toast--error", "\u{2715}", "An error occurred");
    grid.append(&card3);

    // Info
    let card4 = render_toast_kind_card("Info", "toast--info", "\u{2139}", "Informational message");
    grid.append(&card4);

    canvas.append(&grid);
    section.append(&canvas);
    container.append(&section);

    // Convenience functions
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Convenience Functions"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let grid2 = create_element("div", &["story-grid"]);

    let fn1 = render_toast_fn_card("success(msg)", "Creates a success toast message");
    grid2.append(&fn1);

    let fn2 = render_toast_fn_card("warning(msg)", "Creates a warning toast message");
    grid2.append(&fn2);

    let fn3 = render_toast_fn_card("error(msg)", "Creates an error toast message");
    grid2.append(&fn3);

    let fn4 = render_toast_fn_card("info(msg)", "Creates an info toast message");
    grid2.append(&fn4);

    canvas2.append(&grid2);
    section2.append(&canvas2);
    container.append(&section2);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Creating Toast Messages"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r#"use ui_toast::{success, error, warning, info, show, ToastKind};

// Using convenience functions
let msg = success("File saved successfully");
let msg = error("Failed to connect");
let msg = warning("Low disk space");
let msg = info("New version available");

// Using show() for more control
let msg = show("Custom message", ToastKind::Success);

// With custom icon
let msg = show_with_icon("Uploaded!", ToastKind::Success, "🚀");"#,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}

// ============================================================================
// Toast Usage Story
// ============================================================================

pub fn render_toast_usage_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Toast Usage"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Integrate toasts into your widget using the HasToasts trait.",
    ));
    header.append(&desc);
    container.append(&header);

    // HasToasts trait section
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("HasToasts Trait"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &["story-grid"]);

    let m1 = render_trait_method_card(
        "toasts()",
        "&VecDeque<Toast>",
        "Get reference to toast queue",
    );
    grid.append(&m1);

    let m2 = render_trait_method_card(
        "toasts_mut()",
        "&mut VecDeque<Toast>",
        "Get mutable reference to toast queue",
    );
    grid.append(&m2);

    let m3 = render_trait_method_card("next_toast_id()", "u32", "Get the next toast ID");
    grid.append(&m3);

    let m4 = render_trait_method_card("set_next_toast_id(id)", "()", "Set the next toast ID");
    grid.append(&m4);

    canvas.append(&grid);
    section.append(&canvas);
    container.append(&section);

    // Provided methods section
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Provided Methods"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let grid2 = create_element("div", &["story-grid"]);

    let p1 = render_trait_method_card("add_toast(msg, kind)", "u32", "Add a toast, returns its ID");
    grid2.append(&p1);

    let p2 = render_trait_method_card(
        "add_toast_with_icon(...)",
        "u32",
        "Add toast with custom icon",
    );
    grid2.append(&p2);

    let p3 = render_trait_method_card("dismiss_toast(id)", "()", "Remove a specific toast");
    grid2.append(&p3);

    let p4 = render_trait_method_card(
        "cleanup_expired_toasts()",
        "()",
        "Remove all expired toasts",
    );
    grid2.append(&p4);

    canvas2.append(&grid2);
    section2.append(&canvas2);
    container.append(&section2);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Implementation Example"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r#"use ui_toast::{Toast, ToastKind, HasToasts};
use std::collections::VecDeque;

struct Model {
    toasts: VecDeque<Toast>,
    next_toast_id: u32,
}

impl HasToasts for Model {
    fn toasts(&self) -> &VecDeque<Toast> { &self.toasts }
    fn toasts_mut(&mut self) -> &mut VecDeque<Toast> { &mut self.toasts }
    fn next_toast_id(&self) -> u32 { self.next_toast_id }
    fn set_next_toast_id(&mut self, id: u32) { self.next_toast_id = id; }
}

// Then use the provided methods:
model.add_toast("Success!".to_string(), ToastKind::Success);
model.cleanup_expired_toasts();"#,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}
