//! PFP City API client for WASM
//!
//! Provides unauthenticated access to PFP City collection and asset APIs.
//!
//! ## Endpoints Used
//!
//! - `GET /v3/api/collections/{policy_id}/assets` - List collection assets (paginated)
//! - `GET /v3/api/assets/{policy_id}:{asset_name_hex}` - Get single asset details with traits
//!
//! ## Usage
//!
//! ```ignore
//! use crate::api::pfp_city::{fetch_collection_assets, fetch_asset_details};
//!
//! // Fetch collection listing
//! let assets = fetch_collection_assets(policy_id, 12, 0).await?;
//!
//! // Fetch single asset with traits
//! let details = fetch_asset_details(policy_id, asset_name_hex).await?;
//! ```

use gloo_net::http::Request;
use serde::Deserialize;
use std::collections::HashMap;

const API_BASE_URL: &str = "https://a2.pfp.city";

// ============================================================================
// Common Types
// ============================================================================

/// Asset ID from PFP City API
#[derive(Deserialize, Debug, Clone)]
pub struct AssetId {
    pub policy_id: String,
    pub asset_name_hex: String,
}

impl AssetId {
    /// Get concatenated asset ID (policy_id + asset_name_hex)
    pub fn concatenated(&self) -> String {
        format!("{}{}", self.policy_id, self.asset_name_hex)
    }
}

// ============================================================================
// Collection Assets API
// ============================================================================

/// Asset from collection listing (minimal info)
#[derive(Deserialize, Debug, Clone)]
pub struct CollectionAsset {
    pub id: AssetId,
    pub name: String,
    #[serde(default)]
    pub image: Option<String>,
}

/// Pagination info from collection API
#[derive(Deserialize, Debug, Clone)]
pub struct PaginationInfo {
    pub limit: u32,
    pub offset: u32,
    pub has_more: bool,
    #[serde(default)]
    pub total_available: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
struct CollectionAssetsData {
    pub assets: Vec<CollectionAsset>,
    pub pagination: PaginationInfo,
}

#[derive(Deserialize, Debug, Clone)]
struct CollectionAssetsResponse {
    pub success: bool,
    pub data: CollectionAssetsData,
}

/// Fetch assets from a collection with pagination
///
/// # Arguments
/// * `policy_id` - Cardano policy ID
/// * `limit` - Number of assets to fetch (max ~100)
/// * `offset` - Pagination offset
///
/// # Returns
/// Tuple of (assets, pagination_info)
pub async fn fetch_collection_assets(
    policy_id: &str,
    limit: u32,
    offset: u32,
) -> Result<(Vec<CollectionAsset>, PaginationInfo), String> {
    let url = format!(
        "{API_BASE_URL}/v3/api/collections/{policy_id}/assets?limit={limit}&offset={offset}"
    );

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if !response.ok() {
        return Err(format!("API error: {}", response.status()));
    }

    let data: CollectionAssetsResponse = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {e}"))?;

    if !data.success {
        return Err("API returned success: false".to_string());
    }

    Ok((data.data.assets, data.data.pagination))
}

// ============================================================================
// Asset Details API
// ============================================================================

/// Full asset details including traits
#[derive(Deserialize, Debug, Clone)]
pub struct AssetDetails {
    pub id: AssetId,
    pub name: String,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub media_type: Option<String>,
    #[serde(default)]
    pub traits: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub rarity_rank: Option<u32>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct AssetDetailsResponse {
    pub success: bool,
    pub data: AssetDetails,
}

/// Fetch single asset details with traits
///
/// # Arguments
/// * `policy_id` - Cardano policy ID
/// * `asset_name_hex` - Asset name in hex format
///
/// # Returns
/// Full asset details including traits and rarity
pub async fn fetch_asset_details(
    policy_id: &str,
    asset_name_hex: &str,
) -> Result<AssetDetails, String> {
    let url = format!("{API_BASE_URL}/v3/api/assets/{policy_id}:{asset_name_hex}");

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if !response.ok() {
        return Err(format!("API error: {}", response.status()));
    }

    let data: AssetDetailsResponse = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {e}"))?;

    if !data.success {
        return Err("API returned success: false".to_string());
    }

    Ok(data.data)
}

// ============================================================================
// Known Collections
// ============================================================================

/// Known NFT collections for demos
pub const KNOWN_COLLECTIONS: &[(&str, &str)] = &[
    (
        "b3dab69f7e6100849434fb1781e34bd12a916557f6231b8d2629b6f6",
        "Black Flag Pirates",
    ),
    (
        "5ac825392b7608d6e92a4e5c528fe9b8fadd6eaa3e36a685e37175d1",
        "Rotten Ape",
    ),
    (
        "d5e6bf0500378d4f0da4e8dde6becec7621cd8cbf5cbb9b87013d4cc",
        "Spacebudz",
    ),
];
