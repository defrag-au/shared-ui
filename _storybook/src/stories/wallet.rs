//! Wallet stories - wallet providers and connection states

use leptos::*;
use wallet_core::{ConnectionState, Network, WalletProvider};

// ============================================================================
// Wallet Providers Story
// ============================================================================

#[component]
pub fn WalletProvidersStory() -> impl IntoView {
    view! {
        <div>
            <div class="story-header">
                <h2>"Wallet Providers"</h2>
                <p>"Supported Cardano wallet providers for CIP-30 integration."</p>
            </div>

            <div class="story-section">
                <h3>"Supported Wallets"</h3>
                <div class="story-canvas">
                    <div class="story-grid">
                        {WalletProvider::all().iter().map(|provider| {
                            view! {
                                <div class="wallet-card">
                                    <div class="wallet-card__header">
                                        <div class="wallet-card__icon">{wallet_icon(*provider)}</div>
                                        <span class="wallet-card__name">{provider.display_name()}</span>
                                    </div>
                                    <div class="wallet-card__body">
                                        <div class="wallet-card__row">
                                            <span class="wallet-card__label">"API Name"</span>
                                            <span class="wallet-card__value">{provider.api_name()}</span>
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>

            <div class="story-section">
                <h3>"Usage"</h3>
                <pre class="code-block">{r#"use wallet_core::WalletProvider;

// Get all providers
for provider in WalletProvider::all() {
    println!("{}: {}", provider.display_name(), provider.api_name());
}

// Check specific provider
let nami = WalletProvider::Nami;
assert_eq!(nami.api_name(), "nami");
assert_eq!(nami.display_name(), "Nami");"#}</pre>
            </div>
        </div>
    }
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

#[component]
pub fn ConnectionStatesStory() -> impl IntoView {
    let states = vec![
        ConnectionState::Disconnected,
        ConnectionState::Connecting,
        ConnectionState::Connected {
            provider: WalletProvider::Eternl,
            address: "addr1qx...abc123".to_string(),
            network: Network::Mainnet,
        },
        ConnectionState::Error("User rejected connection".to_string()),
    ];

    view! {
        <div>
            <div class="story-header">
                <h2>"Connection States"</h2>
                <p>"Visual representation of wallet connection states."</p>
            </div>

            <div class="story-section">
                <h3>"State Indicators"</h3>
                <div class="story-canvas">
                    <div class="story-inline">
                        <span class="status-indicator status-indicator--disconnected">"Disconnected"</span>
                        <span class="status-indicator status-indicator--connecting">"Connecting..."</span>
                        <span class="status-indicator status-indicator--connected">"Connected"</span>
                    </div>
                </div>
            </div>

            <div class="story-section">
                <h3>"Connection State Examples"</h3>
                <div class="story-canvas">
                    <div class="story-grid">
                        {states.into_iter().map(|state| {
                            view! { <ConnectionCard state=state /> }
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ConnectionCard(state: ConnectionState) -> impl IntoView {
    let (status_class, status_text) = match &state {
        ConnectionState::Disconnected => ("status-indicator--disconnected", "Disconnected"),
        ConnectionState::Connecting => ("status-indicator--connecting", "Connecting"),
        ConnectionState::Connected { .. } => ("status-indicator--connected", "Connected"),
        ConnectionState::Error(_) => ("status-indicator--disconnected", "Error"),
    };

    view! {
        <div class="wallet-card">
            <div class="wallet-card__header">
                <span class=format!("status-indicator {status_class}")>{status_text}</span>
            </div>
            <div class="wallet-card__body">
                {match state {
                    ConnectionState::Connected { provider, address, network } => {
                        let network_name = match network {
                            Network::Mainnet => "Mainnet",
                            Network::Preprod => "Preprod",
                            Network::Preview => "Preview",
                        };
                        view! {
                            <>
                                <div class="wallet-card__row">
                                    <span class="wallet-card__label">"Provider"</span>
                                    <span class="wallet-card__value">{provider.display_name()}</span>
                                </div>
                                <div class="wallet-card__row">
                                    <span class="wallet-card__label">"Address"</span>
                                    <span class="wallet-card__value">{address}</span>
                                </div>
                                <div class="wallet-card__row">
                                    <span class="wallet-card__label">"Network"</span>
                                    <span class="wallet-card__value">{network_name}</span>
                                </div>
                            </>
                        }.into_view()
                    }
                    ConnectionState::Error(msg) => {
                        view! {
                            <div class="wallet-card__row">
                                <span class="wallet-card__label">"Error"</span>
                                <span class="wallet-card__value">{msg}</span>
                            </div>
                        }.into_view()
                    }
                    _ => {
                        view! {
                            <p>"No additional details"</p>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
