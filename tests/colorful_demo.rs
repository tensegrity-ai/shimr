use shimr::prelude::*;

/// Colorful TEST → COMPLETE with Tokyo Night colors
#[test]
#[ignore] // Run with: cargo test --test colorful_demo -- --ignored --nocapture
fn visual_demo_tokyo_night() {
    println!("\n=== TEST → COMPLETE (Tokyo Night) ===\n");

    let mut transition = shimr::morph("TEST", "COMPLETE")
        .generations(12)
        .glyph_set(GlyphSet::classic())
        .color_map(TokyoNight)
        .decay_rate(64)
        .build();

    for i in 0..12 {
        println!("Frame {}:", i);
        println!("{}\n", transition.render_colored());
        transition.next();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

/// Fiery TEST → COMPLETE
#[test]
#[ignore]
fn visual_demo_fire() {
    println!("\n=== TEST → COMPLETE (Fire) ===\n");

    let mut transition = shimr::morph("TEST", "COMPLETE")
        .generations(10)
        .glyph_set(GlyphSet::classic())
        .color_map(Fire)
        .decay_rate(48)
        .build();

    for i in 0..10 {
        println!("Frame {}:", i);
        println!("{}\n", transition.render_colored());
        transition.next();
        std::thread::sleep(std::time::Duration::from_millis(120));
    }
}

/// Matrix-style morphing
#[test]
#[ignore]
fn visual_demo_matrix() {
    println!("\n=== HI → BYE (Matrix) ===\n");

    let mut transition = shimr::morph("HI", "BYE")
        .generations(8)
        .glyph_set(GlyphSet::matrix())
        .color_map(Matrix)
        .rule(Seeds)  // Seeds rule for flowing trails
        .decay_rate(96)
        .build();

    for i in 0..8 {
        println!("Frame {}:", i);
        println!("{}\n", transition.render_colored());
        transition.next();
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
}

/// Cyberpunk with HighLife rule
#[test]
#[ignore]
fn visual_demo_cyberpunk() {
    println!("\n=== GO → OK (Cyberpunk + HighLife) ===\n");

    let mut transition = shimr::morph("GO", "OK")
        .generations(10)
        .glyph_set(GlyphSet::cyberpunk())
        .color_map(Cyberpunk)
        .rule(HighLife)  // More chaotic than Conway
        .decay_rate(64)
        .build();

    for i in 0..10 {
        println!("Frame {}:", i);
        println!("{}\n", transition.render_colored());
        transition.next();
        std::thread::sleep(std::time::Duration::from_millis(120));
    }
}

/// Ocean colors with Day & Night rule
#[test]
#[ignore]
fn visual_demo_ocean() {
    println!("\n=== A → B (Ocean + Day&Night) ===\n");

    let mut transition = shimr::morph("A", "B")
        .generations(8)
        .glyph_set(GlyphSet::dots())
        .color_map(Ocean)
        .rule(DayAndNight)  // Symmetric rule
        .decay_rate(80)
        .build();

    for i in 0..8 {
        println!("Frame {}:", i);
        println!("{}\n", transition.render_colored());
        transition.next();
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
}

/// Test that color rendering works (non-visual)
#[test]
fn test_color_output() {
    let mut transition = shimr::morph("HI", "OK")
        .generations(3)
        .color_map(TokyoNight)
        .build();

    // Render colored
    let colored = transition.render_colored();

    // Should contain ANSI escape codes
    assert!(colored.contains("\x1b["), "Should have ANSI color codes");
    assert!(colored.contains("m"), "Should have ANSI color format");
}

/// Test different color maps
#[test]
fn test_all_color_maps() {
    // Test Tokyo Night
    {
        let mut transition = shimr::morph("A", "B")
            .generations(2)
            .color_map(TokyoNight)
            .build();
        let output = transition.render_colored();
        assert!(output.contains("\x1b["), "TokyoNight should produce colored output");
    }

    // Test Matrix
    {
        let mut transition = shimr::morph("A", "B")
            .generations(2)
            .color_map(Matrix)
            .build();
        let output = transition.render_colored();
        assert!(output.contains("\x1b["), "Matrix should produce colored output");
    }

    // Test Fire
    {
        let mut transition = shimr::morph("A", "B")
            .generations(2)
            .color_map(Fire)
            .build();
        let output = transition.render_colored();
        assert!(output.contains("\x1b["), "Fire should produce colored output");
    }
}

/// Test different rules
#[test]
fn test_all_rules() {
    // Test Conway
    {
        let frames: Vec<_> = shimr::morph("A", "B")
            .generations(3)
            .rule(Conway)
            .build()
            .collect();
        assert_eq!(frames.len(), 3);
    }

    // Test HighLife
    {
        let frames: Vec<_> = shimr::morph("A", "B")
            .generations(3)
            .rule(HighLife)
            .build()
            .collect();
        assert_eq!(frames.len(), 3);
    }

    // Test Seeds
    {
        let frames: Vec<_> = shimr::morph("A", "B")
            .generations(3)
            .rule(Seeds)
            .build()
            .collect();
        assert_eq!(frames.len(), 3);
    }
}
