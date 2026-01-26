//! Card Game UI Toolkit
//!
//! A Leptos component library for building card game UIs. Provides visual
//! building blocks without imposing game logic - consumers own their game
//! state and rules.
//!
//! ## Core Concepts
//!
//! - **GameCard**: Full-sized card for hand display and detail views
//! - **CompactCard**: Square format for deployed cards (engines on the board)
//! - **MonsterStage**: The monster-as-gameboard - target fills the play area
//! - **CardHand**: Player's hand of cards
//! - **DeploymentZone**: Container for deployed cards over the monster
//!
//! ## Visual Architecture
//!
//! For games like Leviathan Hunt, the monster itself becomes the play surface:
//!
//! ```text
//! +---------------------------------------------+
//! |           +-------------------+             |
//! |           |    MONSTER        |             |
//! |      [E]  | (fills play area) |  [E]        |
//! |           |   [E]      [E]    |             |
//! |           +-------------------+             |
//! |   +-------------------------------------+   |
//! |   |  Yours (5)  |  Theirs (12)          |   |
//! |   +-------------------------------------+   |
//! |   +---+ +---+ +---+ +---+  YOUR HAND        |
//! +---------------------------------------------+
//! ```
//!
//! ## Theming
//!
//! All visual properties are exposed via CSS custom properties:
//!
//! ```css
//! :root {
//!   --cardkit-card-radius: 8px;
//!   --cardkit-highlight: rgba(255, 215, 0, 0.6);
//!   --cardkit-owner-you: rgba(76, 175, 80, 0.3);
//!   /* ... see styles/_variables.scss for full list */
//! }
//! ```
//!
//! ## Usage
//!
//! ```ignore
//! use ui_cardkit::{
//!     GameCard, CardSize,
//!     CompactCard, CompactSize, Owner,
//!     MonsterStage, DeploymentZone, DeploymentLayout,
//!     CardHand, HandCardState,
//!     DeploymentSummary, HealthBar,
//!     CardDetailModal,
//!     STYLES,
//! };
//!
//! // Include styles once at app root
//! view! {
//!     <style>{STYLES}</style>
//!     // ... game UI
//! }
//! ```

mod card_detail_modal;
mod card_hand;
mod compact_card;
mod deployment_summary;
mod deployment_zone;
mod game_card;
mod health_bar;
mod monster_stage;
mod styles;

pub use card_detail_modal::CardDetailModal;
pub use card_hand::{CardHand, HandCardState};
pub use compact_card::{CompactCard, CompactSize, Owner};
pub use deployment_summary::DeploymentSummary;
pub use deployment_zone::{DeploymentLayout, DeploymentZone};
pub use game_card::{CardSize, GameCard};
pub use health_bar::{HealthBar, HealthBarVariant};
pub use monster_stage::MonsterStage;
pub use styles::STYLES;
