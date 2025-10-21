# Backends and Framework Integrations

## Overview

shimr core is framework-agnostic. This plan covers rendering backends and integrations with popular TUI frameworks (ratatui, cursive).

## Architecture

```
shimr::core (Grid, Automata, Frame)
        ↓
shimr::backend::Backend trait
        ↓
    ┌───┴───┬─────────┬─────────┐
    │       │         │         │
  Raw   Ratatui   Cursive   Custom
```

## Backend Trait

### Interface (`backend/trait.rs`)

```rust
use crate::transition::Frame;

/// Rendering backend
pub trait Backend {
    /// Error type for this backend
    type Error;

    /// Render a frame to the output
    fn render(&mut self, frame: &Frame) -> Result<(), Self::Error>;

    /// Clear the output
    fn clear(&mut self) -> Result<(), Self::Error>;

    /// Flush any buffered output
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Get terminal dimensions (if available)
    fn size(&self) -> Option<(usize, usize)> {
        None
    }
}

/// Helper for running animations on a backend
pub trait BackendExt: Backend {
    /// Run a transition animation
    fn run_transition<I>(&mut self, transition: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = Frame>,
    {
        for frame in transition {
            self.clear()?;
            self.render(&frame)?;
            self.flush()?;

            // TODO: frame timing?
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
        Ok(())
    }
}

impl<T: Backend> BackendExt for T {}
```

## Raw Terminal Backend

For direct terminal output without frameworks.

### Implementation (`backend/raw.rs`)

```rust
use std::io::{self, Write, Stdout};
use crossterm::{
    terminal::{self, ClearType},
    cursor,
    ExecutableCommand,
};
use super::{Backend, Frame};

/// Raw terminal backend using crossterm
pub struct RawTerminal {
    stdout: Stdout,
    width: usize,
    height: usize,
}

impl RawTerminal {
    pub fn new() -> io::Result<Self> {
        let (width, height) = terminal::size()?;

        Ok(RawTerminal {
            stdout: io::stdout(),
            width: width as usize,
            height: height as usize,
        })
    }

    /// Enter raw mode
    pub fn enter(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        self.stdout.execute(cursor::Hide)?;
        Ok(())
    }

    /// Exit raw mode
    pub fn exit(&mut self) -> io::Result<()> {
        self.stdout.execute(cursor::Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

impl Backend for RawTerminal {
    type Error = io::Error;

    fn render(&mut self, frame: &Frame) -> Result<(), Self::Error> {
        // Move cursor to top-left
        self.stdout.execute(cursor::MoveTo(0, 0))?;

        // Write frame content
        write!(self.stdout, "{}", frame.render())?;

        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        self.stdout.execute(terminal::Clear(ClearType::All))?;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.stdout.flush()
    }

    fn size(&self) -> Option<(usize, usize)> {
        Some((self.width, self.height))
    }
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        let _ = self.exit();
    }
}
```

### Usage Example

```rust
use shimr::prelude::*;
use shimr::backend::{RawTerminal, BackendExt};

fn main() -> io::Result<()> {
    let mut terminal = RawTerminal::new()?;
    terminal.enter()?;

    let animation = shimr::morph("hello", "world")
        .generations(30)
        .build();

    terminal.run_transition(animation)?;
    terminal.exit()?;

    Ok(())
}
```

## Ratatui Integration

Feature-gated integration with ratatui.

### Implementation (`integrations/ratatui.rs`)

```rust
#[cfg(feature = "ratatui")]
use ratatui::{
    backend::Backend as RatatuiBackend,
    Terminal,
    widgets::{Widget, Block},
    layout::Rect,
    buffer::Buffer,
    text::{Line, Span},
    style::Style,
};
use crate::transition::Frame;

/// Widget wrapper for shimr frames
#[cfg(feature = "ratatui")]
pub struct ShimrWidget<'a> {
    frame: &'a Frame,
    style: Style,
}

#[cfg(feature = "ratatui")]
impl<'a> ShimrWidget<'a> {
    pub fn new(frame: &'a Frame) -> Self {
        ShimrWidget {
            frame,
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

#[cfg(feature = "ratatui")]
impl<'a> Widget for ShimrWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render frame into ratatui buffer
        for y in 0..self.frame.height.min(area.height as usize) {
            let mut spans = Vec::new();

            for x in 0..self.frame.width.min(area.width as usize) {
                let glyph = self.frame.get(x, y);
                spans.push(Span::styled(glyph.to_string(), self.style));
            }

            let line = Line::from(spans);
            buf.set_line(area.x, area.y + y as u16, &line, area.width);
        }
    }
}

/// Helper for running transitions in ratatui
#[cfg(feature = "ratatui")]
pub fn run_transition<B: RatatuiBackend>(
    terminal: &mut Terminal<B>,
    transition: impl Iterator<Item = Frame>,
) -> io::Result<()> {
    for frame in transition {
        terminal.draw(|f| {
            let size = f.area();
            f.render_widget(ShimrWidget::new(&frame), size);
        })?;

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}
```

### Usage Example

```rust
use shimr::prelude::*;
use shimr::integrations::ratatui::{ShimrWidget, run_transition};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let animation = shimr::morph("hello", "world")
        .generations(30)
        .glyph_set(GlyphSet::cyberpunk())
        .build();

    run_transition(&mut terminal, animation)?;

    Ok(())
}
```

## Cursive Integration

Feature-gated integration with cursive.

### Implementation (`integrations/cursive.rs`)

```rust
#[cfg(feature = "cursive")]
use cursive::{
    View,
    Printer,
    Vec2,
    event::{Event, EventResult},
};
use crate::transition::Frame;

/// Cursive view for shimr animations
#[cfg(feature = "cursive")]
pub struct ShimrView {
    frame: Frame,
    transition: Option<Box<dyn Iterator<Item = Frame>>>,
}

#[cfg(feature = "cursive")]
impl ShimrView {
    pub fn new(frame: Frame) -> Self {
        ShimrView {
            frame,
            transition: None,
        }
    }

    pub fn with_transition(transition: impl Iterator<Item = Frame> + 'static) -> Self {
        let mut iter = Box::new(transition);
        let first_frame = iter.next().unwrap_or_else(|| Frame::new(1, 1));

        ShimrView {
            frame: first_frame,
            transition: Some(iter),
        }
    }

    pub fn update(&mut self) -> bool {
        if let Some(ref mut trans) = self.transition {
            if let Some(next_frame) = trans.next() {
                self.frame = next_frame;
                return true;
            }
        }
        false
    }
}

#[cfg(feature = "cursive")]
impl View for ShimrView {
    fn draw(&self, printer: &Printer) {
        for y in 0..self.frame.height {
            for x in 0..self.frame.width {
                let glyph = self.frame.get(x, y);
                printer.print((x, y), &glyph.to_string());
            }
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        Vec2::new(self.frame.width, self.frame.height)
    }

    fn on_event(&mut self, _event: Event) -> EventResult {
        EventResult::Ignored
    }
}
```

### Usage Example

```rust
use shimr::prelude::*;
use shimr::integrations::cursive::ShimrView;
use cursive::Cursive;

fn main() {
    let mut siv = Cursive::default();

    let animation = shimr::morph("hello", "world")
        .generations(30)
        .build();

    let view = ShimrView::with_transition(animation);
    siv.add_fullscreen_layer(view);

    // Update animation on timer
    siv.set_fps(60);

    siv.run();
}
```

## Feature Flags

### Cargo.toml

```toml
[features]
default = ["raw"]

# Raw terminal backend
raw = ["dep:crossterm"]

# Framework integrations
ratatui = ["dep:ratatui"]
cursive = ["dep:cursive"]

# Advanced features (future)
particles = []
color = []

[dependencies]
# Core dependencies (always included)
# ... (none yet)

# Optional dependencies
crossterm = { version = "0.27", optional = true }
ratatui = { version = "0.26", optional = true }
cursive = { version = "0.20", optional = true }
```

## Examples

### Raw Terminal Example (`examples/text_morph.rs`)

```rust
use shimr::prelude::*;
use shimr::backend::{RawTerminal, BackendExt};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = RawTerminal::new()?;
    terminal.enter()?;

    let animation = shimr::morph("hello world", "goodbye moon")
        .generations(30)
        .glyph_set(GlyphSet::cyberpunk())
        .build();

    terminal.run_transition(animation)?;

    terminal.exit()?;
    Ok(())
}
```

### Ratatui Example (`examples/ratatui_demo.rs`)

```rust
#[cfg(feature = "ratatui")]
use shimr::prelude::*;
use shimr::integrations::ratatui::run_transition;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

#[cfg(feature = "ratatui")]
fn main() -> io::Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let animation = shimr::morph("layer 0: input", "layer 1: attention")
        .generations(40)
        .glyph_set(GlyphSet::cyberpunk())
        .build();

    run_transition(&mut terminal, animation)?;

    Ok(())
}

#[cfg(not(feature = "ratatui"))]
fn main() {
    println!("This example requires the 'ratatui' feature");
}
```

## Testing Strategy

### Backend Tests

```rust
#[test]
fn test_raw_terminal_creation() {
    let result = RawTerminal::new();
    assert!(result.is_ok());
}

#[test]
fn test_backend_trait() {
    struct MockBackend;

    impl Backend for MockBackend {
        type Error = ();

        fn render(&mut self, _frame: &Frame) -> Result<(), ()> {
            Ok(())
        }

        fn clear(&mut self) -> Result<(), ()> {
            Ok(())
        }
    }

    let mut backend = MockBackend;
    let frame = Frame::new(10, 10);

    assert!(backend.render(&frame).is_ok());
}
```

### Integration Tests

Run examples as integration tests:

```rust
#[test]
#[ignore]  // Requires interactive terminal
fn test_text_morph_example() {
    // Ensure example compiles and runs without panicking
}
```

## Performance Considerations

1. **Frame rendering:** Most backends do buffered I/O
2. **Terminal updates:** Minimize flicker with clear → render → flush
3. **Timing:** Target 60fps = 16.67ms budget
   - CA step: ~5ms
   - Render: ~8ms
   - Terminal I/O: ~3ms

## Implementation Order

1. **Backend trait** - interface definition
2. **RawTerminal** - basic backend with crossterm
3. **Feature flags** - set up in Cargo.toml
4. **Ratatui integration** - behind feature flag
5. **Examples** - text_morph, ratatui_demo
6. **Cursive integration** - if time permits

## Open Questions

1. **Frame timing?** Should backends handle timing or user?
   - Decision: User controls timing (via sleep in iterator)

2. **Color support?** Add to Backend trait?
   - Decision: Phase 5 feature, add style/color params later

3. **Input handling?** Should backends support events?
   - Decision: No, keep backends simple. Use framework's event system.

## Next Steps

1. Define Backend trait
2. Implement RawTerminal backend
3. Create text_morph example
4. Add ratatui integration
5. Test on real terminals
