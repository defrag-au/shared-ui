//! Asset Cache Web Component
//!
//! A non-visual component that preloads NFT asset images in the background.
//! When images are preloaded, they will be served from browser cache when
//! displayed by `<asset-card>` or `<image-card>` components.
//!
//! ## Attributes
//!
//! - `assets` - JSON array of asset IDs to preload (each as `{policy_id, asset_name_hex}`)
//!
//! ## Events
//!
//! - `cache-ready` - Dispatched when all images have been preloaded (detail: { loaded, failed })
//! - `cache-progress` - Dispatched as each image loads (detail: { loaded, total })
//!
//! ## Usage
//!
//! ```html
//! <!-- Preload assets for a memory game -->
//! <asset-cache
//!     assets='[{"policy_id":"abc...","asset_name_hex":"def..."}]'
//!     on:cache-ready="handleReady">
//! </asset-cache>
//! ```

use custom_elements::CustomElement;
use js_sys::{Array, Object, Reflect};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

/// Asset cache custom element - preloads images in background
#[derive(Default)]
pub struct AssetCache;

impl AssetCache {
    /// Register the custom element. Call once at app startup.
    pub fn define() {
        <Self as CustomElement>::define("asset-cache");
    }

    /// Build the IIIF thumbnail URL for an asset (400px thumb)
    fn build_image_url(policy_id: &str, asset_name_hex: &str) -> String {
        let full_id = format!("{policy_id}{asset_name_hex}");
        format!("https://img.pfp.city/{policy_id}/iiif/{full_id}/full/400,/0/default.png")
    }

    /// Dispatch a custom event with detail object
    fn dispatch_detail_event(element: &HtmlElement, event_name: &str, detail: &JsValue) {
        let init = web_sys::CustomEventInit::new();
        init.set_detail(detail);
        init.set_bubbles(true);
        init.set_composed(true);

        if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict(event_name, &init) {
            let _ = element.dispatch_event(&event);
        }
    }

    /// Create a JS object with the given properties
    fn make_detail(props: &[(&str, u32)]) -> JsValue {
        let obj = Object::new();
        for (key, value) in props {
            let _ = Reflect::set(&obj, &JsValue::from_str(key), &JsValue::from(*value));
        }
        obj.into()
    }

    /// Parse assets from JSON attribute and start preloading
    fn preload_assets(&self, element: &HtmlElement, assets_json: &str) {
        let host = element.clone();

        // Parse the JSON array
        let parsed: JsValue = match js_sys::JSON::parse(assets_json) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("Failed to parse assets JSON: {:?}", e);
                return;
            }
        };

        let assets = match parsed.dyn_ref::<Array>() {
            Some(arr) => arr.clone(),
            None => {
                tracing::warn!("Assets attribute must be a JSON array");
                return;
            }
        };

        let total = assets.length();
        if total == 0 {
            // Nothing to preload, emit ready immediately
            Self::dispatch_detail_event(
                &host,
                "cache-ready",
                &Self::make_detail(&[("loaded", 0), ("failed", 0)]),
            );
            return;
        }

        let loaded = Rc::new(RefCell::new(0u32));
        let failed = Rc::new(RefCell::new(0u32));

        for i in 0..total {
            let asset = assets.get(i);

            // Extract policy_id and asset_name_hex from the object
            let policy_id = Reflect::get(&asset, &JsValue::from_str("policy_id"))
                .ok()
                .and_then(|v| v.as_string());
            let asset_name_hex = Reflect::get(&asset, &JsValue::from_str("asset_name_hex"))
                .ok()
                .and_then(|v| v.as_string());

            let (policy_id, asset_name_hex) = match (policy_id, asset_name_hex) {
                (Some(p), Some(a)) => (p, a),
                _ => {
                    *failed.borrow_mut() += 1;
                    continue;
                }
            };

            let url = Self::build_image_url(&policy_id, &asset_name_hex);
            let host = host.clone();
            let loaded = Rc::clone(&loaded);
            let failed = Rc::clone(&failed);

            // Create an Image element to preload
            let window = web_sys::window().expect("no window");
            let document = window.document().expect("no document");
            let img: web_sys::HtmlImageElement = document
                .create_element("img")
                .expect("create img")
                .dyn_into()
                .expect("cast to HtmlImageElement");

            // Set up load handler
            let host_load = host.clone();
            let loaded_load = Rc::clone(&loaded);
            let failed_load = Rc::clone(&failed);
            let on_load = Closure::wrap(Box::new(move || {
                *loaded_load.borrow_mut() += 1;
                let current_loaded = *loaded_load.borrow();
                let current_failed = *failed_load.borrow();

                // Emit progress
                Self::dispatch_detail_event(
                    &host_load,
                    "cache-progress",
                    &Self::make_detail(&[("loaded", current_loaded), ("total", total)]),
                );

                // Check if all done
                if current_loaded + current_failed >= total {
                    Self::dispatch_detail_event(
                        &host_load,
                        "cache-ready",
                        &Self::make_detail(&[
                            ("loaded", current_loaded),
                            ("failed", current_failed),
                        ]),
                    );
                }
            }) as Box<dyn Fn()>);

            // Set up error handler
            let host_error = host.clone();
            let loaded_error = Rc::clone(&loaded);
            let failed_error = Rc::clone(&failed);
            let on_error = Closure::wrap(Box::new(move || {
                *failed_error.borrow_mut() += 1;
                let current_loaded = *loaded_error.borrow();
                let current_failed = *failed_error.borrow();

                // Check if all done
                if current_loaded + current_failed >= total {
                    Self::dispatch_detail_event(
                        &host_error,
                        "cache-ready",
                        &Self::make_detail(&[
                            ("loaded", current_loaded),
                            ("failed", current_failed),
                        ]),
                    );
                }
            }) as Box<dyn Fn()>);

            img.set_onload(Some(on_load.as_ref().unchecked_ref()));
            img.set_onerror(Some(on_error.as_ref().unchecked_ref()));

            // Leak closures - they need to live until the image loads
            on_load.forget();
            on_error.forget();

            // Start loading
            img.set_src(&url);
        }
    }
}

impl CustomElement for AssetCache {
    fn observed_attributes() -> &'static [&'static str] {
        &["assets"]
    }

    fn constructor(&mut self, _this: &HtmlElement) {
        // No-op - assets will be set via attribute
    }

    fn attribute_changed_callback(
        &mut self,
        this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if name == "assets" {
            if let Some(json) = new_value {
                if !json.is_empty() {
                    self.preload_assets(this, &json);
                }
            }
        }
    }

    fn inject_children(&mut self, _this: &HtmlElement) {
        // No children to inject - this is a non-visual component
        // Assets will be loaded via attribute_changed_callback
    }
}
