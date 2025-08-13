# shimr

> *terminal transition effects using cellular automata*

[![crates.io](https://img.shields.io/crates/v/shimr.svg)](https://crates.io/crates/shimr)
[![docs.rs](https://docs.rs/shimr/badge.svg)](https://docs.rs/shimr)

shimr creates organic transition effects for terminal UIs. watch your text dissolve into conway patterns, then crystallize into new forms. perfect for cyberpunk aesthetics and smooth state changes.

## features

- **cellular automata transitions**: conway's game of life drives state morphing
- **glyph gradients**: smooth decay sequences (`█ → ▓ → ▒ → ░ → · → `)
- **text morphing**: dissolve and reform text through simulation
- **performance focused**: 60fps animations with zero allocations in render loop
- **framework agnostic**: works with ratatui, cursive, or raw terminal control

## quick start

```rust
use shimr::prelude::*;

// morph between two text states
let start = "layer 0: attention";
let end = "layer 1: transformation";

let animation = shimr::morph(start, end)
    .generations(15)
    .glyph_set(GlyphSet::Cyberpunk)
    .build();

// render each frame at 60fps
for frame in animation {
    terminal.draw(frame)?;
}
```

## visualization modes

### text transitions
```
hello world     →  [conway simulation]  →  goodbye moon
█████ █████        ░▒▓█░ ▒░▓██           ███████ ████
```

### matrix morphing
transform data grids through organic patterns:
```rust
let attention_matrix = arr2(&[[0.8, 0.2], [0.3, 0.9]]);
shimr::morph_grid(attention_matrix, target_matrix)
    .rule(Rule::Conway)
    .colormap(ColorMap::TokyoNight)
    .build()
```

### particle effects
cells emit sparks during death, pull energy during birth:
```rust
shimr::particles()
    .on_death('✦')
    .on_birth('·')
    .critical_mass('⚡')
```

## installation

```toml
[dependencies]
shimr = "0.1"
```

### feature flags

- `ratatui` - integration with ratatui (enabled by default)
- `cursive` - cursive framework support
- `particles` - particle system effects
- `color` - 24-bit color support

## examples

```bash
# basic text morphing
cargo run --example text_morph

# attention visualization (like in aiayn)
cargo run --example attention

# particle system demo
cargo run --example particles
```

## philosophy

shimr emerged from building [aiayn](https://github.com/yourusername/aiayn), a terminal attention visualizer. we needed transitions that felt organic, like thought patterns evolving through neural networks.

every frame should feel alive. no harsh cuts or boring fades. just smooth, cellular evolution from one state to another.

## roadmap

- [ ] custom rule sets beyond conway
- [ ] midi/audio reactive transitions
- [ ] wasm support for browser terminals
- [ ] gradient maps from any color palette
- [ ] transition choreography dsl

## contributing

we welcome contributions! please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

ideas especially welcome for:
- new cellular automata rules
- innovative glyph sequences
- performance optimizations
- creative use cases

## license

dual-licensed under MIT and Apache 2.0. choose whichever you prefer.

## acknowledgments

- inspired by conway's game of life
- tokyo night color palette by enkia
- the rust community for excellent terminal crates

---

*shimr: because state changes should shimmer*