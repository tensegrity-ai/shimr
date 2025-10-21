use crate::core::Grid;

/// Cellular automata rule
///
/// Defines how cells transition from one generation to the next.
pub trait Rule: Send + Sync {
    /// Compute next state for cell at (x, y)
    ///
    /// Given the current grid state, return whether the cell
    /// at position (x, y) should be alive in the next generation.
    ///
    /// # Arguments
    /// * `grid` - Current grid state
    /// * `x` - X coordinate of cell
    /// * `y` - Y coordinate of cell
    ///
    /// # Returns
    /// `true` if cell should be alive, `false` if dead
    fn apply(&self, grid: &Grid, x: usize, y: usize) -> bool;

    /// Rule name for debugging
    fn name(&self) -> &str {
        "CustomRule"
    }
}
