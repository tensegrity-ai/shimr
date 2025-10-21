//! shimr - Terminal transition effects using cellular automata
//!
//! Create organic transition animations for terminal UIs. Watch text dissolve
//! into Conway patterns, then crystallize into new forms.
//!
//! # Quick Start
//!
//! ```no_run
//! use shimr::prelude::*;
//!
//! // Coming soon: Full text morphing API
//! // let animation = shimr::morph("hello", "world")
//! //     .generations(20)
//! //     .glyph_set(GlyphSet::cyberpunk())
//! //     .build();
//! ```

pub mod core;
pub mod rules;
pub mod glyphs;
pub mod transition;

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::core::{Cell, Grid, Automata};
    pub use crate::rules::{Rule, Conway};
    pub use crate::glyphs::GlyphSet;
    pub use crate::transition::Frame;
}
