//! Base traits and helpers for web components

use web_sys::{Element, HtmlElement, ShadowRoot, ShadowRootInit, ShadowRootMode};

/// Trait for defining a custom element
pub trait WebComponent: Sized {
    /// The tag name for this component (e.g., "my-component")
    const TAG_NAME: &'static str;

    /// Create a new instance of this component
    fn new() -> Self;

    /// Called when the element is connected to the DOM
    fn connected_callback(&self, element: &HtmlElement);

    /// Called when the element is disconnected from the DOM
    fn disconnected_callback(&self, _element: &HtmlElement) {}

    /// Called when an observed attribute changes
    fn attribute_changed_callback(
        &self,
        _element: &HtmlElement,
        _name: &str,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
    }

    /// List of attributes to observe for changes
    fn observed_attributes() -> Vec<&'static str> {
        vec![]
    }
}

/// Create a shadow root with open mode
pub fn create_shadow_root(element: &HtmlElement) -> ShadowRoot {
    let init = ShadowRootInit::new(ShadowRootMode::Open);
    element.attach_shadow(&init).unwrap()
}

/// Helper to get or create shadow root
pub fn get_or_create_shadow(element: &HtmlElement) -> ShadowRoot {
    element
        .shadow_root()
        .unwrap_or_else(|| create_shadow_root(element))
}

/// Dispatch a custom event from an element
pub fn dispatch_event(element: &Element, event_name: &str) {
    let event = web_sys::Event::new(event_name).unwrap();
    let _ = element.dispatch_event(&event);
}

/// Dispatch a custom event with detail data
pub fn dispatch_custom_event(element: &Element, event_name: &str, detail: &wasm_bindgen::JsValue) {
    let init = web_sys::CustomEventInit::new();
    init.set_detail(detail);
    init.set_bubbles(true);
    init.set_composed(true); // crosses shadow DOM boundary

    let event = web_sys::CustomEvent::new_with_event_init_dict(event_name, &init).unwrap();
    let _ = element.dispatch_event(&event);
}
