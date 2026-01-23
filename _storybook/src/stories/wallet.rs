//! Wallet stories - wallet providers and connection states

use primitives::{create_element, AppendChild};
use wallet_core::{ConnectionState, Network, WalletProvider};
use web_sys::Element;

// ============================================================================
// Wallet Providers Story
// ============================================================================

pub fn render_wallet_providers_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Wallet Providers"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some(
        "Supported Cardano wallet providers for CIP-30 integration.",
    ));
    header.append(&desc);
    container.append(&header);

    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("Supported Wallets"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &["story-grid"]);

    for provider in WalletProvider::all() {
        let card = create_element("div", &["wallet-card"]);

        let header = create_element("div", &["wallet-card__header"]);
        let icon = create_element("div", &["wallet-card__icon"]);
        icon.set_text_content(Some(wallet_icon(*provider)));
        header.append(&icon);

        let name = create_element("span", &["wallet-card__name"]);
        name.set_text_content(Some(provider.display_name()));
        header.append(&name);
        card.append(&header);

        let body = create_element("div", &["wallet-card__body"]);

        let row = create_element("div", &["wallet-card__row"]);
        let label = create_element("span", &["wallet-card__label"]);
        label.set_text_content(Some("API Name"));
        row.append(&label);
        let value = create_element("span", &["wallet-card__value"]);
        value.set_text_content(Some(provider.api_name()));
        row.append(&value);
        body.append(&row);

        card.append(&body);
        grid.append(&card);
    }

    canvas.append(&grid);
    section.append(&canvas);
    container.append(&section);

    // Code example
    let code_section = create_element("div", &["story-section"]);
    let code_h3 = create_element("h3", &[]);
    code_h3.set_text_content(Some("Usage"));
    code_section.append(&code_h3);

    let code = create_element("pre", &["code-block"]);
    code.set_text_content(Some(
        r#"use wallet_core::WalletProvider;

// Get all providers
for provider in WalletProvider::all() {
    println!("{}: {}", provider.display_name(), provider.api_name());
}

// Check specific provider
let nami = WalletProvider::Nami;
assert_eq!(nami.api_name(), "nami");
assert_eq!(nami.display_name(), "Nami");"#,
    ));
    code_section.append(&code);
    container.append(&code_section);

    container
}

fn wallet_icon(provider: WalletProvider) -> &'static str {
    match provider {
        WalletProvider::Nami => "N",
        WalletProvider::Eternl => "E",
        WalletProvider::Lace => "L",
        WalletProvider::Flint => "F",
        WalletProvider::Typhon => "T",
        WalletProvider::Vespr => "V",
        WalletProvider::NuFi => "Nu",
        WalletProvider::Gero => "G",
        WalletProvider::Yoroi => "Y",
    }
}

// ============================================================================
// Connection States Story
// ============================================================================

pub fn render_connection_states_story() -> Element {
    let container = create_element("div", &[]);

    let header = create_element("div", &["story-header"]);
    let h2 = create_element("h2", &[]);
    h2.set_text_content(Some("Connection States"));
    header.append(&h2);
    let desc = create_element("p", &[]);
    desc.set_text_content(Some("Visual representation of wallet connection states."));
    header.append(&desc);
    container.append(&header);

    let section = create_element("div", &["story-section"]);
    let h3 = create_element("h3", &[]);
    h3.set_text_content(Some("State Indicators"));
    section.append(&h3);

    let canvas = create_element("div", &["story-canvas"]);
    let inline = create_element("div", &["story-inline"]);

    // Disconnected
    let disc = create_element(
        "span",
        &["status-indicator", "status-indicator--disconnected"],
    );
    disc.set_text_content(Some("Disconnected"));
    inline.append(&disc);

    // Connecting
    let connecting = create_element(
        "span",
        &["status-indicator", "status-indicator--connecting"],
    );
    connecting.set_text_content(Some("Connecting..."));
    inline.append(&connecting);

    // Connected
    let connected = create_element("span", &["status-indicator", "status-indicator--connected"]);
    connected.set_text_content(Some("Connected"));
    inline.append(&connected);

    canvas.append(&inline);
    section.append(&canvas);
    container.append(&section);

    // Connection state cards
    let section2 = create_element("div", &["story-section"]);
    let h3_2 = create_element("h3", &[]);
    h3_2.set_text_content(Some("Connection State Examples"));
    section2.append(&h3_2);

    let canvas2 = create_element("div", &["story-canvas"]);
    let grid = create_element("div", &["story-grid"]);

    // Disconnected state
    let state1 = ConnectionState::Disconnected;
    grid.append(&render_connection_card(&state1));

    // Connecting state
    let state2 = ConnectionState::Connecting;
    grid.append(&render_connection_card(&state2));

    // Connected state
    let state3 = ConnectionState::Connected {
        provider: WalletProvider::Eternl,
        address: "addr1qx...abc123".to_string(),
        network: Network::Mainnet,
    };
    grid.append(&render_connection_card(&state3));

    // Error state
    let state4 = ConnectionState::Error("User rejected connection".to_string());
    grid.append(&render_connection_card(&state4));

    canvas2.append(&grid);
    section2.append(&canvas2);
    container.append(&section2);

    container
}

fn render_connection_card(state: &ConnectionState) -> Element {
    let card = create_element("div", &["wallet-card"]);

    let header = create_element("div", &["wallet-card__header"]);

    let (status_class, status_text) = match state {
        ConnectionState::Disconnected => ("status-indicator--disconnected", "Disconnected"),
        ConnectionState::Connecting => ("status-indicator--connecting", "Connecting"),
        ConnectionState::Connected { .. } => ("status-indicator--connected", "Connected"),
        ConnectionState::Error(_) => ("status-indicator--disconnected", "Error"),
    };

    let status = create_element("span", &["status-indicator", status_class]);
    status.set_text_content(Some(status_text));
    header.append(&status);
    card.append(&header);

    let body = create_element("div", &["wallet-card__body"]);

    match state {
        ConnectionState::Connected {
            provider,
            address,
            network,
        } => {
            // Provider row
            let row1 = create_element("div", &["wallet-card__row"]);
            let label1 = create_element("span", &["wallet-card__label"]);
            label1.set_text_content(Some("Provider"));
            row1.append(&label1);
            let value1 = create_element("span", &["wallet-card__value"]);
            value1.set_text_content(Some(provider.display_name()));
            row1.append(&value1);
            body.append(&row1);

            // Address row
            let row2 = create_element("div", &["wallet-card__row"]);
            let label2 = create_element("span", &["wallet-card__label"]);
            label2.set_text_content(Some("Address"));
            row2.append(&label2);
            let value2 = create_element("span", &["wallet-card__value"]);
            value2.set_text_content(Some(address));
            row2.append(&value2);
            body.append(&row2);

            // Network row
            let row3 = create_element("div", &["wallet-card__row"]);
            let label3 = create_element("span", &["wallet-card__label"]);
            label3.set_text_content(Some("Network"));
            row3.append(&label3);
            let value3 = create_element("span", &["wallet-card__value"]);
            value3.set_text_content(Some(match network {
                Network::Mainnet => "Mainnet",
                Network::Preprod => "Preprod",
                Network::Preview => "Preview",
            }));
            row3.append(&value3);
            body.append(&row3);
        }
        ConnectionState::Error(msg) => {
            let row = create_element("div", &["wallet-card__row"]);
            let label = create_element("span", &["wallet-card__label"]);
            label.set_text_content(Some("Error"));
            row.append(&label);
            let value = create_element("span", &["wallet-card__value"]);
            value.set_text_content(Some(msg));
            row.append(&value);
            body.append(&row);
        }
        _ => {
            let msg = create_element("p", &[]);
            msg.set_text_content(Some("No additional details"));
            body.append(&msg);
        }
    }

    card.append(&body);
    card
}
