use shimr::prelude::*;

/// The main event: TEST → COMPLETE morph
///
/// This test demonstrates the full text morphing pipeline
/// and serves as both a functional test and a visual demo.
#[test]
fn test_morph_test_to_complete() {
    let frames: Vec<Frame> = shimr::morph("TEST", "COMPLETE")
        .generations(15)
        .glyph_set(GlyphSet::classic())
        .decay_rate(48)
        .build()
        .collect();

    // Should generate 15 frames
    assert_eq!(frames.len(), 15);

    // Each frame should be renderable
    for (i, frame) in frames.iter().enumerate() {
        let rendered = frame.render();

        assert!(!rendered.is_empty(), "Frame {} should not be empty", i);

        // Each frame should have multiple lines (height of 5)
        let line_count = rendered.lines().count();
        assert_eq!(line_count, 5, "Frame {} should have 5 lines", i);
    }

    // First frame should have visible content
    let first_rendered = frames[0].render();
    assert!(first_rendered.contains('█') || first_rendered.contains('#'),
            "First frame should have visible glyphs");

    // Last frame should have visible content
    let last_rendered = frames[frames.len() - 1].render();
    assert!(last_rendered.contains('█') || last_rendered.contains('#'),
            "Last frame should have visible glyphs");

    // Frames should change over time (not all identical)
    let unique_frames: std::collections::HashSet<_> = frames.iter().map(|f| f.render()).collect();
    assert!(unique_frames.len() > 1, "Frames should evolve over time");
}

/// Test with different glyph sets
#[test]
fn test_morph_with_cyberpunk() {
    let frames: Vec<Frame> = shimr::morph("HI", "BYE")
        .generations(10)
        .glyph_set(GlyphSet::cyberpunk())
        .build()
        .collect();

    assert_eq!(frames.len(), 10);

    // Should have cyberpunk glyphs
    let rendered = frames.iter()
        .map(|f| f.render())
        .collect::<String>();

    // Cyberpunk set includes these
    let has_cyberpunk_glyphs = rendered.contains('▰') ||
                                rendered.contains('▓') ||
                                rendered.contains('▒') ||
                                rendered.contains('░') ||
                                rendered.contains('▱');

    assert!(has_cyberpunk_glyphs, "Should use cyberpunk glyphs");
}

/// Test with ASCII-safe glyph set
#[test]
fn test_morph_with_ascii() {
    let frames: Vec<Frame> = shimr::morph("GO", "OK")
        .generations(5)
        .glyph_set(GlyphSet::ascii())
        .build()
        .collect();

    assert_eq!(frames.len(), 5);

    // ASCII set should only use #, +, ., and space
    let rendered = frames.iter()
        .map(|f| f.render())
        .collect::<String>();

    for ch in rendered.chars() {
        assert!(ch == '#' || ch == '+' || ch == '.' || ch == ' ' || ch == '\n',
                "ASCII mode should only use ASCII-safe chars, found '{}'", ch);
    }
}

/// Quick smoke test
#[test]
fn test_quick_morph() {
    let frames: Vec<Frame> = shimr::morph("A", "B")
        .generations(3)
        .build()
        .collect();

    assert_eq!(frames.len(), 3);
}

/// Print a visual demo (only runs with --nocapture)
#[test]
#[ignore] // Run with: cargo test --test test_to_complete -- --ignored --nocapture
fn visual_demo_test_to_complete() {
    println!("\n=== TEST → COMPLETE Morph ===\n");

    for (i, frame) in shimr::morph("TEST", "COMPLETE")
        .generations(10)
        .glyph_set(GlyphSet::classic())
        .decay_rate(64)
        .build()
        .enumerate()
    {
        println!("Frame {}:\n{}\n", i, frame.render());
    }
}
