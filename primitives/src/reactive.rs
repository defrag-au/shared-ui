//! Minimal reactive DOM bindings using futures-signals
//!
//! This module provides Lit-like fine-grained reactivity for web components.
//! Instead of replacing innerHTML on every change, we create DOM nodes once
//! and bind signals to update only the parts that change.
//!
//! ## Example
//!
//! ```ignore
//! use futures_signals::signal::Mutable;
//! use primitives::reactive::*;
//!
//! let name = Mutable::new("World".to_string());
//!
//! // Create a text node that updates when `name` changes
//! let text = text_signal(name.signal_cloned());
//! parent.append_child(&text);
//!
//! // Later: only the text node updates, nothing else
//! name.set("Rust".to_string());
//! ```

use futures_signals::signal::{Signal, SignalExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Document, Element, HtmlElement, HtmlInputElement, Node, ShadowRoot, Text};

/// Get the document
pub fn document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

/// Create a text node bound to a signal - updates automatically when signal changes
pub fn text_signal<S>(signal: S) -> Text
where
    S: Signal<Item = String> + 'static,
{
    let text_node = document().create_text_node("");
    let text_node_clone = text_node.clone();

    spawn_local(signal.for_each(move |value| {
        text_node_clone.set_text_content(Some(&value));
        async {}
    }));

    text_node
}

/// Bind a signal to an element's text content
pub fn bind_text_content<S>(element: &Element, signal: S)
where
    S: Signal<Item = String> + 'static,
{
    let element = element.clone();
    spawn_local(signal.for_each(move |value| {
        element.set_text_content(Some(&value));
        async {}
    }));
}

/// Bind a signal to an element's attribute
pub fn bind_attr<S>(element: &Element, name: &str, signal: S)
where
    S: Signal<Item = String> + 'static,
{
    let element = element.clone();
    let name = name.to_string();
    spawn_local(signal.for_each(move |value| {
        let _ = element.set_attribute(&name, &value);
        async {}
    }));
}

/// Bind a signal to an element's attribute, removing it when None
pub fn bind_attr_option<S>(element: &Element, name: &str, signal: S)
where
    S: Signal<Item = Option<String>> + 'static,
{
    let element = element.clone();
    let name = name.to_string();
    spawn_local(signal.for_each(move |value| {
        match value {
            Some(v) => {
                let _ = element.set_attribute(&name, &v);
            }
            None => {
                let _ = element.remove_attribute(&name);
            }
        }
        async {}
    }));
}

/// Bind a signal to an input element's value property
pub fn bind_input_value<S>(input: &HtmlInputElement, signal: S)
where
    S: Signal<Item = String> + 'static,
{
    let input = input.clone();
    spawn_local(signal.for_each(move |value| {
        // Only update if different to avoid cursor position issues
        if input.value() != value {
            input.set_value(&value);
        }
        async {}
    }));
}

/// Bind a signal to toggle a CSS class
pub fn bind_class<S>(element: &Element, class_name: &str, signal: S)
where
    S: Signal<Item = bool> + 'static,
{
    let element = element.clone();
    let class_name = class_name.to_string();
    spawn_local(signal.for_each(move |enabled| {
        let class_list = element.class_list();
        if enabled {
            let _ = class_list.add_1(&class_name);
        } else {
            let _ = class_list.remove_1(&class_name);
        }
        async {}
    }));
}

/// Bind a signal to an element's visibility (display: none)
pub fn bind_visible<S>(element: &HtmlElement, signal: S)
where
    S: Signal<Item = bool> + 'static,
{
    let element = element.clone();
    spawn_local(signal.for_each(move |visible| {
        let style = element.style();
        let _ = style.set_property("display", if visible { "" } else { "none" });
        async {}
    }));
}

/// Clear all children from a node
pub trait ClearChildren {
    fn clear_children(&self);
}

impl ClearChildren for Element {
    fn clear_children(&self) {
        while let Some(child) = self.first_child() {
            let _ = self.remove_child(&child);
        }
    }
}

impl ClearChildren for ShadowRoot {
    fn clear_children(&self) {
        while let Some(child) = self.first_child() {
            let _ = self.remove_child(&child);
        }
    }
}

/// Append a child and return the parent for chaining
pub trait AppendChild {
    fn append<T: AsRef<Node>>(&self, child: &T) -> &Self;
    fn append_text(&self, text: &str) -> &Self;
}

impl AppendChild for Element {
    fn append<T: AsRef<Node>>(&self, child: &T) -> &Self {
        let _ = self.append_child(child.as_ref());
        self
    }

    fn append_text(&self, text: &str) -> &Self {
        let text_node = document().create_text_node(text);
        let _ = self.append_child(&text_node);
        self
    }
}

impl AppendChild for ShadowRoot {
    fn append<T: AsRef<Node>>(&self, child: &T) -> &Self {
        let _ = self.append_child(child.as_ref());
        self
    }

    fn append_text(&self, text: &str) -> &Self {
        let text_node = document().create_text_node(text);
        let _ = self.append_child(&text_node);
        self
    }
}

/// Set an attribute and return self for chaining
pub trait SetAttr {
    fn attr(&self, name: &str, value: &str) -> &Self;
    fn data(&self, name: &str, value: &str) -> &Self;
}

impl SetAttr for Element {
    fn attr(&self, name: &str, value: &str) -> &Self {
        let _ = self.set_attribute(name, value);
        self
    }

    fn data(&self, name: &str, value: &str) -> &Self {
        let _ = self.set_attribute(&format!("data-{name}"), value);
        self
    }
}

/// Inject a style element into a shadow root
pub fn inject_styles(shadow: &ShadowRoot, css: &str) {
    let style = document().create_element("style").unwrap();
    style.set_text_content(Some(css));
    let _ = shadow.append_child(&style);
}
