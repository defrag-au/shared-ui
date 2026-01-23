//! UI Loader stories - loading states and configuration

use super::helpers::{
    render_config_option_card, render_loader_error_card, render_loader_step_card,
};
use primitives::{create_element, AppendChild};
use web_sys::Element;

// ============================================================================
// Loading States Story
// ============================================================================

pub fn render_loading_states_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Loading States"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "The ui-loader crate provides a framework-agnostic loading orchestrator for WASM widgets.",
    ));
    header.append(&desc);
    container.append(&header);

    // Overview section
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Loading Flow"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &["story-grid"]);

    // Step 1
    let card1 = render_loader_step_card(
        "1",
        "Show Loading",
        "Immediately display a loading screen with customizable message",
    );
    grid.append(&card1);

    // Step 2
    let card2 = render_loader_step_card(
        "2",
        "Validate Auth",
        "Parse URL token and validate JWT authentication",
    );
    grid.append(&card2);

    // Step 3
    let card3 = render_loader_step_card(
        "3",
        "Fetch Data",
        "Run async load function with progress updates",
    );
    grid.append(&card3);

    // Step 4
    let card4 = render_loader_step_card(
        "4",
        "Hand Off",
        "Pass loaded data to your framework (Seed, Leptos, etc.)",
    );
    grid.append(&card4);

    canvas.append(&grid);
    section.append(&canvas);
    container.append(&section);

    // Error states section
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Error States"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let grid2 = create_element("div", &["story-grid"]);

    // Auth Required
    let err1 = render_loader_error_card("AuthRequired", "Missing or invalid authentication token");
    grid2.append(&err1);

    // Token Expired
    let err2 = render_loader_error_card("TokenExpired", "JWT token has passed its expiration time");
    grid2.append(&err2);

    // Fetch Failed
    let err3 = render_loader_error_card(
        "FetchFailed",
        "Network error or API returned an error status",
    );
    grid2.append(&err3);

    canvas2.append(&grid2);
    section2.append(&canvas2);
    container.append(&section2);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Usage"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r#"use ui_loader::{LoadingOrchestrator, LoaderConfig};

#[wasm_bindgen(start)]
pub async fn start() {
    let config = LoaderConfig::new()
        .auth_required(true)
        .initial_message("Loading...");

    let result = LoadingOrchestrator::run(config, |auth, loader| async move {
        loader.set_message("Fetching data...");
        let data = fetch_data(&auth).await?;
        Ok(data)
    }).await;

    match result {
        Ok(loaded) => {
            // Start your UI framework with loaded.data
        }
        Err(_) => {
            // Error screen already shown
        }
    }
}"#,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}

// ============================================================================
// Loader Config Story
// ============================================================================

pub fn render_loader_config_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Loader Configuration"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Configure the loading orchestrator behavior with LoaderConfig.",
    ));
    header.append(&desc);
    container.append(&header);

    // Options section
    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Configuration Options"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &["story-grid"]);

    // auth_required
    let opt1 = render_config_option_card(
        "auth_required",
        "bool",
        "true",
        "Whether authentication is required. If true, shows error for missing/invalid tokens.",
    );
    grid.append(&opt1);

    // log_level
    let opt2 = render_config_option_card(
        "log_level",
        "Level",
        "DEBUG",
        "Tracing log level for the widget runtime (DEBUG, INFO, WARN, ERROR).",
    );
    grid.append(&opt2);

    // initial_message
    let opt3 = render_config_option_card(
        "initial_message",
        "String",
        "\"Loading...\"",
        "The message shown on the loading screen before fetch starts.",
    );
    grid.append(&opt3);

    // on_before_load
    let opt4 = render_config_option_card(
        "on_before_load",
        "fn()",
        "None",
        "Optional hook called after auth validation, before the load function runs.",
    );
    grid.append(&opt4);

    canvas.append(&grid);
    section.append(&canvas);
    container.append(&section);

    // LoadResult section
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("LoadResult Fields"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let grid2 = create_element("div", &["story-grid"]);

    let field1 = render_config_option_card(
        "auth",
        "AuthState",
        "-",
        "The validated authentication state (Authenticated, Anonymous, etc.)",
    );
    grid2.append(&field1);

    let field2 = render_config_option_card(
        "data",
        "T",
        "-",
        "Your loaded data returned from the fetch function",
    );
    grid2.append(&field2);

    let field3 = render_config_option_card(
        "world_id",
        "Option<String>",
        "-",
        "World ID extracted from URL ?world= parameter",
    );
    grid2.append(&field3);

    let field4 = render_config_option_card(
        "discord_url",
        "Option<String>",
        "-",
        "Return URL from JWT claims for 'Return to Discord' button",
    );
    grid2.append(&field4);

    canvas2.append(&grid2);
    section2.append(&canvas2);
    container.append(&section2);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Builder Pattern"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r#"use ui_loader::{LoaderConfig, Level};

let config = LoaderConfig::new()
    .auth_required(true)
    .log_level(Level::INFO)
    .initial_message("Starting up...")
    .on_before_load(|| {
        // Register custom elements, etc.
    });"#,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}
