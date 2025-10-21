use super::cell::Cell;

/// 2D grid of cells for cellular automata simulation
///
/// Uses flat storage for better cache locality and double buffering
/// for zero-copy state updates.
pub struct Grid {
    width: usize,
    height: usize,

    /// Current generation (flat storage: index = y * width + x)
    current: Vec<Cell>,

    /// Next generation buffer (for swap-based updates)
    next: Vec<Cell>,
}

impl Grid {
    /// Create a new grid filled with dead cells
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Grid {
            width,
            height,
            current: vec![Cell::dead(); size],
            next: vec![Cell::dead(); size],
        }
    }

    /// Create a grid from a vector of cells
    ///
    /// # Panics
    /// Panics if cells.len() != width * height
    pub fn from_cells(width: usize, height: usize, cells: Vec<Cell>) -> Self {
        assert_eq!(
            cells.len(),
            width * height,
            "cells length must equal width * height"
        );

        Grid {
            width,
            height,
            current: cells,
            next: vec![Cell::dead(); width * height],
        }
    }

    /// Get cell at (x, y)
    ///
    /// Returns dead cell if out of bounds (treats edges as dead)
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Cell {
        if x >= self.width || y >= self.height {
            return Cell::dead();
        }
        self.current[y * self.width + x]
    }

    /// Set cell in current buffer (for initialization)
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.current[y * self.width + x] = cell;
        }
    }

    /// Set cell in next buffer (used during CA updates)
    #[inline]
    pub(crate) fn set_next(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.next[y * self.width + x] = cell;
        }
    }

    /// Count alive neighbors in 8-connected neighborhood
    #[inline]
    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        // Check all 8 neighbors
        for dy in -1..=1_i32 {
            for dx in -1..=1_i32 {
                // Skip center cell
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                // Check bounds and alive state
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
    ///
    /// This is called after computing the next generation to make it current.
    #[inline]
    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.current, &mut self.next);
    }

    /// Get grid width
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get grid height
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get mutable access to current buffer (for decay updates)
    #[inline]
    pub(crate) fn current_mut(&mut self) -> &mut [Cell] {
        &mut self.current
    }

    /// Iterator over all cells with their coordinates
    pub fn cells(&self) -> impl Iterator<Item = (usize, usize, Cell)> + '_ {
        (0..self.height).flat_map(move |y| {
            (0..self.width).map(move |x| (x, y, self.get(x, y)))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(10, 5);
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 5);

        // All cells should be dead
        for y in 0..5 {
            for x in 0..10 {
                assert!(!grid.get(x, y).is_alive());
            }
        }
    }

    #[test]
    fn test_grid_set_get() {
        let mut grid = Grid::new(5, 5);

        // Set a cell alive
        grid.set(2, 3, Cell::alive());
        assert!(grid.get(2, 3).is_alive());

        // Other cells should still be dead
        assert!(!grid.get(0, 0).is_alive());
        assert!(!grid.get(4, 4).is_alive());
    }

    #[test]
    fn test_grid_out_of_bounds() {
        let grid = Grid::new(5, 5);

        // Out of bounds should return dead cell
        assert!(!grid.get(10, 10).is_alive());
        assert!(!grid.get(5, 0).is_alive());
        assert!(!grid.get(0, 5).is_alive());
    }

    #[test]
    fn test_count_neighbors() {
        let mut grid = Grid::new(5, 5);

        // Create a 3x3 block of alive cells
        for y in 1..=3 {
            for x in 1..=3 {
                grid.set(x, y, Cell::alive());
            }
        }

        // Center cell should have 8 neighbors
        assert_eq!(grid.count_neighbors(2, 2), 8);

        // Corner of block should have 3 neighbors
        assert_eq!(grid.count_neighbors(1, 1), 3);

        // Cell outside block should have appropriate neighbors
        assert_eq!(grid.count_neighbors(0, 0), 1);
        assert_eq!(grid.count_neighbors(0, 2), 3);
    }

    #[test]
    fn test_buffer_swap() {
        let mut grid = Grid::new(3, 3);

        // Set current buffer
        grid.set(1, 1, Cell::alive());
        assert!(grid.get(1, 1).is_alive());

        // Set next buffer differently
        grid.set_next(0, 0, Cell::alive());
        grid.set_next(1, 1, Cell::dead());

        // Swap buffers
        grid.swap_buffers();

        // Next buffer is now current
        assert!(grid.get(0, 0).is_alive());
        assert!(!grid.get(1, 1).is_alive());
    }

    #[test]
    fn test_cells_iterator() {
        let mut grid = Grid::new(3, 2);
        grid.set(0, 0, Cell::alive());
        grid.set(2, 1, Cell::alive());

        let alive_cells: Vec<_> = grid
            .cells()
            .filter(|(_, _, cell)| cell.is_alive())
            .collect();

        assert_eq!(alive_cells.len(), 2);
        assert_eq!(alive_cells[0].0, 0); // x
        assert_eq!(alive_cells[0].1, 0); // y
        assert_eq!(alive_cells[1].0, 2);
        assert_eq!(alive_cells[1].1, 1);
    }

    #[test]
    #[should_panic(expected = "cells length must equal width * height")]
    fn test_from_cells_wrong_size() {
        let cells = vec![Cell::dead(); 5];
        Grid::from_cells(3, 3, cells); // 5 cells but 3x3 = 9 expected
    }
}
