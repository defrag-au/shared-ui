//! Value and balance decoding utilities
//!
//! Decode CBOR-encoded Cardano values (lovelace + multi-assets) from CIP-30 wallet API.

use crate::PallasError;
use pallas_codec::minicbor;
use pallas_primitives::conway::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Decoded wallet balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    /// Total lovelace (ADA = lovelace / 1_000_000)
    pub lovelace: u64,
    /// Multi-assets grouped by policy ID
    /// Key: policy ID (hex), Value: map of asset name (hex) to quantity
    pub assets: HashMap<String, HashMap<String, u64>>,
}

impl WalletBalance {
    /// Get the ADA amount (lovelace / 1_000_000)
    pub fn ada(&self) -> f64 {
        self.lovelace as f64 / 1_000_000.0
    }

    /// Get total number of distinct native tokens
    pub fn token_count(&self) -> usize {
        self.assets.values().map(|m| m.len()).sum()
    }

    /// Get total number of distinct policies
    pub fn policy_count(&self) -> usize {
        self.assets.len()
    }
}

/// Decode a CBOR-encoded Value from the wallet's getBalance response
///
/// The Value type in Cardano can be either:
/// - A simple integer (just lovelace)
/// - A tuple of (lovelace, multi-asset map)
pub fn decode_balance(balance_hex: &str) -> Result<WalletBalance, PallasError> {
    let bytes = hex::decode(balance_hex)?;

    let value: Value =
        minicbor::decode(&bytes).map_err(|e| PallasError::CborDecode(e.to_string()))?;

    Ok(value_to_balance(value))
}

/// Convert a pallas Value to our WalletBalance struct
fn value_to_balance(value: Value) -> WalletBalance {
    match value {
        Value::Coin(lovelace) => WalletBalance {
            lovelace,
            assets: HashMap::new(),
        },
        Value::Multiasset(lovelace, multi_assets) => {
            let mut assets: HashMap<String, HashMap<String, u64>> = HashMap::new();

            for (policy_id, asset_map) in multi_assets.iter() {
                let policy_hex = hex::encode(policy_id.as_ref());
                let mut policy_assets: HashMap<String, u64> = HashMap::new();

                for (asset_name, quantity) in asset_map.iter() {
                    let asset_bytes: &[u8] = asset_name.as_ref();
                    let asset_name_hex = hex::encode(asset_bytes);
                    policy_assets.insert(asset_name_hex, u64::from(*quantity));
                }

                assets.insert(policy_hex, policy_assets);
            }

            WalletBalance { lovelace, assets }
        }
    }
}

/// A single native token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeToken {
    /// Policy ID (hex)
    pub policy_id: String,
    /// Asset name (hex)
    pub asset_name_hex: String,
    /// Asset name as UTF-8 string (if valid)
    pub asset_name: Option<String>,
    /// Quantity
    pub quantity: u64,
}

impl WalletBalance {
    /// Get all native tokens as a flat list
    pub fn tokens(&self) -> Vec<NativeToken> {
        let mut tokens = Vec::new();

        for (policy_id, asset_map) in &self.assets {
            for (asset_name_hex, quantity) in asset_map {
                let asset_name = hex::decode(asset_name_hex)
                    .ok()
                    .and_then(|bytes| String::from_utf8(bytes).ok());

                tokens.push(NativeToken {
                    policy_id: policy_id.clone(),
                    asset_name_hex: asset_name_hex.clone(),
                    asset_name,
                    quantity: *quantity,
                });
            }
        }

        tokens
    }
}
