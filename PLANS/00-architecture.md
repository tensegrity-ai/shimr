# Architecture Plan

## Overview

shimr is a terminal transition effects library built around cellular automata. The architecture prioritizes performance (60fps, zero-allocation render loop) while maintaining flexibility and composability.

## Core Principles

1. **Zero-allocation render loop** - all buffers pre-allocated, no heap allocation during frame generation
2. **Framework agnostic** - core logic independent of TUI frameworks
3. **Composable API** - builder pattern for configuration, iterators for frame emission
4. **Extensible rules** - trait-based system for custom CA rules

## Module Structure

```
shimr/
├── lib.rs              # Public API, prelude
├── core/
│   ├── mod.rs
│   ├── grid.rs         # 2D grid abstraction
│   ├── cell.rs         # Cell state representation
│   └── automata.rs     # CA engine
├── rules/
│   ├── mod.rs
│   ├── conway.rs       # Conway's Game of Life
│   └── trait.rs        # Rule trait
├── glyphs/
│   ├── mod.rs
│   ├── sets.rs         # Predefined glyph sets
│   └── decay.rs        # Decay sequences
├── transition/
│   ├── mod.rs
│   ├── morph.rs        # Text morphing
│   ├── builder.rs      # Animation builder
│   └── frames.rs       # Frame iterator
├── backend/
│   ├── mod.rs
│   ├── trait.rs        # Backend abstraction
│   └── raw.rs          # Raw terminal backend
└── integrations/
    ├── ratatui.rs      # Feature-gated
    └── cursive.rs      # Feature-gated
```

## Key Types

### Core Types

```rust
// Cell state (keeps decay level for smooth transitions)
pub struct Cell {
    alive: bool,
    decay: u8,  // 0-255, for glyph interpolation
}

// 2D grid with double buffering
pub struct Grid {
    width: usize,
    height: usize,
    current: Vec<Cell>,
    next: Vec<Cell>,  // for swap-based updates
}

// Cellular automata rule
pub trait Rule {
    fn apply(&self, grid: &Grid, x: usize, y: usize) -> bool;
}
```

### Transition Types

```rust
// Animation state
pub struct Transition {
    grid: Grid,
    rule: Box<dyn Rule>,
    glyph_set: GlyphSet,
    current_gen: usize,
    max_gen: usize,
}

// Frame output
pub struct Frame {
    width: usize,
    height: usize,
    glyphs: Vec<char>,  // pre-allocated, reused
}
```

### Builder API

```rust
pub struct MorphBuilder {
    start: String,
    end: String,
    generations: usize,
    rule: Box<dyn Rule>,
    glyph_set: GlyphSet,
}
```

## Data Flow

```
User Input (text/grid)
    ↓
Builder Configuration
    ↓
Transition::new()
    ↓ (pre-allocate all buffers)
    ↓
Iterator::next() loop:
    ├→ Apply CA rules
    ├→ Update decay values
    ├→ Map to glyphs
    └→ Return Frame
```

## Performance Strategy

### Zero-Allocation Render Loop

1. **Pre-allocation:** All `Vec`s allocated in `Transition::new()`
2. **Buffer reuse:** Double buffering with swap, never reallocate
3. **Glyph caching:** Pre-compute decay → glyph mapping
4. **Stack locals only:** No heap allocation in hot path

### Memory Layout

```rust
// Bad: allocation per frame
fn render_frame(&self) -> Frame {
    Frame { glyphs: self.compute_glyphs() }  // allocates Vec
}

// Good: reuse buffer
fn render_frame(&self, frame: &mut Frame) {
    self.compute_glyphs_into(&mut frame.glyphs);  // fills existing Vec
}

// Better: iterator with pre-allocated buffer
impl Iterator for Transition {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        // self.frame_buffer already allocated
        self.compute_into_buffer();
        Some(self.frame_buffer.clone())  // or return reference?
    }
}
```

### Timing

Target 60fps = ~16.67ms per frame budget:
- CA update: < 5ms
- Glyph mapping: < 2ms
- Render to terminal: < 8ms
- Remaining: 1-2ms buffer

## API Design

### Simple Usage

```rust
use shimr::prelude::*;

let animation = shimr::morph("hello", "world")
    .generations(20)
    .glyph_set(GlyphSet::Cyberpunk)
    .build();

for frame in animation {
    // render frame
}
```

### Advanced Usage

```rust
use shimr::prelude::*;

let animation = Transition::builder()
    .from_grid(start_grid)
    .to_grid(end_grid)
    .rule(Rule::Conway)
    .rule_custom(|grid, x, y| {
        // custom logic
    })
    .glyph_set(GlyphSet::custom(&['█', '▓', '▒', '░', '·', ' ']))
    .generations(30)
    .build();
```

## Open Design Questions

1. **Frame ownership:** Should iterator return `Frame` by value or `&Frame`?
   - By value: easier API, requires clone
   - By reference: zero-copy, but lifetime issues
   - **Decision:** Return by value initially, optimize later if needed

2. **Grid storage:** Flat vec or vec-of-vecs?
   - Flat: better cache locality, manual indexing
   - Nested: easier indexing, cache misses
   - **Decision:** Flat vec with inline indexing helpers

3. **Rule dispatch:** Trait objects vs. enum dispatch?
   - Trait: flexible, extensible, virtual call overhead
   - Enum: monomorphized, fast, limited to built-in rules
   - **Decision:** Start with trait objects, add enum fast-path later

4. **Color support:** Include from start or add later?
   - **Decision:** Add as feature flag in Phase 5

## Testing Strategy

- Unit tests for CA rules (known patterns)
- Integration tests for full transitions
- Benchmarks for render loop performance
- Visual tests (snapshot testing for ASCII output)

## Next Steps

1. Implement core grid and cell types
2. Implement Conway's Game of Life rule
3. Create basic glyph system
4. Build simple transition iterator
5. Add text → grid conversion
