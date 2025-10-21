use crate::core::{Automata, Grid, Cell};
use crate::glyphs::GlyphSet;
use super::Frame;

/// Transition animation between two states
///
/// Generates frames by running cellular automata simulation,
/// mapping cell decay values to glyphs for smooth visual transitions.
pub struct Transition {
    automata: Automata,
    glyph_set: GlyphSet,
    current_gen: usize,
    max_gen: usize,
    frame_buffer: Frame,
}

impl Transition {
    /// Create a new transition
    ///
    /// # Arguments
    /// * `automata` - Initialized automata with starting grid
    /// * `glyph_set` - Glyph set for rendering
    /// * `generations` - Number of generations to run
    pub fn new(
        automata: Automata,
        glyph_set: GlyphSet,
        generations: usize,
    ) -> Self {
        let grid = automata.grid();
        let frame_buffer = Frame::new(grid.width(), grid.height());

        Transition {
            automata,
            glyph_set,
            current_gen: 0,
            max_gen: generations,
            frame_buffer,
        }
    }

    /// Render current grid state to frame buffer
    fn render_frame(&mut self) {
        let grid = self.automata.grid();

        for (x, y, cell) in grid.cells() {
            let glyph = self.glyph_set.map(cell.decay());
            self.frame_buffer.set(x, y, glyph);
        }
    }
}

impl Iterator for Transition {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_gen >= self.max_gen {
            return None;
        }

        // Render current state
        self.render_frame();

        // Step simulation for next frame
        self.automata.step();
        self.current_gen += 1;

        // Return clone of frame
        Some(self.frame_buffer.clone())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.max_gen.saturating_sub(self.current_gen);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for Transition {
    fn len(&self) -> usize {
        self.max_gen.saturating_sub(self.current_gen)
    }
}

/// Merge two grids by overlaying them
///
/// For cells that exist in both grids, uses OR logic (alive if either is alive).
/// Used to create initial state for morphing.
pub fn merge_grids(start: &Grid, end: &Grid) -> Grid {
    let width = start.width().max(end.width());
    let height = start.height().max(end.height());

    let mut merged = Grid::new(width, height);

    // Copy start grid
    for (x, y, cell) in start.cells() {
        if cell.is_alive() {
            merged.set(x, y, Cell::alive());
        }
    }

    // Overlay end grid (with lower decay for visual distinction)
    for (x, y, cell) in end.cells() {
        if cell.is_alive() {
            // If already alive from start, keep it
            // Otherwise add with low decay
            if !merged.get(x, y).is_alive() {
                merged.set(x, y, Cell::with_decay(true, 128));
            }
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Conway;

    #[test]
    fn test_transition_creation() {
        let grid = Grid::new(5, 5);
        let automata = Automata::new(grid, Box::new(Conway));
        let glyph_set = GlyphSet::classic();

        let transition = Transition::new(automata, glyph_set, 10);

        assert_eq!(transition.len(), 10);
    }

    #[test]
    fn test_transition_iteration() {
        let mut grid = Grid::new(5, 5);

        // Create a blinker
        grid.set(2, 1, Cell::alive());
        grid.set(2, 2, Cell::alive());
        grid.set(2, 3, Cell::alive());

        let automata = Automata::new(grid, Box::new(Conway));
        let glyph_set = GlyphSet::ascii();

        let transition = Transition::new(automata, glyph_set, 5);

        let frames: Vec<Frame> = transition.collect();

        assert_eq!(frames.len(), 5);

        // Each frame should be the right size
        for frame in &frames {
            assert_eq!(frame.width, 5);
            assert_eq!(frame.height, 5);
        }
    }

    #[test]
    fn test_transition_exact_size() {
        let grid = Grid::new(3, 3);
        let automata = Automata::new(grid, Box::new(Conway));
        let glyph_set = GlyphSet::classic();

        let mut transition = Transition::new(automata, glyph_set, 10);

        assert_eq!(transition.len(), 10);

        // After taking some frames
        transition.next();
        transition.next();

        assert_eq!(transition.len(), 8);
    }

    #[test]
    fn test_merge_grids() {
        let mut start = Grid::new(5, 5);
        start.set(0, 0, Cell::alive());
        start.set(1, 1, Cell::alive());

        let mut end = Grid::new(5, 5);
        end.set(3, 3, Cell::alive());
        end.set(4, 4, Cell::alive());

        let merged = merge_grids(&start, &end);

        // Should have cells from both grids
        assert!(merged.get(0, 0).is_alive()); // from start
        assert!(merged.get(1, 1).is_alive()); // from start
        assert!(merged.get(3, 3).is_alive()); // from end
        assert!(merged.get(4, 4).is_alive()); // from end
    }
}
