use shimr::prelude::*;

#[test]
fn test_numbers() {
    let transition = shimr::morph("123", "456")
        .generations(10)
        .build();

    let frames: Vec<_> = transition.collect();
    assert_eq!(frames.len(), 10);

    for frame in &frames {
        assert!(!frame.render().is_empty());
    }
}

#[test]
fn test_punctuation() {
    let transition = shimr::morph("Hello!", "Goodbye?")
        .generations(15)
        .build();

    let frames: Vec<_> = transition.collect();
    assert_eq!(frames.len(), 15);
}

#[test]
fn test_mixed_case() {
    let transition = shimr::morph("AbC", "xYz")
        .generations(12)
        .build();

    let frames: Vec<_> = transition.collect();
    assert_eq!(frames.len(), 12);
}

#[test]
fn test_special_chars() {
    let transition = shimr::morph("(A+B)", "[X-Y]")
        .generations(10)
        .build();

    let frames: Vec<_> = transition.collect();
    assert_eq!(frames.len(), 10);
}

#[test]
fn test_comprehensive() {
    let transition = shimr::morph("Hello, World!", "Goodbye 2024!")
        .generations(20)
        .glyph_set(GlyphSet::cyberpunk())
        .build();

    let frames: Vec<_> = transition.collect();
    assert_eq!(frames.len(), 20);

    for (i, frame) in frames.iter().enumerate() {
        let rendered = frame.render();
        assert!(!rendered.is_empty(), "Frame {} should not be empty", i);
    }
}

#[test]
#[ignore]
fn visual_demo_comprehensive() {
    println!("\n=== Morphing: 'Code 2024' -> 'Rust 2025!' ===\n");

    let transition = shimr::morph("Code 2024", "Rust 2025!")
        .generations(25)
        .glyph_set(GlyphSet::classic())
        .build();

    for frame in transition {
        println!("{}", frame.render());
        println!();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[test]
#[ignore]
fn visual_demo_colorful() {
    println!("\n=== Colorful Morphing: 'A@B#C' -> '1&2%3' ===\n");

    let mut transition = shimr::morph("A@B#C", "1&2%3")
        .generations(20)
        .glyph_set(GlyphSet::cyberpunk())
        .color_map(Cyberpunk)
        .build();

    for i in 0..20 {
        println!("Frame {}:", i);
        println!("{}", transition.render_colored());
        println!();
        transition.next();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
