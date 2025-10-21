# Core Engine Implementation Plan

## Overview

The core engine consists of the cellular automata simulation and grid management. This is the performance-critical heart of shimr.

## Components

### 1. Cell (`core/cell.rs`)

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    /// Current alive state
    alive: bool,

    /// Decay value (0-255) for smooth transitions
    /// 255 = fully alive, 0 = fully dead
    /// Used for glyph interpolation
    decay: u8,
}

impl Cell {
    pub fn alive() -> Self {
        Cell { alive: true, decay: 255 }
    }

    pub fn dead() -> Self {
        Cell { alive: false, decay: 0 }
    }

    pub fn with_decay(alive: bool, decay: u8) -> Self {
        Cell { alive, decay }
    }

    /// Update decay value based on alive state
    /// Decay gradually moves toward target (255 or 0)
    pub fn update_decay(&mut self, rate: u8) {
        let target = if self.alive { 255 } else { 0 };
        if self.decay < target {
            self.decay = self.decay.saturating_add(rate);
        } else if self.decay > target {
            self.decay = self.decay.saturating_sub(rate);
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn decay(&self) -> u8 {
        self.decay
    }
}
```

### 2. Grid (`core/grid.rs`)

```rust
pub struct Grid {
    width: usize,
    height: usize,

    // Flat storage for cache locality
    // Index calculation: y * width + x
    current: Vec<Cell>,
    next: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Grid {
            width,
            height,
            current: vec![Cell::dead(); size],
            next: vec![Cell::dead(); size],
        }
    }

    pub fn from_cells(width: usize, height: usize, cells: Vec<Cell>) -> Self {
        assert_eq!(cells.len(), width * height);
        Grid {
            width,
            height,
            current: cells,
            next: vec![Cell::dead(); width * height],
        }
    }

    /// Get cell at (x, y)
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Cell {
        if x >= self.width || y >= self.height {
            return Cell::dead();  // Out of bounds = dead
        }
        self.current[y * self.width + x]
    }

    /// Set cell in next buffer
    #[inline]
    fn set_next(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.next[y * self.width + x] = cell;
        }
    }

    /// Count alive neighbors (8-connected)
    #[inline]
    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        // Check all 8 neighbors
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && ny >= 0 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if self.get(nx, ny).is_alive() {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Swap current and next buffers (zero-copy update)
    #[inline]
    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.current, &mut self.next);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Iterator over all cells with their coordinates
    pub fn cells(&self) -> impl Iterator<Item = (usize, usize, Cell)> + '_ {
        (0..self.height).flat_map(move |y| {
            (0..self.width).map(move |x| {
                (x, y, self.get(x, y))
            })
        })
    }
}
```

### 3. Rule Trait (`rules/trait.rs`)

```rust
/// Cellular automata rule
pub trait Rule: Send + Sync {
    /// Compute next state for cell at (x, y)
    ///
    /// Given the current grid state, return whether the cell
    /// at position (x, y) should be alive in the next generation.
    fn apply(&self, grid: &Grid, x: usize, y: usize) -> bool;

    /// Optional: rule name for debugging
    fn name(&self) -> &str {
        "CustomRule"
    }
}

/// Function-based rule (convenience wrapper)
pub struct FnRule<F>
where
    F: Fn(&Grid, usize, usize) -> bool + Send + Sync,
{
    func: F,
    name: String,
}

impl<F> FnRule<F>
where
    F: Fn(&Grid, usize, usize) -> bool + Send + Sync,
{
    pub fn new(func: F) -> Self {
        FnRule {
            func,
            name: "FnRule".to_string(),
        }
    }

    pub fn with_name(func: F, name: impl Into<String>) -> Self {
        FnRule {
            func,
            name: name.into(),
        }
    }
}

impl<F> Rule for FnRule<F>
where
    F: Fn(&Grid, usize, usize) -> bool + Send + Sync,
{
    fn apply(&self, grid: &Grid, x: usize, y: usize) -> bool {
        (self.func)(grid, x, y)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
```

### 4. Conway's Rule (`rules/conway.rs`)

```rust
use super::trait::Rule;
use crate::core::Grid;

/// Conway's Game of Life
///
/// Rules:
/// - Any live cell with 2-3 live neighbors survives
/// - Any dead cell with exactly 3 live neighbors becomes alive
/// - All other cells die or stay dead
pub struct Conway;

impl Rule for Conway {
    fn apply(&self, grid: &Grid, x: usize, y: usize) -> bool {
        let neighbors = grid.count_neighbors(x, y);
        let alive = grid.get(x, y).is_alive();

        match (alive, neighbors) {
            (true, 2) | (true, 3) => true,   // Survival
            (false, 3) => true,               // Birth
            _ => false,                       // Death
        }
    }

    fn name(&self) -> &str {
        "Conway's Game of Life"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Grid, Cell};

    #[test]
    fn test_blinker() {
        // Blinker pattern oscillates with period 2
        // Generation 0:  .*.  →  Generation 1:  ...
        //                .*.                     ***
        //                .*.                     ...

        let mut grid = Grid::new(3, 3);
        // Set up vertical blinker
        // TODO: implement grid setting
    }

    #[test]
    fn test_block() {
        // Block is a still life (stable)
        // **
        // **

        // TODO: implement
    }
}
```

### 5. Automata Engine (`core/automata.rs`)

```rust
use crate::core::{Grid, Cell};
use crate::rules::Rule;

/// Cellular automata simulation engine
pub struct Automata {
    grid: Grid,
    rule: Box<dyn Rule>,
    decay_rate: u8,
}

impl Automata {
    pub fn new(grid: Grid, rule: Box<dyn Rule>) -> Self {
        Automata {
            grid,
            rule,
            decay_rate: 32,  // Default decay speed
        }
    }

    pub fn with_decay_rate(mut self, rate: u8) -> Self {
        self.decay_rate = rate;
        self
    }

    /// Run one generation
    ///
    /// This is the hot path - must be fast and allocation-free
    pub fn step(&mut self) {
        // Apply rule to compute next generation
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let alive = self.rule.apply(&self.grid, x, y);
                let mut cell = self.grid.get(x, y);
                cell.alive = alive;
                self.grid.set_next(x, y, cell);
            }
        }

        // Swap buffers (zero-copy)
        self.grid.swap_buffers();

        // Update decay values for smooth transitions
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let mut cell = self.grid.get(x, y);
                cell.update_decay(self.decay_rate);
                // Write back to current buffer
                let idx = y * self.grid.width() + x;
                self.grid.current[idx] = cell;
            }
        }
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }
}
```

## Implementation Order

1. **Cell** - Simple, no dependencies
2. **Grid** - Depends on Cell
3. **Rule trait** - Interface definition
4. **Conway's Rule** - First concrete rule
5. **Automata** - Orchestrates everything
6. **Tests** - Known CA patterns (blinker, glider, etc.)

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_cell_decay_alive() {
    let mut cell = Cell::dead();
    cell.alive = true;

    // Decay should increase toward 255
    for _ in 0..10 {
        let prev = cell.decay();
        cell.update_decay(32);
        assert!(cell.decay() >= prev);
    }
}

#[test]
fn test_grid_neighbors() {
    let mut grid = Grid::new(3, 3);
    // Set center cell alive
    // ... (set cells)

    assert_eq!(grid.count_neighbors(1, 1), 0);
}

#[test]
fn test_conway_blinker() {
    // Test oscillating pattern
}

#[test]
fn test_conway_glider() {
    // Test moving pattern
}
```

### Benchmarks

```rust
#[bench]
fn bench_conway_step_100x100(b: &mut Bencher) {
    let grid = Grid::new(100, 100);
    let mut automata = Automata::new(grid, Box::new(Conway));

    b.iter(|| {
        automata.step();
    });
}
```

## Performance Targets

- 100x100 grid: < 1ms per generation
- 200x200 grid: < 5ms per generation
- Memory: O(width × height), no allocations in step()

## Open Questions

1. **Toroidal wrap?** Should grid edges wrap around?
   - Decision: No wrap initially, treat edges as dead cells

2. **Parallel processing?** Use rayon for large grids?
   - Decision: Add as optimization later if needed

3. **SIMD?** Vectorize neighbor counting?
   - Decision: Profile first, optimize if bottleneck

## Next Steps

1. Implement Cell
2. Implement Grid with tests
3. Implement Rule trait and Conway
4. Implement Automata engine
5. Benchmark and profile
