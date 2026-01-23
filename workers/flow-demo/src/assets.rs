//! PFP City API integration for fetching NFT assets.
//!
//! This module provides functions to fetch random assets from the PFP City API
//! for use in the memory game.

use crate::types::Card;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use serde::Deserialize;
use worker::*;

const API_BASE_URL: &str = "https://a2.pfp.city";

/// Asset details from PFP City API
#[derive(Deserialize, Debug, Clone)]
pub struct AssetDetails {
    pub asset_id: String,
    #[serde(default)]
    pub asset_name: String,
    pub name: String,
}

/// Response from /v3/api/collections/{policy_id}/assets endpoint
#[derive(Deserialize, Debug, Clone)]
pub struct CollectionAssetsResponse {
    pub assets: Vec<AssetDetails>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

/// Fetch collection metadata to get total asset count
pub async fn get_collection_total(policy_id: &str) -> Result<u32> {
    let url = format!("{API_BASE_URL}/v3/api/collections/{policy_id}/assets?limit=1&offset=0");

    let mut response = Fetch::Url(Url::parse(&url)?).send().await?;

    let status = response.status_code();
    if !(200..300).contains(&status) {
        return Err(Error::from(format!("PFP City API error: {status}")));
    }

    let data: CollectionAssetsResponse = response.json().await?;
    Ok(data.total)
}

/// Fetch a batch of assets from a collection at a specific offset
pub async fn fetch_assets_batch(
    policy_id: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<AssetDetails>> {
    let url = format!(
        "{API_BASE_URL}/v3/api/collections/{policy_id}/assets?limit={limit}&offset={offset}"
    );

    let mut response = Fetch::Url(Url::parse(&url)?).send().await?;

    let status = response.status_code();
    if !(200..300).contains(&status) {
        return Err(Error::from(format!("PFP City API error: {status}")));
    }

    let data: CollectionAssetsResponse = response.json().await?;
    Ok(data.assets)
}

/// Generate image URL for an asset
pub fn image_url(policy_id: &str, asset_id: &str) -> String {
    // Use thumbnail size (400px) for card display
    format!("https://img.pfp.city/{policy_id}/img/thumb/{asset_id}.png")
}

/// Fetch random assets and create card pairs for the memory game.
///
/// # Arguments
/// * `policy_id` - The Cardano policy ID for the NFT collection
/// * `pair_count` - Number of pairs needed (e.g., 32 for an 8x8 grid)
/// * `seed` - Random seed for deterministic shuffling
///
/// # Returns
/// A shuffled vector of cards (2 * pair_count cards total)
pub async fn fetch_game_cards(policy_id: &str, pair_count: u8, seed: u64) -> Result<Vec<Card>> {
    // Get total assets in collection
    let total = get_collection_total(policy_id).await?;

    if total < pair_count as u32 {
        return Err(Error::from(format!(
            "Collection only has {} assets, need at least {} for {} pairs",
            total, pair_count, pair_count
        )));
    }

    // Create RNG from seed
    let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);

    // Generate random offsets to fetch diverse assets
    // We'll fetch from multiple random positions in the collection
    let mut all_assets: Vec<AssetDetails> = Vec::with_capacity(pair_count as usize);

    // Fetch assets in batches, picking random offsets
    let batch_size = 20u32;
    let mut attempts = 0;
    let max_attempts = 10;

    while all_assets.len() < pair_count as usize && attempts < max_attempts {
        let max_offset = total.saturating_sub(batch_size);
        let offset = if max_offset > 0 {
            rand::Rng::gen_range(&mut rng, 0..max_offset)
        } else {
            0
        };

        let batch = fetch_assets_batch(policy_id, batch_size, offset).await?;

        for asset in batch {
            // Avoid duplicates
            if !all_assets.iter().any(|a| a.asset_id == asset.asset_id) {
                all_assets.push(asset);
                if all_assets.len() >= pair_count as usize {
                    break;
                }
            }
        }

        attempts += 1;
    }

    if all_assets.len() < pair_count as usize {
        return Err(Error::from(format!(
            "Could only fetch {} unique assets, need {}",
            all_assets.len(),
            pair_count
        )));
    }

    // Truncate to exact pair count
    all_assets.truncate(pair_count as usize);

    // Create card pairs - each asset appears twice
    let mut cards: Vec<Card> = Vec::with_capacity(pair_count as usize * 2);

    for (pair_id, asset) in all_assets.into_iter().enumerate() {
        let card = Card {
            asset_id: asset.asset_id.clone(),
            name: asset.name.clone(),
            image_url: image_url(policy_id, &asset.asset_id),
            matched: false,
            matched_by: None,
            pair_id: pair_id as u8,
        };

        // Add two copies of each card
        cards.push(card.clone());
        cards.push(card);
    }

    // Shuffle the cards deterministically using the seed
    cards.shuffle(&mut rng);

    Ok(cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_url() {
        let url = image_url(
            "5ac825392b7608d6e92a4e5c528fe9b8fadd6eaa3e36a685e37175d1",
            "abc123",
        );
        assert_eq!(
            url,
            "https://img.pfp.city/5ac825392b7608d6e92a4e5c528fe9b8fadd6eaa3e36a685e37175d1/img/thumb/abc123.png"
        );
    }
}
