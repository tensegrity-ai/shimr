use super::{CharBitmap, FontRenderer};
use std::collections::HashMap;

pub struct FigletFont {
    glyphs: HashMap<char, CharBitmap>,
    height: usize,
}

impl FigletFont {
    pub fn bundled() -> Self {
        let font_data = include_str!("../../fonts/banner.flf");
        Self::parse(font_data).expect("Bundled font should be valid")
    }

    pub fn from_str(data: &str) -> Result<Self, String> {
        Self::parse(data)
    }

    fn parse(data: &str) -> Result<Self, String> {
        let mut lines = data.lines().peekable();

        let header = lines.next().ok_or("Empty font file")?;
        let height = Self::parse_header(header)?;

        let mut glyphs = HashMap::new();

        // Skip comment lines
        while let Some(line) = lines.peek() {
            if line.starts_with("$$") {
                lines.next();
                break;
            }
            lines.next();
        }

        // Read character definitions
        // Format: character code on one line (e.g., "0  $@"), followed by height lines
        while let Some(line) = lines.next() {
            if line.trim().is_empty() {
                continue;
            }

            // Skip $$ markers
            if line.starts_with("$$") {
                continue;
            }

            // Extract character from line like "A  $@"
            if let Some(ch) = Self::extract_char(line) {
                let bitmap = Self::read_glyph(&mut lines, height)?;
                glyphs.insert(ch, bitmap);
            }
        }

        Ok(FigletFont { glyphs, height })
    }

    fn parse_header(header: &str) -> Result<usize, String> {
        let parts: Vec<&str> = header.split_whitespace().collect();
        if parts.len() < 2 {
            return Err("Invalid header".to_string());
        }

        if !parts[0].starts_with("flf2") {
            return Err("Not a FIGlet 2 file".to_string());
        }

        let height_str = parts[1];
        height_str.parse::<usize>()
            .map_err(|_| "Invalid height in header".to_string())
    }

    fn extract_char(line: &str) -> Option<char> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return None;
        }

        // First character on the line is the character code
        // Format: "A  $@" or "0  $@"
        trimmed.chars().next()
    }

    fn read_glyph<'a, I>(lines: &mut I, height: usize) -> Result<CharBitmap, String>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut glyph_lines = Vec::new();
        let mut max_width = 0;

        for _ in 0..height {
            if let Some(line) = lines.next() {
                let clean = Self::clean_glyph_line(line);
                max_width = max_width.max(clean.len());
                glyph_lines.push(clean);
            } else {
                return Err("Unexpected end of glyph".to_string());
            }
        }

        let mut bitmap = CharBitmap::new(max_width, height);

        for (y, line) in glyph_lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if !ch.is_whitespace() {
                    bitmap.set(x, y, true);
                }
            }
        }

        Ok(bitmap)
    }

    fn clean_glyph_line(line: &str) -> String {
        line.trim_end_matches('@')
            .trim_end_matches('$')
            .trim_end_matches('@')
            .to_string()
    }
}

impl FontRenderer for FigletFont {
    fn render_char(&self, ch: char) -> CharBitmap {
        self.glyphs.get(&ch)
            .cloned()
            .unwrap_or_else(|| {
                self.glyphs.get(&'?')
                    .cloned()
                    .unwrap_or_else(|| CharBitmap::new(3, self.height))
            })
    }

    fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundled_font() {
        let font = FigletFont::bundled();
        assert!(font.height > 0);
        println!("Font height: {}", font.height);
        println!("Loaded {} glyphs", font.glyphs.len());

        let a_bitmap = font.render_char('A');
        assert!(a_bitmap.width > 0);
        assert_eq!(a_bitmap.height, font.height);
    }

    #[test]
    fn test_fallback_char() {
        let font = FigletFont::bundled();
        let unknown = font.render_char('\u{1F600}');
        assert!(unknown.width > 0);
    }

    #[test]
    fn test_char_rendering() {
        let font = FigletFont::bundled();
        let bitmap = font.render_char('A');

        // Print the bitmap for debugging
        for y in 0..bitmap.height {
            for x in 0..bitmap.width {
                print!("{}", if bitmap.get(x, y) { '#' } else { ' ' });
            }
            println!();
        }

        // Should have some pixels set
        let pixels_set = bitmap.pixels.iter().filter(|&&p| p).count();
        assert!(pixels_set > 0, "Character 'A' should have some pixels set");
    }
}
