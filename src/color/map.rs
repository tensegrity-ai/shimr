use super::rgb::Color;

/// Maps decay values (0-255) to colors
pub trait ColorMap {
    /// Map decay value to color
    ///
    /// # Arguments
    /// * `decay` - Decay value (0-255)
    ///   - 255 = fully alive
    ///   - 0 = fully dead
    fn map(&self, decay: u8) -> Color;

    /// Name of the color map
    fn name(&self) -> &str {
        "Custom"
    }
}

/// Tokyo Night color scheme
///
/// Deep purple → cyan → teal gradient
pub struct TokyoNight;

impl ColorMap for TokyoNight {
    fn map(&self, decay: u8) -> Color {
        let t = decay as f32 / 255.0;

        if t < 0.5 {
            // Dead → mid: dark purple → bright purple
            let dark_purple = Color::rgb(42, 38, 80);
            let bright_purple = Color::rgb(187, 154, 247);
            dark_purple.lerp(&bright_purple, t * 2.0)
        } else {
            // Mid → alive: bright purple → cyan
            let bright_purple = Color::rgb(187, 154, 247);
            let cyan = Color::rgb(125, 207, 255);
            bright_purple.lerp(&cyan, (t - 0.5) * 2.0)
        }
    }

    fn name(&self) -> &str {
        "Tokyo Night"
    }
}

/// Matrix green gradient
pub struct Matrix;

impl ColorMap for Matrix {
    fn map(&self, decay: u8) -> Color {
        let t = decay as f32 / 255.0;

        // Black → dark green → bright green
        let black = Color::rgb(0, 0, 0);
        let dark_green = Color::rgb(0, 100, 0);
        let bright_green = Color::rgb(0, 255, 0);

        if t < 0.5 {
            black.lerp(&dark_green, t * 2.0)
        } else {
            dark_green.lerp(&bright_green, (t - 0.5) * 2.0)
        }
    }

    fn name(&self) -> &str {
        "Matrix"
    }
}

/// Fire: Black → red → orange → yellow
pub struct Fire;

impl ColorMap for Fire {
    fn map(&self, decay: u8) -> Color {
        let t = decay as f32 / 255.0;

        let black = Color::rgb(0, 0, 0);
        let red = Color::rgb(200, 0, 0);
        let orange = Color::rgb(255, 100, 0);
        let yellow = Color::rgb(255, 255, 0);

        if t < 0.33 {
            black.lerp(&red, t * 3.0)
        } else if t < 0.67 {
            red.lerp(&orange, (t - 0.33) * 3.0)
        } else {
            orange.lerp(&yellow, (t - 0.67) * 3.0)
        }
    }

    fn name(&self) -> &str {
        "Fire"
    }
}

/// Ocean: Dark blue → cyan → white
pub struct Ocean;

impl ColorMap for Ocean {
    fn map(&self, decay: u8) -> Color {
        let t = decay as f32 / 255.0;

        let dark_blue = Color::rgb(0, 20, 40);
        let cyan = Color::rgb(0, 180, 180);
        let white = Color::rgb(200, 240, 255);

        if t < 0.5 {
            dark_blue.lerp(&cyan, t * 2.0)
        } else {
            cyan.lerp(&white, (t - 0.5) * 2.0)
        }
    }

    fn name(&self) -> &str {
        "Ocean"
    }
}

/// Cyberpunk: Pink → purple → cyan
pub struct Cyberpunk;

impl ColorMap for Cyberpunk {
    fn map(&self, decay: u8) -> Color {
        let t = decay as f32 / 255.0;

        let dark = Color::rgb(20, 0, 30);
        let pink = Color::rgb(255, 0, 150);
        let cyan = Color::rgb(0, 255, 255);

        if t < 0.5 {
            dark.lerp(&pink, t * 2.0)
        } else {
            pink.lerp(&cyan, (t - 0.5) * 2.0)
        }
    }

    fn name(&self) -> &str {
        "Cyberpunk"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokyo_night_extremes() {
        let map = TokyoNight;

        let dead = map.map(0);
        let alive = map.map(255);

        // Dead should be dark
        assert!(dead.r < 100);
        assert!(dead.g < 100);

        // Alive should be brighter
        assert!(alive.r > 100 || alive.g > 100 || alive.b > 100);
    }

    #[test]
    fn test_matrix_green() {
        let map = Matrix;

        let dead = map.map(0);
        let alive = map.map(255);

        // Dead should be black
        assert_eq!(dead, Color::BLACK);

        // Alive should be bright green
        assert_eq!(alive, Color::GREEN);
    }

    #[test]
    fn test_fire_gradient() {
        let map = Fire;

        let low = map.map(50);
        let mid = map.map(127);
        let high = map.map(200);

        // Should progress from dark → red → orange → yellow
        // Low should have more red, less green/blue
        assert!(low.r > low.g);
        assert!(low.r > low.b);

        // High should have yellow tones (high R and G)
        assert!(high.r > 150);
        assert!(high.g > 100);
    }

    #[test]
    fn test_all_maps_work() {
        let maps: Vec<Box<dyn ColorMap>> = vec![
            Box::new(TokyoNight),
            Box::new(Matrix),
            Box::new(Fire),
            Box::new(Ocean),
            Box::new(Cyberpunk),
        ];

        for map in maps {
            // Should not panic
            let _ = map.map(0);
            let _ = map.map(127);
            let _ = map.map(255);
        }
    }
}
