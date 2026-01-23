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

/// Normalize the element passed to custom element callbacks.
///
/// The `custom-elements` crate's JS shim has a quirk: when shadow mode is enabled,
/// it passes `this.shadowRoot` (a ShadowRoot) to `_injectChildren` instead of `this`
/// (the host element). Due to JS's loose typing, Rust receives this as an HtmlElement.
///
/// This helper detects which case we're in and returns both the shadow root and
/// the actual host element, regardless of what was passed.
///
/// # Usage
/// ```ignore
/// fn inject_children(&mut self, this: &HtmlElement) {
///     let (shadow, host) = get_shadow_and_host(this);
///     shadow.set_inner_html(&self.render());
///     // Use `host` to dispatch events, read attributes, etc.
/// }
/// ```
pub fn get_shadow_and_host(element: &HtmlElement) -> (ShadowRoot, HtmlElement) {
    use wasm_bindgen::JsCast;

    if let Some(shadow) = element.shadow_root() {
        // element is the host - normal case
        (shadow, element.clone())
    } else {
        // element is actually a ShadowRoot (custom-elements JS shim quirk)
        let shadow: &ShadowRoot = element.unchecked_ref();
        let host: HtmlElement = shadow.host().unchecked_into();
        (shadow.clone(), host)
    }
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
