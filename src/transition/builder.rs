use crate::core::Automata;
use crate::rules::{Rule, Conway};
use crate::glyphs::GlyphSet;
use crate::color::ColorMap;
use crate::fonts::FontRenderer;
use super::{Transition, text::{text_to_grid, default_font}, morph::merge_grids};

/// Builder for text morphing animations
///
/// Provides a fluent API for configuring and creating transitions.
///
/// # Example
///
/// ```no_run
/// use shimr::prelude::*;
///
/// let animation = MorphBuilder::new("hello", "world")
///     .generations(20)
///     .glyph_set(GlyphSet::cyberpunk())
///     .build();
///
/// for frame in animation {
///     println!("{}", frame);
/// }
/// ```
pub struct MorphBuilder {
    start: String,
    end: String,
    generations: usize,
    rule: Box<dyn Rule>,
    glyph_set: GlyphSet,
    color_map: Option<Box<dyn ColorMap>>,
    decay_rate: u8,
    font: Box<dyn FontRenderer>,
}

impl MorphBuilder {
    /// Create a new morph builder
    ///
    /// # Arguments
    /// * `start` - Starting text
    /// * `end` - Target text
    pub fn new(start: impl Into<String>, end: impl Into<String>) -> Self {
        MorphBuilder {
            start: start.into(),
            end: end.into(),
            generations: 30,
            rule: Box::new(Conway),
            glyph_set: GlyphSet::classic(),
            color_map: None,
            decay_rate: 32,
            font: Box::new(default_font()),
        }
    }

    /// Set number of generations (frames) to generate
    pub fn generations(mut self, n: usize) -> Self {
        self.generations = n;
        self
    }

    /// Set the cellular automata rule
    pub fn rule(mut self, rule: impl Rule + 'static) -> Self {
        self.rule = Box::new(rule);
        self
    }

    /// Set the glyph set for rendering
    pub fn glyph_set(mut self, set: GlyphSet) -> Self {
        self.glyph_set = set;
        self
    }

    /// Set color map for colored output
    pub fn color_map(mut self, color_map: impl ColorMap + 'static) -> Self {
        self.color_map = Some(Box::new(color_map));
        self
    }

    /// Set decay rate (how fast visual transitions happen)
    ///
    /// Higher values = faster transitions
    /// Range: 1-255 (default: 32)
    pub fn decay_rate(mut self, rate: u8) -> Self {
        self.decay_rate = rate;
        self
    }

    /// Set the font renderer
    pub fn font(mut self, font: Box<dyn FontRenderer>) -> Self {
        self.font = font;
        self
    }

    /// Build the transition
    pub fn build(self) -> Transition {
        // Convert text to grids
        let start_grid = text_to_grid(&self.start, &*self.font);
        let end_grid = text_to_grid(&self.end, &*self.font);

        // Merge grids for initial state
        let grid = merge_grids(&start_grid, &end_grid);

        // Create automata
        let automata = Automata::new(grid, self.rule)
            .with_decay_rate(self.decay_rate);

        // Build transition
        let mut transition = Transition::new(automata, self.glyph_set, self.generations);

        // Add color map if provided
        if let Some(color_map) = self.color_map {
            transition = transition.with_color_map(color_map);
        }

        transition
    }
}

/// Convenience function for creating text morphing animations
///
/// # Example
///
/// ```no_run
/// use shimr::morph;
///
/// for frame in morph("hello", "world").build() {
///     println!("{}", frame);
/// }
/// ```
pub fn morph(start: impl Into<String>, end: impl Into<String>) -> MorphBuilder {
    MorphBuilder::new(start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let builder = MorphBuilder::new("start", "end");

        assert_eq!(builder.start, "start");
        assert_eq!(builder.end, "end");
        assert_eq!(builder.generations, 30); // default
    }

    #[test]
    fn test_builder_configuration() {
        let builder = MorphBuilder::new("a", "b")
            .generations(50)
            .decay_rate(64);

        assert_eq!(builder.generations, 50);
        assert_eq!(builder.decay_rate, 64);
    }

    #[test]
    fn test_builder_build() {
        let transition = MorphBuilder::new("hi", "bye")
            .generations(10)
            .build();

        assert_eq!(transition.len(), 10);
    }

    #[test]
    fn test_morph_convenience() {
        let transition = morph("test", "done")
            .generations(5)
            .build();

        assert_eq!(transition.len(), 5);
    }

    #[test]
    fn test_full_morph() {
        let frames: Vec<_> = morph("a", "b")
            .generations(3)
            .glyph_set(GlyphSet::ascii())
            .build()
            .collect();

        assert_eq!(frames.len(), 3);

        // Each frame should be renderable
        for frame in &frames {
            let rendered = frame.render();
            assert!(!rendered.is_empty());
        }
    }
}
