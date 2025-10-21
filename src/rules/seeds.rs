use crate::core::Grid;
use super::rule::Rule;

/// Seeds - Creates beautiful trailing patterns
///
/// Rules:
/// - ALL live cells die in the next generation
/// - Dead cells with exactly 2 neighbors become alive
/// - All other cells stay dead
///
/// This creates "seeds" that sprout and immediately die, leaving trails.
/// Very different from Conway - no stable patterns, everything is transient.
/// Great for creating flowing, organic animations.
pub struct Seeds;

impl Rule for Seeds {
    fn apply(&self, grid: &Grid, x: usize, y: usize) -> bool {
        let neighbors = grid.count_neighbors(x, y);
        let alive = grid.get(x, y).is_alive();

        // All living cells die
        // Only birth on exactly 2 neighbors
        !alive && neighbors == 2
    }

    fn name(&self) -> &str {
        "Seeds"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Grid, Cell};

    #[test]
    fn test_seeds_all_cells_die() {
        let mut grid = Grid::new(5, 5);

        // Create any alive cell
        grid.set(2, 2, Cell::alive());

        let rule = Seeds;

        // All alive cells should die
        assert!(!rule.apply(&grid, 2, 2));
    }

    #[test]
    fn test_seeds_birth_on_two() {
        let mut grid = Grid::new(5, 5);

        // Create exactly 2 neighbors
        grid.set(1, 1, Cell::alive());
        grid.set(2, 1, Cell::alive());

        let rule = Seeds;

        // Dead cell at (1,2) has exactly 2 neighbors - should be born
        assert!(rule.apply(&grid, 1, 2));

        // Dead cell at (2,2) has exactly 2 neighbors - should be born
        assert!(rule.apply(&grid, 2, 2));
    }

    #[test]
    fn test_seeds_no_stable_patterns() {
        let mut grid = Grid::new(5, 5);

        // Even a stable block in Conway dies in Seeds
        grid.set(1, 1, Cell::alive());
        grid.set(2, 1, Cell::alive());
        grid.set(1, 2, Cell::alive());
        grid.set(2, 2, Cell::alive());

        let rule = Seeds;

        // All cells in block should die
        assert!(!rule.apply(&grid, 1, 1));
        assert!(!rule.apply(&grid, 2, 1));
        assert!(!rule.apply(&grid, 1, 2));
        assert!(!rule.apply(&grid, 2, 2));
    }
}
