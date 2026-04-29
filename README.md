# iGame

> **A Bevy-based UGC game engine and map editor** — think Warcraft III World Editor, but built in Rust with a data-driven ECS architecture, targeting Desktop and Web (wasm).

[![CI](https://github.com/dulingzhi/iGame/actions/workflows/ci.yml/badge.svg)](https://github.com/dulingzhi/iGame/actions/workflows/ci.yml)

---

## Project Status

| Milestone | Status |
|-----------|--------|
| M0 — Workspace skeleton & CI | ✅ Done |
| M1 — Runtime MVP (2D/3D + RTS Camera + Demo) | ✅ Done |
| M2 — MapPackage v0 (manifest.toml + scene.json) | ✅ Done |
| M3 — Editor MVP (egui Viewport/Inspector) | 🔜 Planned |
| M4 — Trigger system (visual ECA node graph) | 🔜 Planned |
| M5 — Web play & content distribution | 🔜 Planned |

See [ROADMAP.md](ROADMAP.md) for the full roadmap.

---

## Repository Layout

```
iGame/
├── Cargo.toml               # Workspace root
├── crates/
│   ├── shared/              # Data types, serialization, validation (no Bevy dep)
│   ├── runtime/             # Bevy runtime: map loading, RTS camera, demo app
│   ├── editor/              # Map editor stub (full egui editor in M3)
│   └── ugc/                 # UGC package management stub (M6)
├── examples/
│   └── demo_map/
│       ├── manifest.toml    # Map package metadata
│       └── scene.json       # Entities & components
├── docs/                    # Design documents
└── .github/workflows/ci.yml # CI: fmt / clippy / test / wasm build
```

---

## Quick Start

### Prerequisites

- **Rust** 1.75+ (2021 edition)  
- On Linux: `sudo apt-get install libudev-dev libasound2-dev libx11-dev libxkbcommon-dev libwayland-dev`

### Run the demo

```bash
# Clone
git clone https://github.com/dulingzhi/iGame
cd iGame

# Run runtime demo (opens a 3D window)
cargo run -p igame-runtime

# Run editor stub
cargo run -p igame-editor
```

**Demo Controls:**

| Input | Action |
|-------|--------|
| `W/A/S/D` or Arrow keys | Pan camera |
| `Q / E` | Rotate camera |
| Scroll wheel | Zoom in / out |
| Middle-mouse drag | Pan (mouse) |
| `Esc` | Quit |

---

## Running Tests

```bash
# All tests (unit + integration)
cargo test --workspace

# Specific crate
cargo test -p igame-shared
cargo test -p igame-runtime
cargo test -p igame-ugc

# Run integration tests only
cargo test -p igame-runtime --test integration_map_load
```

### What the tests cover

| Crate | Tests |
|-------|-------|
| `igame-shared` | Manifest parsing, scene JSON (de)serialization, trigger graph round-trip, validation |
| `igame-runtime` (unit) | Map entity spawning, RTS camera setup |
| `igame-runtime` (integration) | Load `demo_map` → run ticks → assert entities/components |
| `igame-ugc` | Package index CRUD |

---

## Map Package Format

Map packages live in a directory (or `.zip`) with this structure:

```
my_map/
├── manifest.toml    # Metadata (name, version, author, …)
├── scene.json       # Entity + component tree
└── triggers/        # (M4) ECA trigger graphs in JSON
```

**`manifest.toml` example:**

```toml
name        = "My Map"
version     = "0.1.0"
author      = "You"
description = "A demo map"
engine_min  = "0.1.0"
entry_scene = "scene.json"
```

**`scene.json` example:**

```json
{
  "entities": [
    {
      "name": "Ground",
      "transform": { "scale": [20.0, 1.0, 20.0] },
      "components": [{ "type": "mesh", "mesh_ref": "plane" }]
    }
  ]
}
```

---

## Acceptance Checklist (M0–M2)

- [x] `cargo test --workspace` — all tests pass
- [x] `cargo clippy --workspace --all-targets` — no warnings
- [x] `cargo fmt --all -- --check` — no formatting issues
- [x] `cargo run -p igame-runtime` — opens a 3D window with demo map
- [x] Demo map loads 4 entities (Ground, PlayerStart, EnemyCamp, ResourceNode)
- [x] Integration tests assert entity names, transforms, component counts

---

## Contributing

See [TESTING.md](TESTING.md) for the test strategy and how to add new tests.
See [ROADMAP.md](ROADMAP.md) for the full development plan.
