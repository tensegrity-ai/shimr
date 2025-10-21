use super::Grid;
use crate::rules::Rule;

/// Cellular automata simulation engine
///
/// Orchestrates grid updates using CA rules and manages decay transitions.
pub struct Automata {
    grid: Grid,
    rule: Box<dyn Rule>,
    decay_rate: u8,
}

impl Automata {
    /// Create new automata with a grid and rule
    pub fn new(grid: Grid, rule: Box<dyn Rule>) -> Self {
        Automata {
            grid,
            rule,
            decay_rate: 32, // Default decay speed
        }
    }

    /// Set decay rate (how fast cells transition visually)
    pub fn with_decay_rate(mut self, rate: u8) -> Self {
        self.decay_rate = rate;
        self
    }

    /// Run one generation
    ///
    /// This is the hot path - applies rules, updates grid, and updates decay values.
    pub fn step(&mut self) {
        // Apply rule to compute next generation
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let alive = self.rule.apply(&self.grid, x, y);
                let mut cell = self.grid.get(x, y);
                cell.set_alive(alive);
                self.grid.set_next(x, y, cell);
            }
        }

        // Swap buffers (zero-copy)
        self.grid.swap_buffers();

        // Update decay values for smooth transitions
        for cell in self.grid.current_mut() {
            cell.update_decay(self.decay_rate);
        }
    }

    /// Get reference to grid
    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    /// Get mutable reference to grid (for setup)
    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Cell;
    use crate::rules::Conway;

    #[test]
    fn test_automata_step() {
        let mut grid = Grid::new(5, 5);

        // Create blinker
        grid.set(2, 1, Cell::alive());
        grid.set(2, 2, Cell::alive());
        grid.set(2, 3, Cell::alive());

        let mut automata = Automata::new(grid, Box::new(Conway));

        // Step once - blinker should rotate
        automata.step();

        // Should be horizontal now
        assert!(automata.grid().get(1, 2).is_alive());
        assert!(automata.grid().get(2, 2).is_alive());
        assert!(automata.grid().get(3, 2).is_alive());

        // Vertical should be dead
        assert!(!automata.grid().get(2, 1).is_alive());
        assert!(!automata.grid().get(2, 3).is_alive());
    }

    #[test]
    fn test_automata_decay() {
        let mut grid = Grid::new(4, 4);

        // Create a stable 2x2 block with low decay
        grid.set(1, 1, Cell::with_decay(true, 0));
        grid.set(2, 1, Cell::with_decay(true, 0));
        grid.set(1, 2, Cell::with_decay(true, 0));
        grid.set(2, 2, Cell::with_decay(true, 0));

        let mut automata = Automata::new(grid, Box::new(Conway))
            .with_decay_rate(64);

        // Step should update decay (block stays alive)
        automata.step();

        // Decay should have increased (moving toward 255)
        let cell = automata.grid().get(1, 1);
        assert!(cell.decay() > 0, "decay should increase for alive cells");
        assert!(cell.is_alive(), "block should still be alive");
    }
}
