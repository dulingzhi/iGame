# iGame

A data-driven, cross-platform UGC game engine and editor built with [Bevy](https://bevyengine.org/) (Rust), inspired by Warcraft III World Editor.

## Features (MVP)

- **Runtime**: Loads map packages and spawns 2D entities with an RTS-style camera
- **MapPackage format v0**: `manifest.toml` + `scene.ron` for describing scenes
- **Extensible**: Workspace crate structure for runtime, shared data types, and editor

## Quick Start

### Prerequisites

- Rust (stable, 1.75+): https://rustup.rs
- On Linux: `sudo apt-get install libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev`

### Run the Demo Map

```bash
cargo run -p igame-runtime -- assets/maps/demo
```

Use **WASD** or **Arrow keys** to pan the camera, **mouse scroll** to zoom.
Press **Escape** to quit.

### Run All Tests

```bash
cargo test --workspace
```

### Using the Makefile

```bash
make run        # Run the demo
make test       # Run all tests
make check      # Check compilation
make fmt        # Check formatting
make clippy     # Run linter
make wasm-build # Check shared crate compiles for WASM
```

## Project Structure

```
iGame/
├── crates/
│   ├── shared/     # Core types: MapPackage, Manifest, Scene, validation
│   ├── runtime/    # Bevy app: loads maps, RTS camera, entity spawning
│   └── editor/     # Editor (stub – coming soon)
├── assets/
│   └── maps/
│       └── demo/   # Example map package
│           ├── manifest.toml
│           └── scene.ron
└── ROADMAP.md
```

## Map Package Format (v0)

A map package is a directory containing:

- `manifest.toml` – metadata (name, version, author, entry scene path)
- `scene.ron` – entities with Transform, Sprite, Name, and tags

**Example `manifest.toml`:**
```toml
name = "My Map"
version = "0.1.0"
author = "You"
entry_scene = "scene.ron"
```

**Example `scene.ron`:**
```ron
(
    entities: [
        (
            name: Some("Ground"),
            transform: (
                translation: (0.0, 0.0, 0.0),
                rotation: (0.0, 0.0, 0.0, 1.0),
                scale: (1.0, 1.0, 1.0),
            ),
            sprite: Some((
                color: (0.2, 0.6, 0.2, 1.0),
                custom_size: Some((800.0, 600.0)),
            )),
            tags: [],
        ),
    ],
)
```

## Development

See [ROADMAP.md](ROADMAP.md) for the full development plan and milestones.

### Definition of Done (Sprint 1 / M0+M1+M2)

- [x] Workspace compiles (`cargo check --workspace`)
- [x] All tests pass (`cargo test --workspace`)
- [x] Formatting OK (`cargo fmt --all -- --check`)
- [x] Clippy clean (`cargo clippy --workspace -- -D warnings`)
- [x] WASM build for shared crate passes
- [x] Demo map loads and renders (manual verification)
