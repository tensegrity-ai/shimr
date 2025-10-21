# shimr - Implementation Roadmap

## Project Status

**Current Phase:** Foundation & Core Architecture
**Last Updated:** 2025-10-21

---

## Milestones

### Phase 0: Foundation ✓
- [x] Claim crate namespace
- [x] Initial README and project vision
- [x] Basic Cargo.toml structure

### Phase 1: Core Engine (In Progress)
- [ ] Design core architecture (see `PLANS/00-architecture.md`)
- [ ] Implement cellular automata engine
  - [ ] Grid abstraction
  - [ ] Conway's Game of Life rules
  - [ ] Rule trait for extensibility
- [ ] Glyph system
  - [ ] GlyphSet enum with presets (Cyberpunk, Classic, etc.)
  - [ ] Decay sequences (█ → ▓ → ▒ → ░ → · → ` `)
- [ ] Frame generation
  - [ ] Zero-allocation render loop
  - [ ] 60fps target timing

### Phase 2: Text Morphing
- [ ] Text state representation
- [ ] Morph builder API
- [ ] Transition generation
- [ ] Iterator-based frame emission

### Phase 3: Rendering & Performance
- [ ] Backend abstraction trait
- [ ] Raw terminal backend
- [ ] Performance benchmarks
- [ ] Memory profiling

### Phase 4: Framework Integrations
- [ ] Ratatui integration
- [ ] Cursive integration
- [ ] Documentation and examples

### Phase 5: Advanced Features
- [ ] Particle system
- [ ] Color support (24-bit)
- [ ] Custom rule sets
- [ ] Audio reactive transitions (future)

---

## Current Sprint

**Focus:** Architecture design and core engine foundation

**Active Tasks:**
1. Create detailed implementation plans in `PLANS/`
2. Define core types and traits
3. Implement basic grid and automata engine
4. Create glyph system

**Blockers:** None

---

## Open Questions

1. **Grid Storage:** Use `Vec<Vec<Cell>>` or flat `Vec<Cell>` with width/height?
2. **Memory Strategy:** Pre-allocate buffers vs. dynamic allocation?
3. **Rule API:** Trait-based vs. function pointers for CA rules?
4. **Backend Trait:** Abstract over ratatui/cursive or integrate directly?

See detailed discussions in `PLANS/` directory.

---

## Dependencies to Track

- `ratatui` - TUI framework (feature-gated)
- `crossterm` or `termion` - Terminal control
- Possibly `ndarray` for matrix operations?

---

## Notes

- Prioritize zero-allocation render loop for performance
- Keep API simple and composable
- Framework-agnostic core, integrations as features

---

## Testing Approach

**Philosophy:** Minimal tests that prove functionality, not exhaustive coverage.

**Core Tests (4 total):**
1. **Visual integration test**: `test_morph_test_to_complete()` - Full pipeline test with snapshot
   - Morph "TEST" → "COMPLETE"
   - Verify frame count and progression
   - Use `insta` for visual regression testing
2. **CA correctness**: `test_conway_stable_block()` - Verify Conway's rules work
3. **Glyph mapping**: `test_glyph_set_boundaries()` - Verify decay → char mapping
4. **Performance**: `test_performance_target()` - Ensure < 16ms per frame (60fps)

**Manual Testing:**
- Examples (`cargo run --example text_morph`) for visual verification
- Interactive demos for user testing

**Dependencies:**
- `insta` - Snapshot testing for visual regression
