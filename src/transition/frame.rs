/// A single rendered frame
///
/// Contains the visual representation of a grid state as glyphs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub glyphs: Vec<char>,
}

impl Frame {
    /// Create a new frame filled with spaces
    pub fn new(width: usize, height: usize) -> Self {
        Frame {
            width,
            height,
            glyphs: vec![' '; width * height],
        }
    }

    /// Get glyph at (x, y)
    ///
    /// Returns space if out of bounds
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> char {
        if x >= self.width || y >= self.height {
            return ' ';
        }
        self.glyphs[y * self.width + x]
    }

    /// Set glyph at (x, y)
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, glyph: char) {
        if x < self.width && y < self.height {
            self.glyphs[y * self.width + x] = glyph;
        }
    }

    /// Render frame as string (for terminal output)
    ///
    /// Rows are separated by newlines.
    pub fn render(&self) -> String {
        let mut output = String::with_capacity((self.width + 1) * self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                output.push(self.get(x, y));
            }
            if y < self.height - 1 {
                output.push('\n');
            }
        }

        output
    }

    /// Render frame into pre-allocated buffer (zero-allocation)
    pub fn render_into(&self, buffer: &mut String) {
        buffer.clear();
        buffer.reserve((self.width + 1) * self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                buffer.push(self.get(x, y));
            }
            if y < self.height - 1 {
                buffer.push('\n');
            }
        }
    }
}

impl std::fmt::Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_creation() {
        let frame = Frame::new(5, 3);
        assert_eq!(frame.width, 5);
        assert_eq!(frame.height, 3);
        assert_eq!(frame.glyphs.len(), 15);

        // Should be filled with spaces
        for &glyph in &frame.glyphs {
            assert_eq!(glyph, ' ');
        }
    }

    #[test]
    fn test_frame_set_get() {
        let mut frame = Frame::new(3, 3);

        frame.set(0, 0, 'A');
        frame.set(1, 1, 'B');
        frame.set(2, 2, 'C');

        assert_eq!(frame.get(0, 0), 'A');
        assert_eq!(frame.get(1, 1), 'B');
        assert_eq!(frame.get(2, 2), 'C');
    }

    #[test]
    fn test_frame_render() {
        let mut frame = Frame::new(3, 2);

        frame.set(0, 0, 'A');
        frame.set(1, 0, 'B');
        frame.set(2, 0, 'C');
        frame.set(0, 1, 'X');
        frame.set(1, 1, 'Y');
        frame.set(2, 1, 'Z');

        let rendered = frame.render();
        assert_eq!(rendered, "ABC\nXYZ");
    }

    #[test]
    fn test_frame_render_into() {
        let mut frame = Frame::new(2, 2);
        frame.set(0, 0, '█');
        frame.set(1, 1, '█');

        let mut buffer = String::new();
        frame.render_into(&mut buffer);

        assert_eq!(buffer, "█ \n █");
    }

    #[test]
    fn test_frame_display() {
        let mut frame = Frame::new(2, 2);
        frame.set(0, 0, '#');

        let displayed = format!("{}", frame);
        assert!(displayed.contains('#'));
    }

    #[test]
    fn test_frame_out_of_bounds() {
        let frame = Frame::new(3, 3);

        // Out of bounds should return space
        assert_eq!(frame.get(10, 10), ' ');

        let mut frame = Frame::new(3, 3);
        // Setting out of bounds shouldn't panic
        frame.set(10, 10, 'X');
    }
}
