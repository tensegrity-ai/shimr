use crate::core::Grid;
use super::rule::Rule;

/// HighLife - Conway variant with replicators
///
/// Rules:
/// - Any live cell with 2-3 live neighbors survives (like Conway)
/// - Any dead cell with 3 or 6 live neighbors becomes alive
/// - All other cells die or stay dead
///
/// The B6 rule (birth on 6) creates "replicators" - patterns that copy themselves.
/// More chaotic and dynamic than classic Conway.
pub struct HighLife;

impl Rule for HighLife {
    fn apply(&self, grid: &Grid, x: usize, y: usize) -> bool {
        let neighbors = grid.count_neighbors(x, y);
        let alive = grid.get(x, y).is_alive();

        match (alive, neighbors) {
            (true, 2) | (true, 3) => true,      // Survival (same as Conway)
            (false, 3) | (false, 6) => true,    // Birth (B36 rule)
            _ => false,                          // Death
        }
    }

    fn name(&self) -> &str {
        "HighLife"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Grid, Cell};

    #[test]
    fn test_highlife_conway_compatibility() {
        // Should behave like Conway for standard patterns
        let mut grid = Grid::new(5, 5);

        // 2x2 block should be stable
        grid.set(1, 1, Cell::alive());
        grid.set(2, 1, Cell::alive());
        grid.set(1, 2, Cell::alive());
        grid.set(2, 2, Cell::alive());

        let rule = HighLife;

        // Block should remain stable
        assert!(rule.apply(&grid, 1, 1));
        assert!(rule.apply(&grid, 2, 1));
        assert!(rule.apply(&grid, 1, 2));
        assert!(rule.apply(&grid, 2, 2));
    }

    #[test]
    fn test_highlife_birth_on_six() {
        // Test the B6 rule unique to HighLife
        let mut grid = Grid::new(5, 5);

        // Create a pattern with 6 neighbors around center
        grid.set(1, 1, Cell::alive());
        grid.set(2, 1, Cell::alive());
        grid.set(3, 1, Cell::alive());
        grid.set(1, 2, Cell::alive());
        grid.set(3, 2, Cell::alive());
        grid.set(2, 3, Cell::alive());

        let rule = HighLife;

        // Center cell should be born (6 neighbors)
        assert!(rule.apply(&grid, 2, 2));
    }
}
