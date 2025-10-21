use crate::core::Grid;
use super::rule::Rule;

/// Day & Night - Symmetric cellular automata
///
/// Rules:
/// - Live cells with 3, 4, 6, 7, or 8 neighbors survive
/// - Dead cells with 3, 6, 7, or 8 neighbors become alive
///
/// This rule is "symmetric" - the pattern looks the same if you swap
/// alive/dead cells. Creates very different dynamics from Conway,
/// with both explosive growth and stable oscillators.
pub struct DayAndNight;

impl Rule for DayAndNight {
    fn apply(&self, grid: &Grid, x: usize, y: usize) -> bool {
        let neighbors = grid.count_neighbors(x, y);
        let alive = grid.get(x, y).is_alive();

        match (alive, neighbors) {
            // Survival: 3, 4, 6, 7, 8
            (true, 3) | (true, 4) | (true, 6) | (true, 7) | (true, 8) => true,
            // Birth: 3, 6, 7, 8
            (false, 3) | (false, 6) | (false, 7) | (false, 8) => true,
            // Death/stay dead
            _ => false,
        }
    }

    fn name(&self) -> &str {
        "Day & Night"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Grid, Cell};

    #[test]
    fn test_daynight_survival() {
        let mut grid = Grid::new(5, 5);

        // Create a cell with 3 neighbors (should survive)
        grid.set(2, 2, Cell::alive());
        grid.set(1, 1, Cell::alive());
        grid.set(2, 1, Cell::alive());
        grid.set(3, 1, Cell::alive());

        let rule = DayAndNight;

        // Center cell has 3 neighbors - should survive
        assert!(rule.apply(&grid, 2, 2));
    }

    #[test]
    fn test_daynight_birth() {
        let mut grid = Grid::new(5, 5);

        // Create 6 neighbors around a dead cell
        grid.set(1, 1, Cell::alive());
        grid.set(2, 1, Cell::alive());
        grid.set(3, 1, Cell::alive());
        grid.set(1, 2, Cell::alive());
        grid.set(3, 2, Cell::alive());
        grid.set(2, 3, Cell::alive());

        let rule = DayAndNight;

        // Center cell has 6 neighbors - should be born
        assert!(rule.apply(&grid, 2, 2));
    }

    #[test]
    fn test_daynight_death() {
        let mut grid = Grid::new(5, 5);

        // Single cell with no neighbors
        grid.set(2, 2, Cell::alive());

        let rule = DayAndNight;

        // Should die (0 neighbors)
        assert!(!rule.apply(&grid, 2, 2));
    }

    #[test]
    fn test_daynight_four_neighbors() {
        let mut grid = Grid::new(5, 5);

        // Create alive cell with 4 neighbors (unique to Day&Night survival)
        grid.set(2, 2, Cell::alive());
        grid.set(1, 1, Cell::alive());
        grid.set(2, 1, Cell::alive());
        grid.set(3, 1, Cell::alive());
        grid.set(1, 2, Cell::alive());

        let rule = DayAndNight;

        // Should survive (4 neighbors - doesn't survive in Conway)
        assert!(rule.apply(&grid, 2, 2));
    }
}
