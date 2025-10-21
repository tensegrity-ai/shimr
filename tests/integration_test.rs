use shimr::prelude::*;

/// Integration test demonstrating the core flow:
/// Grid → Automata → Decay → Glyph mapping → Frame rendering
#[test]
fn test_core_pipeline() {
    // Create a grid with a blinker pattern
    let mut grid = Grid::new(5, 5);

    // Vertical blinker
    grid.set(2, 1, Cell::alive());
    grid.set(2, 2, Cell::alive());
    grid.set(2, 3, Cell::alive());

    // Use high decay rate for instant visual feedback
    let mut automata = Automata::new(grid, Box::new(Conway))
        .with_decay_rate(255); // Instant decay

    let glyph_set = GlyphSet::classic();

    // Simulate one generation
    automata.step();

    // Should now be horizontal blinker
    assert!(automata.grid().get(1, 2).is_alive());
    assert!(automata.grid().get(2, 2).is_alive());
    assert!(automata.grid().get(3, 2).is_alive());

    // Render to frame
    let mut frame = Frame::new(automata.grid().width(), automata.grid().height());

    for (x, y, cell) in automata.grid().cells() {
        let glyph = glyph_set.map(cell.decay());
        frame.set(x, y, glyph);
    }

    // Alive cells should have non-space glyphs (decay is increasing)
    let glyph_1_2 = frame.get(1, 2);
    let glyph_2_2 = frame.get(2, 2);
    let glyph_3_2 = frame.get(3, 2);

    assert_ne!(glyph_1_2, ' ', "alive cells should have visible glyphs");
    assert_ne!(glyph_2_2, ' ');
    assert_ne!(glyph_3_2, ' ');

    // Dead cells should be spaces
    assert_eq!(frame.get(0, 0), ' ');

    // Verify frame can be rendered
    let rendered = frame.render();
    assert!(!rendered.is_empty());
}

/// Test that decay creates smooth visual transitions
#[test]
fn test_decay_transitions() {
    // Use stable 2x2 block so cells stay alive
    let mut grid = Grid::new(4, 4);
    grid.set(1, 1, Cell::with_decay(true, 0));
    grid.set(2, 1, Cell::with_decay(true, 0));
    grid.set(1, 2, Cell::with_decay(true, 0));
    grid.set(2, 2, Cell::with_decay(true, 0));

    let mut automata = Automata::new(grid, Box::new(Conway))
        .with_decay_rate(32);

    let glyph_set = GlyphSet::classic();

    // Collect glyphs as cell decays toward alive state (255)
    let mut glyphs = Vec::new();

    for _ in 0..10 {
        automata.step();
        let cell = automata.grid().get(1, 1);
        let glyph = glyph_set.map(cell.decay());
        glyphs.push(glyph);
    }

    // Should end with solid block (fully alive) or close to it
    let last_glyph = glyphs.last().unwrap();
    assert!(*last_glyph == '█' || *last_glyph == '▓', "Should reach high-decay glyph");

    // Should have different glyphs during transition
    let unique_glyphs: std::collections::HashSet<_> = glyphs.iter().collect();
    assert!(unique_glyphs.len() > 1, "Should have multiple glyphs during transition");
}

/// Test creating multiple frames from a CA simulation
#[test]
fn test_multi_generation_rendering() {
    let mut grid = Grid::new(5, 5);

    // Create a glider (moves diagonally)
    grid.set(1, 0, Cell::alive());
    grid.set(2, 1, Cell::alive());
    grid.set(0, 2, Cell::alive());
    grid.set(1, 2, Cell::alive());
    grid.set(2, 2, Cell::alive());

    let mut automata = Automata::new(grid, Box::new(Conway))
        .with_decay_rate(64);

    let glyph_set = GlyphSet::ascii(); // Use ASCII for easier testing

    let mut frames = Vec::new();

    // Run 3 generations
    for _gen in 0..3 {
        // Render current state
        let mut frame = Frame::new(automata.grid().width(), automata.grid().height());
        for (x, y, cell) in automata.grid().cells() {
            let glyph = glyph_set.map(cell.decay());
            frame.set(x, y, glyph);
        }
        frames.push(frame);

        // Advance simulation
        automata.step();
    }

    // Should have 3 frames
    assert_eq!(frames.len(), 3);

    // Each frame should be renderable
    for frame in &frames {
        let rendered = frame.render();
        assert_eq!(rendered.lines().count(), automata.grid().height());
    }

    // Frames should be different (glider is moving)
    assert_ne!(frames[0], frames[1]);
    assert_ne!(frames[1], frames[2]);
}
