# Glyph System Implementation Plan

## Overview

The glyph system maps cell decay values to visual characters, creating smooth transitions from "alive" to "dead" states. This is what makes the animations feel organic and fluid.

## Core Concept

A cell's `decay` value (0-255) maps to a sequence of glyphs:

```
decay: 255 → 192 → 128 → 64 → 0
glyph:  █  →  ▓  →  ▒  →  ░  → ' '
```

## Components

### 1. GlyphSet (`glyphs/sets.rs`)

```rust
#[derive(Debug, Clone)]
pub struct GlyphSet {
    /// Glyphs from most alive to most dead
    /// Must have at least 2 glyphs
    glyphs: Vec<char>,
}

impl GlyphSet {
    /// Create custom glyph set
    pub fn new(glyphs: Vec<char>) -> Result<Self, GlyphSetError> {
        if glyphs.len() < 2 {
            return Err(GlyphSetError::TooFewGlyphs);
        }
        Ok(GlyphSet { glyphs })
    }

    /// Map decay value to glyph
    ///
    /// decay 255 → glyphs[0] (most alive)
    /// decay 0   → glyphs[last] (most dead)
    pub fn map(&self, decay: u8) -> char {
        let idx = self.decay_to_index(decay);
        self.glyphs[idx]
    }

    #[inline]
    fn decay_to_index(&self, decay: u8) -> usize {
        let normalized = decay as f32 / 255.0;  // 0.0 to 1.0
        let inverted = 1.0 - normalized;         // flip so 0 = alive
        let idx = (inverted * (self.glyphs.len() - 1) as f32).round() as usize;
        idx.min(self.glyphs.len() - 1)
    }

    pub fn len(&self) -> usize {
        self.glyphs.len()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GlyphSetError {
    #[error("glyph set must have at least 2 glyphs")]
    TooFewGlyphs,
}
```

### 2. Preset GlyphSets

```rust
impl GlyphSet {
    /// Classic block gradient: █ ▓ ▒ ░ ·
    pub fn classic() -> Self {
        GlyphSet {
            glyphs: vec!['█', '▓', '▒', '░', '·', ' '],
        }
    }

    /// Cyberpunk aesthetic: ▰ ▱ ░ ▭ ·
    pub fn cyberpunk() -> Self {
        GlyphSet {
            glyphs: vec!['▰', '▓', '▒', '░', '▱', '·', ' '],
        }
    }

    /// Matrix style: @ # + · ·
    pub fn matrix() -> Self {
        GlyphSet {
            glyphs: vec!['@', '#', '+', '*', '·', ' '],
        }
    }

    /// Dot density: ● ◉ ○ ∘ ·
    pub fn dots() -> Self {
        GlyphSet {
            glyphs: vec!['●', '◉', '○', '∘', '·', ' '],
        }
    }

    /// Binary: 1 ░
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

    /// Braille patterns (very smooth, 8 levels)
    pub fn braille() -> Self {
        GlyphSet {
            glyphs: vec!['⣿', '⣷', '⣦', '⣤', '⣀', '⠤', '⠀', ' '],
        }
    }
}
```

### 3. Decay Strategies (`glyphs/decay.rs`)

Different ways to control how decay evolves:

```rust
pub trait DecayStrategy: Send + Sync {
    /// Compute next decay value
    ///
    /// - `current`: current decay value (0-255)
    /// - `target`: target value (255 if alive, 0 if dead)
    /// - `delta_t`: time since last update (for time-based decay)
    ///
    /// Returns new decay value
    fn update(&self, current: u8, target: u8, delta_t: f32) -> u8;
}

/// Linear decay (constant rate)
pub struct LinearDecay {
    rate: u8,
}

impl DecayStrategy for LinearDecay {
    fn update(&self, current: u8, target: u8, _delta_t: f32) -> u8 {
        if current < target {
            current.saturating_add(self.rate)
        } else if current > target {
            current.saturating_sub(self.rate)
        } else {
            current
        }
    }
}

/// Exponential decay (faster at start, slower at end)
pub struct ExponentialDecay {
    rate: f32,  // 0.0 to 1.0
}

impl DecayStrategy for ExponentialDecay {
    fn update(&self, current: u8, target: u8, _delta_t: f32) -> u8 {
        let diff = target as f32 - current as f32;
        let delta = diff * self.rate;
        (current as f32 + delta).round() as u8
    }
}

/// Stepped decay (instant changes at thresholds)
pub struct SteppedDecay {
    steps: Vec<u8>,  // Threshold values
}

impl DecayStrategy for SteppedDecay {
    fn update(&self, current: u8, target: u8, _delta_t: f32) -> u8 {
        // Jump to nearest step toward target
        // TODO: implement
        current
    }
}
```

### 4. Integration with Cell

Update `Cell::update_decay` to use strategy:

```rust
impl Cell {
    pub fn update_decay_with<S: DecayStrategy>(&mut self, strategy: &S, delta_t: f32) {
        let target = if self.alive { 255 } else { 0 };
        self.decay = strategy.update(self.decay, target, delta_t);
    }
}
```

## Visual Examples

### Classic Set

```
decay: 255  223  191  159  127   95   63   31    0
glyph:  █    █    ▓    ▓    ▒    ░    ░    ·    ' '
```

### Cyberpunk Set

```
decay: 255  223  191  159  127   95   63   31    0
glyph:  ▰    ▓    ▓    ▒    ░    ▱    ·    ·    ' '
```

### Animation Example

```
Frame 0:  ████████
Frame 1:  ████▓▓▓▓
Frame 2:  ▓▓▓▓▒▒▒▒
Frame 3:  ▒▒▒▒░░░░
Frame 4:  ░░░░····
Frame 5:  ········
```

## API Design

### Basic Usage

```rust
use shimr::glyphs::GlyphSet;

let glyph_set = GlyphSet::cyberpunk();
let glyph = glyph_set.map(192);  // Returns '▓'
```

### Custom Set

```rust
let custom = GlyphSet::new(vec!['@', '#', '+', '*', '.', ' '])?;
```

### With Animation

```rust
let animation = shimr::morph("hello", "world")
    .glyph_set(GlyphSet::cyberpunk())
    .decay_rate(64)  // Faster decay
    .build();
```

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_glyph_mapping() {
    let set = GlyphSet::classic();

    // Fully alive → first glyph
    assert_eq!(set.map(255), '█');

    // Fully dead → last glyph
    assert_eq!(set.map(0), ' ');

    // Mid-range maps to middle glyphs
    let mid_glyph = set.map(127);
    assert!(mid_glyph == '▒' || mid_glyph == '░');
}

#[test]
fn test_custom_glyph_set() {
    let result = GlyphSet::new(vec!['a', 'b', 'c']);
    assert!(result.is_ok());

    let result = GlyphSet::new(vec!['a']);  // Too few
    assert!(result.is_err());
}

#[test]
fn test_linear_decay() {
    let strategy = LinearDecay { rate: 32 };

    let next = strategy.update(100, 200, 0.0);
    assert_eq!(next, 132);  // 100 + 32

    let next = strategy.update(200, 100, 0.0);
    assert_eq!(next, 168);  // 200 - 32
}
```

### Visual Tests

Create snapshot tests for glyph sequences:

```rust
#[test]
fn visual_test_decay_sequence() {
    let set = GlyphSet::classic();
    let sequence: Vec<char> = (0..=255)
        .step_by(32)
        .map(|d| set.map(d))
        .collect();

    insta::assert_debug_snapshot!(sequence);
}
```

## Performance Considerations

1. **Lookup speed:** Direct array indexing - O(1)
2. **Memory:** Small (few chars per set) - negligible
3. **Hot path:** `map()` called once per cell per frame
   - For 100x100 grid at 60fps: 600k calls/sec
   - Must be fast! Currently ~2 arithmetic ops

### Optimization Ideas

Pre-compute decay → index mapping table:

```rust
pub struct GlyphSet {
    glyphs: Vec<char>,
    // Lookup table: decay value → glyph index
    // 256 entries, one per decay value
    lookup: [u8; 256],
}

impl GlyphSet {
    fn build_lookup(glyphs: &[char]) -> [u8; 256] {
        let mut lookup = [0u8; 256];
        for decay in 0..=255 {
            lookup[decay as usize] = decay_to_index(decay, glyphs.len());
        }
        lookup
    }

    #[inline]
    pub fn map(&self, decay: u8) -> char {
        let idx = self.lookup[decay as usize];
        self.glyphs[idx as usize]
    }
}
```

This trades 256 bytes memory for eliminating float math in hot path.

## Implementation Order

1. **GlyphSet** - core type and mapping logic
2. **Preset sets** - classic, cyberpunk, etc.
3. **Decay strategies** - at least linear
4. **Tests** - mapping, presets, edge cases
5. **Benchmarks** - measure lookup performance
6. **Optimization** - lookup table if needed

## Open Questions

1. **Color support?** Map decay to colors too?
   - Decision: Add in Phase 5 as feature flag

2. **Unicode safety?** Validate glyph widths?
   - Decision: Document that users should use single-width chars

3. **Interpolation?** Support sub-glyph smoothing?
   - Decision: No, keep it discrete and fast

## Next Steps

1. Implement GlyphSet with basic mapping
2. Add preset sets
3. Test visual output
4. Integrate with Frame rendering
