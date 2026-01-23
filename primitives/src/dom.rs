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

/// Generic event listener for any event type
///
/// # Example
/// ```ignore
/// on_event(&element, "custom-event", |_: web_sys::Event| {
///     // handle event
/// });
/// ```
pub fn on_event<F>(element: &Element, event_name: &str, handler: F)
where
    F: Fn(web_sys::Event) + 'static,
{
    let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn(_)>);
    let _ = element.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
    closure.forget();
}

/// Forward an event from a source element to a target element, optionally renaming it.
///
/// This is useful for bubbling events up through nested web components.
///
/// # Example
/// ```ignore
/// // Forward "image-loaded" from nested component as "card-loaded" on host
/// forward_event(&nested_element, "image-loaded", &host_element, "card-loaded");
/// ```
pub fn forward_event(source: &Element, source_event: &str, target: &Element, target_event: &str) {
    use crate::dispatch_event;

    let target = target.clone();
    let target_event = target_event.to_string();

    on_event(source, source_event, move |_| {
        dispatch_event(&target, &target_event);
    });
}

/// Event listener for image load events with cached image handling.
///
/// Dispatches immediately if the image is already loaded (from cache),
/// otherwise waits for the load event.
///
/// # Example
/// ```ignore
/// on_image_load(&img_element, || {
///     dispatch_event(&host, "image-loaded");
/// });
/// ```
pub fn on_image_load<F>(img: &web_sys::HtmlImageElement, handler: F)
where
    F: Fn() + Clone + 'static,
{
    // Set up load event listener
    let handler_clone = handler.clone();
    let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
        handler_clone();
    }) as Box<dyn Fn(_)>);

    let _ = img.add_event_listener_with_callback("load", closure.as_ref().unchecked_ref());
    closure.forget();

    // Check if already loaded (cached image)
    if img.complete() && img.natural_width() > 0 {
        handler();
    }
}

/// Try to get an image element from a generic element and set up load handling.
///
/// Returns true if the element was an image and the handler was set up.
///
/// # Example
/// ```ignore
/// if let Ok(Some(img)) = shadow.query_selector(".image-card__image") {
///     setup_image_load_handler(&img, || {
///         dispatch_event(&host, "image-loaded");
///     });
/// }
/// ```
pub fn setup_image_load_handler<F>(element: &Element, handler: F) -> bool
where
    F: Fn() + Clone + 'static,
{
    if let Some(img) = element.dyn_ref::<web_sys::HtmlImageElement>() {
        on_image_load(img, handler);
        true
    } else {
        false
    }
}
