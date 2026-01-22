//! Shared UI primitives for web components
//!
//! Provides reactive helpers, DOM utilities, and base traits for building
//! framework-agnostic web components.

mod reactive;
mod dom;
mod component;

pub use reactive::*;
pub use dom::*;
pub use component::*;
