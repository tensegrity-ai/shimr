/// A single cell in the cellular automata grid
///
/// Each cell has an alive/dead state and a decay value for smooth visual transitions.
/// The decay value (0-255) determines which glyph to render:
/// - 255 = fully alive (solid block)
/// - 0 = fully dead (empty space)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// Current alive state
    alive: bool,

    /// Decay value for smooth transitions (0-255)
    /// 255 = fully alive, 0 = fully dead
    /// Used for glyph interpolation
    decay: u8,
}

impl Cell {
    /// Create a new alive cell with full decay
    #[inline]
    pub fn alive() -> Self {
        Cell {
            alive: true,
            decay: 255,
        }
    }

    /// Create a new dead cell with zero decay
    #[inline]
    pub fn dead() -> Self {
        Cell {
            alive: false,
            decay: 0,
        }
    }

    /// Create a cell with specific alive state and decay value
    #[inline]
    pub fn with_decay(alive: bool, decay: u8) -> Self {
        Cell { alive, decay }
    }

    /// Update decay value based on alive state
    ///
    /// Decay gradually moves toward target (255 if alive, 0 if dead).
    /// This creates smooth visual transitions as cells die/spawn.
    ///
    /// # Arguments
    /// * `rate` - How fast to change decay (higher = faster transitions)
    #[inline]
    pub fn update_decay(&mut self, rate: u8) {
        let target = if self.alive { 255 } else { 0 };

        if self.decay < target {
            self.decay = self.decay.saturating_add(rate);
        } else if self.decay > target {
            self.decay = self.decay.saturating_sub(rate);
        }
    }

    /// Check if cell is alive
    #[inline]
    pub fn is_alive(&self) -> bool {
        self.alive
    }

    /// Get current decay value (0-255)
    #[inline]
    pub fn decay(&self) -> u8 {
        self.decay
    }

    /// Set alive state (for internal use during CA updates)
    #[inline]
    pub(crate) fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::dead()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_creation() {
        let alive = Cell::alive();
        assert!(alive.is_alive());
        assert_eq!(alive.decay(), 255);

        let dead = Cell::dead();
        assert!(!dead.is_alive());
        assert_eq!(dead.decay(), 0);

        let custom = Cell::with_decay(true, 128);
        assert!(custom.is_alive());
        assert_eq!(custom.decay(), 128);
    }

    #[test]
    fn test_decay_toward_alive() {
        let mut cell = Cell::dead();
        cell.set_alive(true);

        // Decay should increase toward 255
        for _ in 0..10 {
            let prev = cell.decay();
            cell.update_decay(32);
            if prev < 255 {
                assert!(cell.decay() >= prev);
            }
        }
    }

    #[test]
    fn test_decay_toward_dead() {
        let mut cell = Cell::alive();
        cell.set_alive(false);

        // Decay should decrease toward 0
        for _ in 0..10 {
            let prev = cell.decay();
            cell.update_decay(32);
            if prev > 0 {
                assert!(cell.decay() <= prev);
            }
        }
    }

    #[test]
    fn test_decay_saturation() {
        let mut cell = Cell::with_decay(true, 250);

        // Should saturate at 255, not overflow
        cell.update_decay(100);
        assert_eq!(cell.decay(), 255);

        cell.set_alive(false);
        cell.update_decay(255);
        assert_eq!(cell.decay(), 0);
    }
}
