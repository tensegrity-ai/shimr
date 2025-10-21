/// RGB color with ANSI escape code support
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color from RGB values
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Convert to ANSI 24-bit color escape code (foreground)
    pub fn to_ansi_fg(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    /// Convert to ANSI 24-bit color escape code (background)
    pub fn to_ansi_bg(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }

    /// ANSI reset code
    pub const fn reset() -> &'static str {
        "\x1b[0m"
    }

    /// Wrap text in ANSI foreground color
    pub fn colorize(&self, text: &str) -> String {
        format!("{}{}{}", self.to_ansi_fg(), text, Self::reset())
    }

    /// Interpolate between two colors
    ///
    /// # Arguments
    /// * `other` - Target color
    /// * `t` - Interpolation factor (0.0 = self, 1.0 = other)
    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
        }
    }

    // Common colors
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    pub const CYAN: Color = Color::rgb(0, 255, 255);
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    pub const YELLOW: Color = Color::rgb(255, 255, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::rgb(100, 150, 200);
        assert_eq!(color.r, 100);
        assert_eq!(color.g, 150);
        assert_eq!(color.b, 200);
    }

    #[test]
    fn test_ansi_fg() {
        let color = Color::rgb(255, 100, 50);
        let ansi = color.to_ansi_fg();
        assert_eq!(ansi, "\x1b[38;2;255;100;50m");
    }

    #[test]
    fn test_ansi_bg() {
        let color = Color::rgb(50, 100, 150);
        let ansi = color.to_ansi_bg();
        assert_eq!(ansi, "\x1b[48;2;50;100;150m");
    }

    #[test]
    fn test_colorize() {
        let color = Color::RED;
        let text = color.colorize("hello");
        assert!(text.contains("hello"));
        assert!(text.contains("\x1b["));
        assert!(text.ends_with("\x1b[0m"));
    }

    #[test]
    fn test_lerp() {
        let black = Color::BLACK;
        let white = Color::WHITE;

        let gray = black.lerp(&white, 0.5);
        assert!(gray.r > 100 && gray.r < 160); // Roughly middle
        assert!(gray.g > 100 && gray.g < 160);
        assert!(gray.b > 100 && gray.b < 160);
    }

    #[test]
    fn test_lerp_extremes() {
        let red = Color::RED;
        let blue = Color::BLUE;

        assert_eq!(red.lerp(&blue, 0.0), red);
        assert_eq!(red.lerp(&blue, 1.0), blue);
    }
}
