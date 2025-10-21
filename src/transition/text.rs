use crate::core::{Grid, Cell};
use crate::fonts::{FontRenderer, FigletFont};

pub fn text_to_grid(text: &str, font: &dyn FontRenderer) -> Grid {
    if text.is_empty() {
        return Grid::new(1, font.height());
    }

    let chars: Vec<char> = text.chars().collect();
    let spacing = 1;
    let height = font.height();

    let bitmaps: Vec<_> = chars.iter()
        .map(|&ch| font.render_char(ch))
        .collect();

    let total_width: usize = bitmaps.iter().map(|b| b.width).sum::<usize>()
        + spacing * (chars.len().saturating_sub(1));

    let mut grid = Grid::new(total_width, height);

    let mut x_offset = 0;
    for bitmap in bitmaps {
        for y in 0..bitmap.height.min(height) {
            for x in 0..bitmap.width {
                if bitmap.get(x, y) {
                    grid.set(x_offset + x, y, Cell::alive());
                }
            }
        }
        x_offset += bitmap.width + spacing;
    }

    grid
}

pub fn default_font() -> FigletFont {
    FigletFont::bundled()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_to_grid() {
        let font = FigletFont::bundled();
        let grid = text_to_grid("A", &font);

        assert_eq!(grid.height(), font.height());
        assert!(grid.width() > 0);

        let alive_count = grid.cells()
            .filter(|(_, _, cell)| cell.is_alive())
            .count();
        assert!(alive_count > 0, "Should have alive cells for 'A'");
    }

    #[test]
    fn test_text_to_grid_multiple_chars() {
        let font = FigletFont::bundled();
        let grid = text_to_grid("AB", &font);

        assert_eq!(grid.height(), font.height());
        assert!(grid.width() > 0);
    }

    #[test]
    fn test_text_to_grid_empty() {
        let font = FigletFont::bundled();
        let grid = text_to_grid("", &font);

        assert_eq!(grid.width(), 1);
        assert_eq!(grid.height(), font.height());
    }

    #[test]
    fn test_text_to_grid_numbers() {
        let font = FigletFont::bundled();
        let grid = text_to_grid("123", &font);

        assert!(grid.width() > 0);
        assert_eq!(grid.height(), font.height());
    }

    #[test]
    fn test_text_to_grid_punctuation() {
        let font = FigletFont::bundled();
        let grid = text_to_grid("Hello!", &font);

        assert!(grid.width() > 0);
        let alive_count = grid.cells()
            .filter(|(_, _, cell)| cell.is_alive())
            .count();
        assert!(alive_count > 0);
    }
}
