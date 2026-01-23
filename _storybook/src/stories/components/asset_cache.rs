//! Asset Cache component story
//!
//! Demonstrates the asset-cache component preloading images and
//! validates that cached images load instantly in asset-cards.

use crate::stories::helpers::render_attribute_card;
use futures_signals::signal::{Mutable, SignalExt};
use primitives::{bind_text_content, create_element, document, AppendChild};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

/// Sample Pirate assets for testing
const TEST_ASSETS: &[(&str, &str)] = &[
    // policy_id, asset_name_hex (Pirate 189, 200, 333, 434, 535)
    (
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6",
        "506972617465313839",
    ),
    (
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6",
        "506972617465323030",
    ),
    (
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6",
        "506972617465333333",
    ),
    (
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6",
        "506972617465343334",
    ),
    (
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6",
        "506972617465353335",
    ),
];

pub fn render_asset_cache_story() -> Element {
    // Register components
    ui_components::define_all();

    let container = create_element("div", &[]);

    // Header
    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Asset Cache"));
    header.append(&h2);

    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Non-visual component that preloads NFT images into a global cache. \
         Cached images are stored as blob URLs and used by <asset-card> and <image-card> \
         for instant display without network requests.",
    ));
    header.append(&desc);
    container.append(&header);

    // Section: Demo
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Preload Demo"));
    section.append(&h3);

    let demo_desc = create_element("p", &[]);
    demo_desc.set_text_content(Some(
        "Click 'Start Preload' to cache 5 images. Watch the status update as images load. \
         Then click 'Show Cards' to see them render instantly from cache.",
    ));
    section.append(&demo_desc);

    let canvas = create_element("div", &["story-canvas"]);

    // Status display
    let status = Mutable::new("Ready to preload".to_string());
    let cache_size = Mutable::new(0usize);
    let cards_visible = Mutable::new(false);
    let preload_started = Mutable::new(false);

    let status_row = create_element("div", &[]);
    status_row
        .set_attribute(
            "style",
            "display: flex; gap: 1rem; align-items: center; margin-bottom: 1rem;",
        )
        .unwrap();

    let status_label = create_element("span", &["status-indicator"]);
    bind_text_content(&status_label, status.signal_cloned());
    status_row.append(&status_label);

    let cache_label = create_element("span", &["status-indicator", "status-indicator--connected"]);
    bind_text_content(
        &cache_label,
        cache_size
            .signal()
            .map(|size| format!("Cache: {size} images")),
    );
    status_row.append(&cache_label);

    canvas.append(&status_row);

    // Buttons row
    let btn_row = create_element("div", &[]);
    btn_row
        .set_attribute("style", "display: flex; gap: 0.5rem; margin-bottom: 1rem;")
        .unwrap();

    // Start Preload button
    let preload_btn: HtmlElement = create_element("button", &["btn"]).unchecked_into();
    preload_btn.set_text_content(Some("Start Preload"));
    preload_btn
        .style()
        .set_property("padding", "0.5rem 1rem")
        .unwrap();

    // Show Cards button
    let show_btn: HtmlElement = create_element("button", &["btn"]).unchecked_into();
    show_btn.set_text_content(Some("Show Cards"));
    show_btn
        .style()
        .set_property("padding", "0.5rem 1rem")
        .unwrap();

    // Clear Cache button
    let clear_btn: HtmlElement = create_element("button", &["btn"]).unchecked_into();
    clear_btn.set_text_content(Some("Reset"));
    clear_btn
        .style()
        .set_property("padding", "0.5rem 1rem")
        .unwrap();

    btn_row.append(&preload_btn);
    btn_row.append(&show_btn);
    btn_row.append(&clear_btn);
    canvas.append(&btn_row);

    // Asset cache element (hidden, non-visual)
    let cache_el: HtmlElement = document()
        .create_element("asset-cache")
        .unwrap()
        .unchecked_into();

    // Listen for cache events
    {
        let status = status.clone();
        let closure =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::CustomEvent| {
                if let Ok(detail) = js_sys::Reflect::get(&e.detail(), &"loaded".into()) {
                    if let Ok(total) = js_sys::Reflect::get(&e.detail(), &"total".into()) {
                        let loaded = detail.as_f64().unwrap_or(0.0) as u32;
                        let total = total.as_f64().unwrap_or(0.0) as u32;
                        status.set(format!("Loading: {loaded}/{total}"));
                    }
                }
            }) as Box<dyn Fn(_)>);
        cache_el
            .add_event_listener_with_callback("cache-progress", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let status = status.clone();
        let cache_size = cache_size.clone();
        let closure =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::CustomEvent| {
                if let Ok(loaded) = js_sys::Reflect::get(&e.detail(), &"loaded".into()) {
                    if let Ok(failed) = js_sys::Reflect::get(&e.detail(), &"failed".into()) {
                        let loaded = loaded.as_f64().unwrap_or(0.0) as u32;
                        let failed = failed.as_f64().unwrap_or(0.0) as u32;
                        status.set(format!("Ready! {loaded} loaded, {failed} failed"));
                        cache_size.set(ui_components::image_cache::cache_size());
                    }
                }
            }) as Box<dyn Fn(_)>);
        cache_el
            .add_event_listener_with_callback("cache-ready", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    canvas.append_child(&cache_el).unwrap();

    // Card container (initially hidden)
    let card_container: HtmlElement = create_element("div", &[]).unchecked_into();
    card_container
        .set_attribute(
            "style",
            "display: none; gap: 1rem; flex-wrap: wrap; margin-top: 1rem;",
        )
        .unwrap();

    // Pre-create the asset cards
    for (i, (policy_id, asset_name_hex)) in TEST_ASSETS.iter().enumerate() {
        let card = document().create_element("asset-card").unwrap();
        let asset_id = format!("{policy_id}{asset_name_hex}");
        card.set_attribute("asset-id", &asset_id).unwrap();
        card.set_attribute("name", &format!("Pirate #{}", i + 1))
            .unwrap();
        card.set_attribute("size", "sm").unwrap();
        card.set_attribute("show-name", "").unwrap();
        card_container.append_child(&card).unwrap();
    }

    canvas.append(&card_container);

    // Button handlers
    {
        let cache_el = cache_el.clone();
        let status = status.clone();
        let preload_started = preload_started.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
            if preload_started.get() {
                return;
            }
            preload_started.set(true);
            status.set("Starting preload...".to_string());

            // Build JSON array of assets
            let assets_json: Vec<String> = TEST_ASSETS
                .iter()
                .map(|(p, a)| format!(r#"{{"policy_id":"{}","asset_name_hex":"{}"}}"#, p, a))
                .collect();
            let json = format!("[{}]", assets_json.join(","));

            cache_el.set_attribute("assets", &json).unwrap();
        }) as Box<dyn Fn(_)>);
        preload_btn
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let card_container = card_container.clone();
        let cards_visible = cards_visible.clone();
        let cache_size = cache_size.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
            cards_visible.set(true);
            card_container
                .style()
                .set_property("display", "flex")
                .unwrap();
            // Update cache size display
            cache_size.set(ui_components::image_cache::cache_size());
        }) as Box<dyn Fn(_)>);
        show_btn
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let card_container = card_container.clone();
        let status = status.clone();
        let cache_size = cache_size.clone();
        let preload_started = preload_started.clone();
        let cache_el = cache_el.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
            // Hide cards
            card_container
                .style()
                .set_property("display", "none")
                .unwrap();
            // Reset state
            status.set("Ready to preload".to_string());
            cache_size.set(0);
            preload_started.set(false);
            // Clear the assets attribute
            cache_el.remove_attribute("assets").unwrap_or(());
            // Note: can't actually clear the global cache, but for demo purposes
            // the user can reload the page
        }) as Box<dyn Fn(_)>);
        clear_btn
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    section.append(&canvas);
    container.append(&section);

    // Section: How It Works
    let how_section = create_element("div", &["story-section"]);
    let how_h3 = create_element("h3", &[]);
    how_h3.set_text_content(Some("How It Works"));
    how_section.append(&how_h3);

    let how_canvas = create_element("div", &["story-canvas"]);
    let how_grid = create_element("div", &["story-grid"]);

    let step1 = render_attribute_card(
        "1. Preload",
        "<asset-cache>",
        "Set the assets attribute with a JSON array of asset IDs. The component fetches each image.",
    );
    how_grid.append(&step1);

    let step2 = render_attribute_card(
        "2. Store",
        "Blob URLs",
        "Images are stored in a global cache as blob URLs, keeping them in memory.",
    );
    how_grid.append(&step2);

    let step3 = render_attribute_card(
        "3. Use",
        "<asset-card> / <image-card>",
        "When rendering, cards check the cache first. Cached images display instantly.",
    );
    how_grid.append(&step3);

    how_canvas.append(&how_grid);
    how_section.append(&how_canvas);
    container.append(&how_section);

    // Section: Attributes
    let attrs_section = create_element("div", &["story-section"]);
    let attrs_h3 = create_element("h3", &[]);
    attrs_h3.set_text_content(Some("Attributes"));
    attrs_section.append(&attrs_h3);

    let attrs_canvas = create_element("div", &["story-canvas"]);
    let attrs_grid = create_element("div", &["story-grid"]);

    let attr1 = render_attribute_card(
        "assets",
        "JSON array",
        r#"Array of asset objects: [{"policy_id": "...", "asset_name_hex": "..."}]"#,
    );
    attrs_grid.append(&attr1);

    attrs_canvas.append(&attrs_grid);
    attrs_section.append(&attrs_canvas);
    container.append(&attrs_section);

    // Section: Events
    let events_section = create_element("div", &["story-section"]);
    let events_h3 = create_element("h3", &[]);
    events_h3.set_text_content(Some("Events"));
    events_section.append(&events_h3);

    let events_canvas = create_element("div", &["story-canvas"]);
    let events_grid = create_element("div", &["story-grid"]);

    let event1 = render_attribute_card(
        "cache-progress",
        "CustomEvent",
        "Dispatched as each image loads. Detail: { loaded: number, total: number }",
    );
    events_grid.append(&event1);

    let event2 = render_attribute_card(
        "cache-ready",
        "CustomEvent",
        "Dispatched when all images finish. Detail: { loaded: number, failed: number }",
    );
    events_grid.append(&event2);

    events_canvas.append(&events_grid);
    events_section.append(&events_canvas);
    container.append(&events_section);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Usage"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r##"// Register components at app startup
ui_components::define_all();

// Preload assets before showing them
<asset-cache
    assets='[
        {"policy_id":"abc...","asset_name_hex":"def..."},
        {"policy_id":"abc...","asset_name_hex":"ghi..."}
    ]'
    on:cache-progress=|e| {
        let loaded = e.detail().get("loaded");
        let total = e.detail().get("total");
        show_progress(loaded, total);
    }
    on:cache-ready=|e| {
        let loaded = e.detail().get("loaded");
        // Now safe to show cards - they'll load instantly
        show_game_board();
    }
/>

// Cards will use cached blob URLs automatically
<asset-card
    asset-id="{policy_id}{asset_name_hex}"
    name="Pirate #1"
    show-name
/>"##,
    ));
    code_section.append(&code);
    container.append(&code_section);

    // API section
    let api_section = create_element("div", &["story-section"]);
    let api_h3 = create_element("h3", &[]);
    api_h3.set_text_content(Some("Programmatic API"));
    api_section.append(&api_h3);

    let api_code = create_element("pre", &["code-block"]);
    api_code.set_text_content(Some(
        r##"use ui_components::image_cache;

// Check cache size
let count = image_cache::cache_size();

// Check if a specific URL is cached
if let Some(blob_url) = image_cache::get_cached_url(&image_url) {
    // Use the blob URL directly
}

// Manually preload an image (async)
let blob_url = image_cache::preload_image(url).await?;"##,
    ));
    api_section.append(&api_code);
    container.append(&api_section);

    container
}
