//! Wallet stories - wallet providers, detection, and connection flow

use leptos::prelude::*;
use wallet_core::{
    detect_wallets, detect_wallets_with_info, ConnectionState, Network, WalletApi, WalletInfo,
    WalletProvider,
};
use wasm_bindgen_futures::spawn_local;

// ============================================================================
// Wallet Providers Story (Static Reference)
// ============================================================================

#[component]
pub fn WalletProvidersStory() -> impl IntoView {
    use leptos::prelude::CollectView;
    view! {
        <div>
            <div class="story-header">
                <h2>"Wallet Providers"</h2>
                <p>"Supported Cardano wallet providers for CIP-30 integration."</p>
            </div>

            <div class="story-section">
                <h3>"Supported Wallets"</h3>
                <p class="story-description">"These are all the wallet providers that wallet-core can detect and connect to."</p>
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
// Connection States Story (Static Reference)
// ============================================================================

#[component]
pub fn ConnectionStatesStory() -> impl IntoView {
    use leptos::prelude::CollectView;
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

            <div class="story-section">
                <h3>"Usage"</h3>
                <pre class="code-block">{r#"use wallet_core::ConnectionState;

let state = ConnectionState::Disconnected;

match state {
    ConnectionState::Disconnected => { /* Show connect button */ }
    ConnectionState::Connecting => { /* Show spinner */ }
    ConnectionState::Connected { provider, address, network } => {
        // Show connected wallet info
    }
    ConnectionState::Error(msg) => { /* Show error */ }
}"#}</pre>
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
        ConnectionState::Error(_) => ("status-indicator--error", "Error"),
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
                        }.into_any()
                    }
                    ConnectionState::Error(msg) => {
                        view! {
                            <div class="wallet-card__row">
                                <span class="wallet-card__label">"Error"</span>
                                <span class="wallet-card__value wallet-card__value--error">{msg}</span>
                            </div>
                        }.into_any()
                    }
                    _ => {
                        view! {
                            <p class="wallet-card__empty">"No additional details"</p>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

// ============================================================================
// Live Wallet Detection Story
// ============================================================================

#[component]
pub fn WalletDetectionStory() -> impl IntoView {
    let (detected_wallets, set_detected_wallets) = signal(Vec::<WalletInfo>::new());
    let (is_scanning, set_is_scanning) = signal(false);
    let (last_scan, set_last_scan) = signal(Option::<String>::None);

    // Scan for wallets
    let scan_wallets = move || {
        set_is_scanning.set(true);

        // Use the sync detection function
        let wallets = detect_wallets_with_info();
        set_detected_wallets.set(wallets);
        set_is_scanning.set(false);

        // Record scan time
        let now = js_sys::Date::new_0();
        let time_str = format!(
            "{:02}:{:02}:{:02}",
            now.get_hours(),
            now.get_minutes(),
            now.get_seconds()
        );
        set_last_scan.set(Some(time_str));
    };

    // Auto-scan on mount
    Effect::new(move |_| {
        scan_wallets();
    });

    view! {
        <div>
            <div class="story-header">
                <h2>"Live Wallet Detection"</h2>
                <p>"Detect Cardano wallets installed in your browser using CIP-30."</p>
            </div>

            <div class="story-section">
                <h3>"Detected Wallets"</h3>
                <div class="story-toolbar">
                    <button
                        class="btn btn--primary"
                        on:click=move |_| scan_wallets()
                        disabled=move || is_scanning.get()
                    >
                        {move || if is_scanning.get() { "Scanning..." } else { "Rescan" }}
                    </button>
                    {move || last_scan.get().map(|time| view! {
                        <span class="story-toolbar__info">"Last scan: " {time}</span>
                    })}
                </div>

                <div class="story-canvas">
                    {move || {
                        let wallets = detected_wallets.get();
                        if wallets.is_empty() {
                            view! {
                                <div class="wallet-empty-state">
                                    <p class="wallet-empty-state__title">"No wallets detected"</p>
                                    <p class="wallet-empty-state__hint">
                                        "Install a Cardano wallet extension like "
                                        <a href="https://eternl.io" target="_blank">"Eternl"</a>
                                        ", "
                                        <a href="https://namiwallet.io" target="_blank">"Nami"</a>
                                        ", or "
                                        <a href="https://www.lace.io" target="_blank">"Lace"</a>
                                        " to test wallet integration."
                                    </p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="story-grid">
                                    {wallets.into_iter().map(|wallet| {
                                        view! { <DetectedWalletCard wallet=wallet /> }
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </div>

            <div class="story-section">
                <h3>"Detection API"</h3>
                <pre class="code-block">{r#"use wallet_core::{detect_wallets, detect_wallets_with_info};

// Simple detection (just provider types)
let providers: Vec<WalletProvider> = detect_wallets();

// Detection with full info (name, icon from extension)
let wallets: Vec<WalletInfo> = detect_wallets_with_info();

for wallet in wallets {
    println!("Found: {} ({})", wallet.name, wallet.api_name);
    if let Some(icon) = wallet.icon {
        // icon is a base64 data URL
    }
}"#}</pre>
            </div>
        </div>
    }
}

#[component]
fn DetectedWalletCard(wallet: WalletInfo) -> impl IntoView {
    view! {
        <div class="wallet-card wallet-card--detected">
            <div class="wallet-card__header">
                {wallet.icon.as_ref().map(|icon| view! {
                    <img class="wallet-card__icon-img" src=icon.clone() alt=wallet.name.clone() />
                })}
                <span class="wallet-card__name">{wallet.name.clone()}</span>
                <span class="wallet-card__badge wallet-card__badge--success">"Installed"</span>
            </div>
            <div class="wallet-card__body">
                <div class="wallet-card__row">
                    <span class="wallet-card__label">"API Name"</span>
                    <span class="wallet-card__value">{wallet.api_name.clone()}</span>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// Wallet Connection Flow Story
// ============================================================================

#[component]
pub fn WalletConnectionStory() -> impl IntoView {
    let (connection_state, set_connection_state) = signal(ConnectionState::Disconnected);
    let (available_wallets, set_available_wallets) = signal(Vec::<WalletProvider>::new());
    let (connection_log, set_connection_log) = signal(Vec::<String>::new());

    // Helper to add log entry
    let add_log = move |msg: String| {
        set_connection_log.update(|log| {
            let now = js_sys::Date::new_0();
            let time_str = format!(
                "{:02}:{:02}:{:02}",
                now.get_hours(),
                now.get_minutes(),
                now.get_seconds()
            );
            log.push(format!("[{time_str}] {msg}"));
            // Keep last 20 entries
            if log.len() > 20 {
                log.remove(0);
            }
        });
    };

    // Detect wallets on mount
    Effect::new(move |_| {
        let wallets = detect_wallets();
        set_available_wallets.set(wallets.clone());
        add_log(format!("Detected {} wallet(s)", wallets.len()));
    });

    // Connect to a wallet
    let connect_to_wallet = move |provider: WalletProvider| {
        set_connection_state.set(ConnectionState::Connecting);
        add_log(format!("Connecting to {}...", provider.display_name()));

        spawn_local(async move {
            match WalletApi::connect(provider).await {
                Ok(api) => {
                    add_log(format!("Connected to {}", provider.display_name()));

                    // Get network ID
                    match api.network_id().await {
                        Ok(network_id) => {
                            let network = match network_id {
                                1 => Network::Mainnet,
                                _ => Network::Preprod,
                            };
                            add_log(format!("Network: {:?} (id={})", network, network_id));

                            // Get change address
                            match api.change_address().await {
                                Ok(address) => {
                                    let short_addr = if address.len() > 20 {
                                        format!(
                                            "{}...{}",
                                            &address[..10],
                                            &address[address.len() - 10..]
                                        )
                                    } else {
                                        address.clone()
                                    };
                                    add_log(format!("Address: {short_addr}"));

                                    set_connection_state.set(ConnectionState::Connected {
                                        provider,
                                        address,
                                        network,
                                    });
                                }
                                Err(e) => {
                                    add_log(format!("Failed to get address: {e}"));
                                    set_connection_state.set(ConnectionState::Error(e.to_string()));
                                }
                            }
                        }
                        Err(e) => {
                            add_log(format!("Failed to get network: {e}"));
                            set_connection_state.set(ConnectionState::Error(e.to_string()));
                        }
                    }
                }
                Err(e) => {
                    add_log(format!("Connection failed: {e}"));
                    set_connection_state.set(ConnectionState::Error(e.to_string()));
                }
            }
        });
    };

    // Disconnect
    let disconnect = move || {
        add_log("Disconnected".to_string());
        set_connection_state.set(ConnectionState::Disconnected);
    };

    view! {
        <div>
            <div class="story-header">
                <h2>"Wallet Connection Flow"</h2>
                <p>"Interactive demo of the full wallet connection lifecycle."</p>
            </div>

            <div class="story-section">
                <h3>"Current State"</h3>
                <div class="story-canvas">
                    <div class="connection-demo">
                        <div class="connection-demo__state">
                            {move || {
                                let state = connection_state.get();
                                view! { <ConnectionCard state=state /> }
                            }}
                        </div>

                        <div class="connection-demo__actions">
                            {move || {
                                let state = connection_state.get();
                                match state {
                                    ConnectionState::Disconnected => {
                                        let wallets = available_wallets.get();
                                        if wallets.is_empty() {
                                            view! {
                                                <p class="connection-demo__hint">"No wallets available. Install a Cardano wallet extension to test."</p>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div class="connection-demo__wallet-list">
                                                    <p class="connection-demo__label">"Select a wallet to connect:"</p>
                                                    <div class="connection-demo__buttons">
                                                        {wallets.into_iter().map(|provider| {
                                                            view! {
                                                                <button
                                                                    class="btn btn--outline"
                                                                    on:click=move |_| connect_to_wallet(provider)
                                                                >
                                                                    {provider.display_name()}
                                                                </button>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                </div>
                                            }.into_any()
                                        }
                                    }
                                    ConnectionState::Connecting => {
                                        view! {
                                            <div class="connection-demo__connecting">
                                                <div class="spinner"></div>
                                                <p>"Waiting for wallet approval..."</p>
                                            </div>
                                        }.into_any()
                                    }
                                    ConnectionState::Connected { .. } => {
                                        view! {
                                            <button
                                                class="btn btn--secondary"
                                                on:click=move |_| disconnect()
                                            >
                                                "Disconnect"
                                            </button>
                                        }.into_any()
                                    }
                                    ConnectionState::Error(_) => {
                                        view! {
                                            <button
                                                class="btn btn--primary"
                                                on:click=move |_| {
                                                    set_connection_state.set(ConnectionState::Disconnected);
                                                }
                                            >
                                                "Try Again"
                                            </button>
                                        }.into_any()
                                    }
                                }
                            }}
                        </div>
                    </div>
                </div>
            </div>

            <div class="story-section">
                <h3>"Connection Log"</h3>
                <div class="story-canvas">
                    <div class="connection-log">
                        {move || {
                            let log = connection_log.get();
                            if log.is_empty() {
                                view! { <p class="connection-log__empty">"No events yet"</p> }.into_any()
                            } else {
                                view! {
                                    <ul class="connection-log__list">
                                        {log.into_iter().map(|entry| {
                                            view! { <li>{entry}</li> }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>
            </div>

            <div class="story-section">
                <h3>"Connection API"</h3>
                <pre class="code-block">{r#"use wallet_core::{WalletApi, WalletProvider, ConnectionState};

// Connect to a specific wallet
let api = WalletApi::connect(WalletProvider::Eternl).await?;

// Get wallet info
let network_id = api.network_id().await?;  // 1 = mainnet, 0 = testnet
let address = api.change_address().await?;  // Hex-encoded address
let balance = api.balance().await?;         // CBOR-encoded value

// Sign a transaction
let witness = api.sign_tx(tx_hex, partial_sign).await?;

// Sign arbitrary data (CIP-8)
let signature = api.sign_data(address, payload_hex).await?;
// Returns DataSignature { signature, key }

// Submit a transaction
let tx_hash = api.submit_tx(signed_tx_hex).await?;"#}</pre>
            </div>
        </div>
    }
}
