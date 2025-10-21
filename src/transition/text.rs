use crate::core::{Grid, Cell};

/// Simple 3x5 bitmap font for ASCII characters
///
/// Each character is represented as a 3-column by 5-row bitmap.
/// This is intentionally simple - proper font rendering can come later.
fn char_to_bitmap(ch: char) -> [[bool; 3]; 5] {
    match ch.to_ascii_uppercase() {
        'A' => [
            [false, true,  false],
            [true,  false, true ],
            [true,  true,  true ],
            [true,  false, true ],
            [true,  false, true ],
        ],
        'C' => [
            [false, true,  true ],
            [true,  false, false],
            [true,  false, false],
            [true,  false, false],
            [false, true,  true ],
        ],
        'E' => [
            [true,  true,  true ],
            [true,  false, false],
            [true,  true,  true ],
            [true,  false, false],
            [true,  true,  true ],
        ],
        'L' => [
            [true,  false, false],
            [true,  false, false],
            [true,  false, false],
            [true,  false, false],
            [true,  true,  true ],
        ],
        'M' => [
            [true,  false, true ],
            [true,  true,  true ],
            [true,  true,  true ],
            [true,  false, true ],
            [true,  false, true ],
        ],
        'O' => [
            [false, true,  false],
            [true,  false, true ],
            [true,  false, true ],
            [true,  false, true ],
            [false, true,  false],
        ],
        'P' => [
            [true,  true,  false],
            [true,  false, true ],
            [true,  true,  false],
            [true,  false, false],
            [true,  false, false],
        ],
        'S' => [
            [false, true,  true ],
            [true,  false, false],
            [false, true,  false],
            [false, false, true ],
            [true,  true,  false],
        ],
        'T' => [
            [true,  true,  true ],
            [false, true,  false],
            [false, true,  false],
            [false, true,  false],
            [false, true,  false],
        ],
        ' ' => [
            [false, false, false],
            [false, false, false],
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ],
        // Default: simple box for unknown chars
        _ => [
            [true,  true,  true ],
            [true,  false, true ],
            [true,  false, true ],
            [true,  false, true ],
            [true,  true,  true ],
        ],
    }
}

/// Convert text string to a grid
///
/// Each character becomes a 3x5 bitmap with 1-column spacing between chars.
pub fn text_to_grid(text: &str) -> Grid {
    if text.is_empty() {
        return Grid::new(1, 5);
    }

    let chars: Vec<char> = text.chars().collect();
    let char_width = 3;
    let char_height = 5;
    let spacing = 1;

    // Calculate total dimensions
    let width = chars.len() * (char_width + spacing) - spacing; // No trailing space
    let height = char_height;

    let mut grid = Grid::new(width, height);

    // Render each character
    for (i, &ch) in chars.iter().enumerate() {
        let x_offset = i * (char_width + spacing);
        let bitmap = char_to_bitmap(ch);

        for y in 0..char_height {
            for x in 0..char_width {
                if bitmap[y][x] {
                    grid.set(x_offset + x, y, Cell::alive());
                }
            }
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_bitmap() {
        let t = char_to_bitmap('T');

        // Top row should be solid
        assert!(t[0][0] && t[0][1] && t[0][2]);

        // Middle column should be solid
        assert!(t[1][1] && t[2][1] && t[3][1] && t[4][1]);
    }

    #[test]
    fn test_text_to_grid() {
        let grid = text_to_grid("T");

        // Should be 3 wide (one char) and 5 tall
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 5);

        // Should have some alive cells
        let alive_count = grid.cells()
            .filter(|(_, _, cell)| cell.is_alive())
            .count();
        assert!(alive_count > 0, "Should have alive cells for 'T'");
    }

    #[test]
    fn test_text_to_grid_multiple_chars() {
        let grid = text_to_grid("HI");

        // 3 + 1 (spacing) + 3 = 7 wide
        assert_eq!(grid.width(), 7);
        assert_eq!(grid.height(), 5);
    }

    #[test]
    fn test_text_to_grid_empty() {
        let grid = text_to_grid("");

        // Should return minimal grid
        assert_eq!(grid.width(), 1);
        assert_eq!(grid.height(), 5);
    }

    #[test]
    fn test_text_to_grid_case_insensitive() {
        let upper = text_to_grid("T");
        let lower = text_to_grid("t");

        // Should be same dimensions
        assert_eq!(upper.width(), lower.width());
        assert_eq!(upper.height(), lower.height());
    }
}
