//! DOM helper utilities for creating elements

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, HtmlInputElement};

use crate::reactive::document;

/// Helper to create an element with classes
pub fn create_element(tag: &str, classes: &[&str]) -> Element {
    let el = document().create_element(tag).unwrap();
    for class in classes {
        let _ = el.class_list().add_1(class);
    }
    el
}

/// Helper to create an HTML element with classes
pub fn create_html_element(tag: &str, classes: &[&str]) -> HtmlElement {
    create_element(tag, classes).unchecked_into()
}

/// Helper to create an input element
pub fn create_input(input_type: &str, classes: &[&str]) -> HtmlInputElement {
    let input: HtmlInputElement = create_element("input", classes).unchecked_into();
    input.set_type(input_type);
    input
}

/// Helper to create a button element
pub fn create_button(text: &str, classes: &[&str]) -> HtmlElement {
    let btn = create_html_element("button", classes);
    btn.set_text_content(Some(text));
    let _ = btn.set_attribute("type", "button");
    btn
}

/// Event listener helper for click events
pub fn on_click<F>(element: &Element, handler: F)
where
    F: Fn(web_sys::MouseEvent) + 'static,
{
    let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn(_)>);
    let _ = element.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
    closure.forget();
}

/// Event listener for input events
pub fn on_input<F>(element: &HtmlInputElement, handler: F)
where
    F: Fn(web_sys::Event) + 'static,
{
    let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn(_)>);
    let _ = element.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref());
    closure.forget();
}

/// Event listener for change events
pub fn on_change<F>(element: &Element, handler: F)
where
    F: Fn(web_sys::Event) + 'static,
{
    let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn(_)>);
    let _ = element.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
    closure.forget();
}

/// Event listener for keydown events
pub fn on_keydown<F>(element: &Element, handler: F)
where
    F: Fn(web_sys::KeyboardEvent) + 'static,
{
    let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn(_)>);
    let _ = element.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
    closure.forget();
}

/// Event listener for focus events
pub fn on_focus<F>(element: &Element, handler: F)
where
    F: Fn(web_sys::FocusEvent) + 'static,
{
    let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn(_)>);
    let _ = element.add_event_listener_with_callback("focus", closure.as_ref().unchecked_ref());
    closure.forget();
}

/// Event listener for blur events
pub fn on_blur<F>(element: &Element, handler: F)
where
    F: Fn(web_sys::FocusEvent) + 'static,
{
    let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn(_)>);
    let _ = element.add_event_listener_with_callback("blur", closure.as_ref().unchecked_ref());
    closure.forget();
}
