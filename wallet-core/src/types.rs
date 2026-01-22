//! Cardano wallet types

use serde::{Deserialize, Serialize};

/// Supported wallet providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WalletProvider {
    Nami,
    Eternl,
    Lace,
    Flint,
    Typhon,
    Vespr,
    NuFi,
    Gero,
    Yoroi,
}

impl WalletProvider {
    /// Get the window.cardano property name for this wallet
    pub fn api_name(&self) -> &'static str {
        match self {
            WalletProvider::Nami => "nami",
            WalletProvider::Eternl => "eternl",
            WalletProvider::Lace => "lace",
            WalletProvider::Flint => "flint",
            WalletProvider::Typhon => "typhon",
            WalletProvider::Vespr => "vespr",
            WalletProvider::NuFi => "nufi",
            WalletProvider::Gero => "gerowallet",
            WalletProvider::Yoroi => "yoroi",
        }
    }

    /// Get display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            WalletProvider::Nami => "Nami",
            WalletProvider::Eternl => "Eternl",
            WalletProvider::Lace => "Lace",
            WalletProvider::Flint => "Flint",
            WalletProvider::Typhon => "Typhon",
            WalletProvider::Vespr => "Vespr",
            WalletProvider::NuFi => "NuFi",
            WalletProvider::Gero => "Gero",
            WalletProvider::Yoroi => "Yoroi",
        }
    }

    /// All known wallet providers
    pub fn all() -> &'static [WalletProvider] {
        &[
            WalletProvider::Nami,
            WalletProvider::Eternl,
            WalletProvider::Lace,
            WalletProvider::Flint,
            WalletProvider::Typhon,
            WalletProvider::Vespr,
            WalletProvider::NuFi,
            WalletProvider::Gero,
            WalletProvider::Yoroi,
        ]
    }
}

/// Cardano network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Preprod,
    Preview,
}

impl Network {
    pub fn network_id(&self) -> u8 {
        match self {
            Network::Mainnet => 1,
            Network::Preprod | Network::Preview => 0,
        }
    }
}

/// Wallet connection state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected {
        provider: WalletProvider,
        address: String,
        network: Network,
    },
    Error(String),
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::Disconnected
    }
}
