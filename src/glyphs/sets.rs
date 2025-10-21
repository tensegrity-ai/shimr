/// Set of glyphs for rendering cell decay states
///
/// Maps decay values (0-255) to visual characters, creating smooth transitions
/// from "alive" to "dead" states.
#[derive(Debug, Clone)]
pub struct GlyphSet {
    /// Glyphs from most alive to most dead
    /// Index 0 = fully alive (decay 255)
    /// Index last = fully dead (decay 0)
    glyphs: Vec<char>,
}

impl GlyphSet {
    /// Create a custom glyph set
    ///
    /// # Arguments
    /// * `glyphs` - Characters from most alive to most dead (min 2 glyphs)
    ///
    /// # Errors
    /// Returns error if fewer than 2 glyphs provided
    pub fn new(glyphs: Vec<char>) -> Result<Self, GlyphSetError> {
        if glyphs.len() < 2 {
            return Err(GlyphSetError::TooFewGlyphs);
        }
        Ok(GlyphSet { glyphs })
    }

    /// Map decay value to glyph
    ///
    /// # Arguments
    /// * `decay` - Decay value (0-255)
    ///   - 255 → first glyph (most alive)
    ///   - 0 → last glyph (most dead)
    ///
    /// # Returns
    /// Appropriate glyph character for the decay level
    #[inline]
    pub fn map(&self, decay: u8) -> char {
        let idx = self.decay_to_index(decay);
        self.glyphs[idx]
    }

    /// Convert decay value to glyph index
    #[inline]
    fn decay_to_index(&self, decay: u8) -> usize {
        // Normalize decay to 0.0-1.0 range
        let normalized = decay as f32 / 255.0;

        // Invert so high decay → low index (first glyph)
        let inverted = 1.0 - normalized;

        // Map to glyph index
        let idx = (inverted * (self.glyphs.len() - 1) as f32).round() as usize;

        // Clamp to valid range
        idx.min(self.glyphs.len() - 1)
    }

    /// Number of glyphs in this set
    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    /// Check if glyph set is empty (should never be true for valid sets)
    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }

    // Preset glyph sets

    /// Classic block gradient: █ ▓ ▒ ░ · ' '
    pub fn classic() -> Self {
        GlyphSet {
            glyphs: vec!['█', '▓', '▒', '░', '·', ' '],
        }
    }

    /// Cyberpunk aesthetic: ▰ ▓ ▒ ░ ▱ · ' '
    pub fn cyberpunk() -> Self {
        GlyphSet {
            glyphs: vec!['▰', '▓', '▒', '░', '▱', '·', ' '],
        }
    }

    /// Matrix style: @ # + * · ' '
    pub fn matrix() -> Self {
        GlyphSet {
            glyphs: vec!['@', '#', '+', '*', '·', ' '],
        }
    }

    /// Dot density: ● ◉ ○ ∘ · ' '
    pub fn dots() -> Self {
        GlyphSet {
            glyphs: vec!['●', '◉', '○', '∘', '·', ' '],
        }
    }

    /// Binary: 1 ░ ' '
    pub fn binary() -> Self {
        GlyphSet {
            glyphs: vec!['1', '░', ' '],
        }
    }

    /// ASCII-safe: # + . ' '
    pub fn ascii() -> Self {
        GlyphSet {
            glyphs: vec!['#', '+', '.', ' '],
        }
    }
}

/// Error type for GlyphSet operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlyphSetError {
    /// Glyph set must have at least 2 glyphs
    TooFewGlyphs,
}

impl std::fmt::Display for GlyphSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlyphSetError::TooFewGlyphs => {
                write!(f, "glyph set must have at least 2 glyphs")
            }
        }
    }
}

impl std::error::Error for GlyphSetError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glyph_set_boundaries() {
        let set = GlyphSet::classic();

        // Fully alive → first glyph
        assert_eq!(set.map(255), '█');

        // Fully dead → last glyph
        assert_eq!(set.map(0), ' ');
    }

    #[test]
    fn test_glyph_mapping_progression() {
        let set = GlyphSet::classic();

        // As decay decreases, glyphs should progress through the set
        let glyphs: Vec<char> = (0..=255).step_by(51).map(|d| set.map(d)).collect();

        // Should have different glyphs at different decay levels
        assert!(glyphs.contains(&'█')); // High decay
        assert!(glyphs.contains(&' ')); // Low decay
    }

    #[test]
    fn test_custom_glyph_set() {
        let set = GlyphSet::new(vec!['A', 'B', 'C']).unwrap();

        assert_eq!(set.len(), 3);
        assert_eq!(set.map(255), 'A'); // Alive
        assert_eq!(set.map(0), 'C');   // Dead
    }

    #[test]
    fn test_glyph_set_too_few() {
        let result = GlyphSet::new(vec!['A']);
        assert!(result.is_err());

        match result {
            Err(GlyphSetError::TooFewGlyphs) => (),
            _ => panic!("Expected TooFewGlyphs error"),
        }
    }

    #[test]
    fn test_preset_sets() {
        // Just verify they create valid sets
        assert!(GlyphSet::classic().len() >= 2);
        assert!(GlyphSet::cyberpunk().len() >= 2);
        assert!(GlyphSet::matrix().len() >= 2);
        assert!(GlyphSet::dots().len() >= 2);
        assert!(GlyphSet::binary().len() >= 2);
        assert!(GlyphSet::ascii().len() >= 2);
    }

    #[test]
    fn test_glyph_index_clamping() {
        let set = GlyphSet::new(vec!['A', 'B']).unwrap();

        // Should clamp to valid indices
        assert_eq!(set.decay_to_index(255), 0); // First glyph
        assert_eq!(set.decay_to_index(0), 1);   // Last glyph
    }
}
