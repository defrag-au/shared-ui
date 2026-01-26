//! CardKit Styles
//!
//! Combined stylesheet for all ui-cardkit components.
//!
//! ## Usage
//!
//! Include this once at your app root:
//!
//! ```ignore
//! use ui_cardkit::STYLES;
//!
//! view! {
//!     <style>{STYLES}</style>
//!     // ... rest of app
//! }
//! ```

use scss_macros::scss;

/// Combined CSS for all ui-cardkit components.
/// Include this once at the app root level.
pub const STYLES: &str = scss!("src/styles/mod.scss");
