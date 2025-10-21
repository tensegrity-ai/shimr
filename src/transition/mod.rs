//! Transition animations and frame generation

mod frame;
mod text;
pub mod morph;
mod builder;

pub use frame::Frame;
pub use builder::{MorphBuilder, morph};
pub use morph::Transition;
