use crate::core::Grid;
use super::rule::Rule;

/// Conway's Game of Life
///
/// Classic cellular automata rules:
/// - Any live cell with 2-3 live neighbors survives
/// - Any dead cell with exactly 3 live neighbors becomes alive
/// - All other cells die or stay dead
pub struct Conway;

impl Rule for Conway {
    fn apply(&self, grid: &Grid, x: usize, y: usize) -> bool {
        let neighbors = grid.count_neighbors(x, y);
        let alive = grid.get(x, y).is_alive();

        match (alive, neighbors) {
            (true, 2) | (true, 3) => true, // Survival
            (false, 3) => true,             // Birth
            _ => false,                     // Death
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
    fn test_conway_stable_block() {
        // 2x2 block is a still life (should stay stable)
        //
        // Grid layout (5x5):
        // . . . . .
        // . █ █ . .
        // . █ █ . .
        // . . . . .
        // . . . . .

        let mut grid = Grid::new(5, 5);

        // Create 2x2 block at (1,1)
        grid.set(1, 1, Cell::alive());
        grid.set(2, 1, Cell::alive());
        grid.set(1, 2, Cell::alive());
        grid.set(2, 2, Cell::alive());

        let rule = Conway;

        // Apply rule to all cells and store in next buffer
        for y in 0..5 {
            for x in 0..5 {
                let alive = rule.apply(&grid, x, y);
                grid.set_next(x, y, if alive { Cell::alive() } else { Cell::dead() });
            }
        }

        grid.swap_buffers();

        // Block should remain stable
        assert!(grid.get(1, 1).is_alive());
        assert!(grid.get(2, 1).is_alive());
        assert!(grid.get(1, 2).is_alive());
        assert!(grid.get(2, 2).is_alive());

        // Everything else should be dead
        assert!(!grid.get(0, 0).is_alive());
        assert!(!grid.get(3, 3).is_alive());
        assert!(!grid.get(0, 1).is_alive());
    }

    #[test]
    fn test_conway_blinker() {
        // Blinker oscillates with period 2
        //
        // Generation 0:        Generation 1:
        // . . . . .            . . . . .
        // . . █ . .            . . . . .
        // . . █ . .    --->    . █ █ █ .
        // . . █ . .            . . . . .
        // . . . . .            . . . . .

        let mut grid = Grid::new(5, 5);

        // Create vertical blinker at x=2
        grid.set(2, 1, Cell::alive());
        grid.set(2, 2, Cell::alive());
        grid.set(2, 3, Cell::alive());

        let rule = Conway;

        // Apply one generation
        for y in 0..5 {
            for x in 0..5 {
                let alive = rule.apply(&grid, x, y);
                grid.set_next(x, y, if alive { Cell::alive() } else { Cell::dead() });
            }
        }

        grid.swap_buffers();

        // Should now be horizontal
        assert!(grid.get(1, 2).is_alive());
        assert!(grid.get(2, 2).is_alive());
        assert!(grid.get(3, 2).is_alive());

        // Vertical positions should be dead
        assert!(!grid.get(2, 1).is_alive());
        assert!(!grid.get(2, 3).is_alive());
    }

    #[test]
    fn test_conway_death() {
        // Single cell should die (underpopulation)
        let mut grid = Grid::new(3, 3);
        grid.set(1, 1, Cell::alive());

        let rule = Conway;
        let next_state = rule.apply(&grid, 1, 1);

        assert!(!next_state);
    }

    #[test]
    fn test_conway_birth() {
        // Dead cell with 3 neighbors should be born
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, Cell::alive());
        grid.set(1, 0, Cell::alive());
        grid.set(0, 1, Cell::alive());

        let rule = Conway;
        // Center cell (1,1) should be born
        let next_state = rule.apply(&grid, 1, 1);

        assert!(next_state);
    }
}
