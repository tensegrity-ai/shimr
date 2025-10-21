mod figlet;

pub use figlet::FigletFont;

#[derive(Debug, Clone)]
pub struct CharBitmap {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<bool>,
}

impl CharBitmap {
    pub fn new(width: usize, height: usize) -> Self {
        CharBitmap {
            width,
            height,
            pixels: vec![false; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.pixels[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = value;
        }
    }
}

pub trait FontRenderer: Send + Sync {
    fn render_char(&self, ch: char) -> CharBitmap;
    fn height(&self) -> usize;
}
