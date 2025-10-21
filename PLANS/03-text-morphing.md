# Text Morphing Implementation Plan

## Overview

Text morphing is the high-level API that converts text strings into cellular automata grids and generates smooth transitions between them.

## Core Flow

```
"hello" → Grid → Automata → Iterator<Frame> → "world"
```

1. Convert start/end text to grids
2. Initialize grid with start state
3. Inject end state as "seed" cells
4. Run CA generations
5. Emit frames as text gradually morphs

## Components

### 1. Text to Grid Conversion (`transition/text.rs`)

```rust
/// Convert text to grid representation
pub struct TextGrid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl TextGrid {
    /// Convert text to grid using simple bitmap
    ///
    /// Each character becomes a 5x7 bitmap (or similar)
    pub fn from_text(text: &str) -> Self {
        // Use simple ASCII representation for now
        // Can upgrade to proper font rendering later

        let chars: Vec<char> = text.chars().collect();
        let char_width = 5;
        let char_height = 7;
        let spacing = 1;

        let width = chars.len() * (char_width + spacing);
        let height = char_height;

        let mut cells = vec![Cell::dead(); width * height];

        for (i, ch) in chars.iter().enumerate() {
            let x_offset = i * (char_width + spacing);
            let bitmap = char_to_bitmap(*ch);

            for (y, row) in bitmap.iter().enumerate() {
                for (x, &bit) in row.iter().enumerate() {
                    if bit {
                        let idx = y * width + (x_offset + x);
                        cells[idx] = Cell::alive();
                    }
                }
            }
        }

        TextGrid { width, height, cells }
    }

    pub fn into_grid(self) -> Grid {
        Grid::from_cells(self.width, self.height, self.cells)
    }
}

/// Simple 5x5 ASCII bitmap representation
fn char_to_bitmap(ch: char) -> [[bool; 5]; 5] {
    match ch {
        'a' | 'A' => [
            [false, true,  true,  true,  false],
            [true,  false, false, false, true ],
            [true,  true,  true,  true,  true ],
            [true,  false, false, false, true ],
            [true,  false, false, false, true ],
        ],
        'h' | 'H' => [
            [true,  false, false, false, true ],
            [true,  false, false, false, true ],
            [true,  true,  true,  true,  true ],
            [true,  false, false, false, true ],
            [true,  false, false, false, true ],
        ],
        // ... more characters
        _ => [
            [true,  true,  true,  true,  true ],
            [true,  false, false, false, true ],
            [true,  false, false, false, true ],
            [true,  false, false, false, true ],
            [true,  true,  true,  true,  true ],
        ],
    }
}
```

### 2. Morph Strategy (`transition/strategy.rs`)

How do we transition from start to end?

```rust
pub trait MorphStrategy: Send + Sync {
    /// Initialize grid for morphing
    ///
    /// - `start_grid`: starting text as grid
    /// - `end_grid`: target text as grid
    /// - Returns initialized grid ready for CA simulation
    fn init(&self, start_grid: Grid, end_grid: Grid) -> Grid;
}

/// Dissolve start, then crystallize end
pub struct DissolveAndForm;

impl MorphStrategy for DissolveAndForm {
    fn init(&self, start_grid: Grid, end_grid: Grid) -> Grid {
        // Start with start_grid
        // Inject some end_grid cells as "seeds"
        // CA will dissolve start and grow end

        let width = start_grid.width().max(end_grid.width());
        let height = start_grid.height().max(end_grid.height());

        let mut grid = Grid::new(width, height);

        // Copy start cells
        for (x, y, cell) in start_grid.cells() {
            grid.set(x, y, cell);
        }

        // Inject end cells as seeds (sparse pattern)
        for (x, y, cell) in end_grid.cells() {
            if cell.is_alive() && (x + y) % 3 == 0 {  // Sparse seeding
                grid.set(x, y, Cell::alive());
            }
        }

        grid
    }
}

/// Cross-fade through chaos
pub struct ChaosFade;

impl MorphStrategy for ChaosFade {
    fn init(&self, start_grid: Grid, end_grid: Grid) -> Grid {
        // Start with start_grid
        // Add random noise
        // End state emerges from chaos

        // TODO: implement
        start_grid
    }
}
```

### 3. Frame Type (`transition/frame.rs`)

```rust
/// A single rendered frame
#[derive(Debug, Clone)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub glyphs: Vec<char>,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Frame {
            width,
            height,
            glyphs: vec![' '; width * height],
        }
    }

    /// Get glyph at (x, y)
    pub fn get(&self, x: usize, y: usize) -> char {
        if x >= self.width || y >= self.height {
            return ' ';
        }
        self.glyphs[y * self.width + x]
    }

    /// Set glyph at (x, y)
    pub fn set(&mut self, x: usize, y: usize, glyph: char) {
        if x < self.width && y < self.height {
            self.glyphs[y * self.width + x] = glyph;
        }
    }

    /// Render as string (for terminal output)
    pub fn render(&self) -> String {
        let mut output = String::with_capacity((self.width + 1) * self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                output.push(self.get(x, y));
            }
            if y < self.height - 1 {
                output.push('\n');
            }
        }

        output
    }

    /// Render to pre-allocated buffer (zero-allocation)
    pub fn render_into(&self, buffer: &mut String) {
        buffer.clear();
        buffer.reserve((self.width + 1) * self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                buffer.push(self.get(x, y));
            }
            if y < self.height - 1 {
                buffer.push('\n');
            }
        }
    }
}

impl std::fmt::Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
```

### 4. Transition Iterator (`transition/morph.rs`)

```rust
use crate::core::Automata;
use crate::glyphs::GlyphSet;
use super::frame::Frame;

/// Transition animation between two states
pub struct Transition {
    automata: Automata,
    glyph_set: GlyphSet,
    current_gen: usize,
    max_gen: usize,
    frame_buffer: Frame,
}

impl Transition {
    pub fn new(
        automata: Automata,
        glyph_set: GlyphSet,
        generations: usize,
    ) -> Self {
        let grid = automata.grid();
        let frame_buffer = Frame::new(grid.width(), grid.height());

        Transition {
            automata,
            glyph_set,
            current_gen: 0,
            max_gen: generations,
            frame_buffer,
        }
    }

    /// Render current grid state to frame
    fn render_frame(&mut self) {
        let grid = self.automata.grid();

        for (x, y, cell) in grid.cells() {
            let glyph = self.glyph_set.map(cell.decay());
            self.frame_buffer.set(x, y, glyph);
        }
    }
}

impl Iterator for Transition {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_gen >= self.max_gen {
            return None;
        }

        // Render current state
        self.render_frame();

        // Step simulation for next frame
        self.automata.step();
        self.current_gen += 1;

        // Clone frame (user gets owned copy)
        Some(self.frame_buffer.clone())
    }
}

impl ExactSizeIterator for Transition {
    fn len(&self) -> usize {
        self.max_gen.saturating_sub(self.current_gen)
    }
}
```

### 5. Builder API (`transition/builder.rs`)

```rust
use crate::core::{Grid, Automata};
use crate::rules::{Rule, Conway};
use crate::glyphs::GlyphSet;
use super::{Transition, TextGrid, MorphStrategy, DissolveAndForm};

pub struct MorphBuilder {
    start: String,
    end: String,
    generations: usize,
    rule: Box<dyn Rule>,
    glyph_set: GlyphSet,
    strategy: Box<dyn MorphStrategy>,
}

impl MorphBuilder {
    pub fn new(start: impl Into<String>, end: impl Into<String>) -> Self {
        MorphBuilder {
            start: start.into(),
            end: end.into(),
            generations: 30,
            rule: Box::new(Conway),
            glyph_set: GlyphSet::classic(),
            strategy: Box::new(DissolveAndForm),
        }
    }

    pub fn generations(mut self, n: usize) -> Self {
        self.generations = n;
        self
    }

    pub fn rule(mut self, rule: impl Rule + 'static) -> Self {
        self.rule = Box::new(rule);
        self
    }

    pub fn glyph_set(mut self, set: GlyphSet) -> Self {
        self.glyph_set = set;
        self
    }

    pub fn strategy(mut self, strategy: impl MorphStrategy + 'static) -> Self {
        self.strategy = Box::new(strategy);
        self
    }

    pub fn build(self) -> Transition {
        // Convert text to grids
        let start_grid = TextGrid::from_text(&self.start).into_grid();
        let end_grid = TextGrid::from_text(&self.end).into_grid();

        // Initialize with strategy
        let grid = self.strategy.init(start_grid, end_grid);

        // Create automata
        let automata = Automata::new(grid, self.rule);

        // Build transition
        Transition::new(automata, self.glyph_set, self.generations)
    }
}

/// Convenience function for simple morphing
pub fn morph(start: impl Into<String>, end: impl Into<String>) -> MorphBuilder {
    MorphBuilder::new(start, end)
}
```

## Public API

### Simple Usage

```rust
use shimr::prelude::*;

// Minimal API
for frame in shimr::morph("hello", "world") {
    println!("{}", frame);
    std::thread::sleep(Duration::from_millis(16));
}
```

### Configured Usage

```rust
use shimr::prelude::*;

let animation = shimr::morph("layer 0", "layer 1")
    .generations(20)
    .glyph_set(GlyphSet::cyberpunk())
    .rule(Conway)
    .build();

for (i, frame) in animation.enumerate() {
    println!("Frame {}: {}", i, frame);
}
```

### Advanced Usage

```rust
use shimr::prelude::*;

let animation = Transition::builder()
    .from_text("start")
    .to_text("end")
    .strategy(CustomStrategy)
    .rule(CustomRule::new())
    .glyph_set(GlyphSet::new(vec!['@', '#', '.', ' '])?)
    .generations(40)
    .build();
```

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_text_to_grid() {
    let grid = TextGrid::from_text("hi");
    assert!(grid.width > 0);
    assert!(grid.height > 0);
}

#[test]
fn test_morph_builder() {
    let transition = shimr::morph("a", "b")
        .generations(10)
        .build();

    assert_eq!(transition.len(), 10);
}

#[test]
fn test_frame_render() {
    let mut frame = Frame::new(3, 2);
    frame.set(0, 0, 'a');
    frame.set(1, 0, 'b');

    let rendered = frame.render();
    assert!(rendered.contains('a'));
    assert!(rendered.contains('b'));
}
```

### Integration Tests

```rust
#[test]
fn test_full_transition() {
    let frames: Vec<Frame> = shimr::morph("hi", "bye")
        .generations(5)
        .collect();

    assert_eq!(frames.len(), 5);

    // First frame should look like "hi"
    // Last frame should look like "bye"
    // Middle frames should be transitioning
}
```

### Visual Tests

```rust
#[test]
fn visual_test_morph() {
    let frames: Vec<String> = shimr::morph("hello", "world")
        .generations(10)
        .glyph_set(GlyphSet::classic())
        .map(|f| f.render())
        .collect();

    insta::assert_debug_snapshot!(frames);
}
```

## Performance Targets

- Text conversion: < 1ms for typical strings
- Frame rendering: < 2ms for 100x100 grid
- Total frame time: < 16ms (60fps)

## Implementation Order

1. **Frame type** - simple, no dependencies
2. **TextGrid** - with basic char bitmaps
3. **MorphStrategy** - at least DissolveAndForm
4. **Transition** - iterator implementation
5. **MorphBuilder** - public API
6. **Tests** - unit, integration, visual

## Open Questions

1. **Font rendering?** Use proper font or simple bitmaps?
   - Decision: Simple 5x5 bitmaps for MVP, font rendering later

2. **Grid sizing?** What if start/end have different sizes?
   - Decision: Use max(width, height) and center text

3. **Seed pattern?** How to inject end state into grid?
   - Decision: Sparse pattern (every 3rd cell) initially

4. **Frame timing?** Include timing info in frames?
   - Decision: No, keep frames simple. User controls timing.

## Example Output

```
Frame 0:  █████  ███
          █   █  █
          █████  ███
          █   █  █
          █   █  ███

Frame 5:  ▓▓░░▒  ░▓▒
          ▒   ░  ░
          ░▓░▒░  ▓▒░
          ░   ▒  ▓
          ▒   ░  ░▓░

Frame 10: █   █  ███  ████  █     ████
          █ █ █  █  █ █  █  █     █  █
          █ █ █  █  █ ████  █     █  █
          █   █  █  █ █  █  █     █  █
          █   █  ███  █  █  ████  ████
```

## Next Steps

1. Implement Frame type
2. Create basic char bitmaps
3. Implement TextGrid conversion
4. Build Transition iterator
5. Create MorphBuilder API
6. Test end-to-end transitions
