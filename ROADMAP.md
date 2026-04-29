# iGame Roadmap

## Vision

iGame is a data-driven, cross-platform UGC game engine and editor built with Bevy (Rust), inspired by Warcraft III World Editor. The goal is to allow non-programmers to create, share, and play custom maps with a simple, extensible format.

---

## Milestone Overview

| ID  | Name                        | Status      |
|-----|-----------------------------|-------------|
| M0  | Project Scaffold            | ✅ Done      |
| M1  | MapPackage Format v0        | ✅ Done      |
| M2  | Runtime MVP                 | ✅ Done      |
| M3  | Editor Stub → Basic UI      | 🔲 Planned  |
| M4  | Tilemap Support             | 🔲 Planned  |
| M5  | Scripting (Lua)             | 🔲 Planned  |
| M6  | Asset Pipeline              | 🔲 Planned  |
| M7  | Multiplayer Foundation      | 🔲 Planned  |
| M8  | Map Marketplace / Sharing   | 🔲 Planned  |
| M9  | Mobile / WASM Export        | 🔲 Planned  |
| M10 | 1.0 Release                 | 🔲 Planned  |

---

## M0 — Project Scaffold ✅

**Goal**: Rust workspace, CI pipeline, crate boundaries.

### Checklist
- [x] Cargo workspace with `shared`, `runtime`, `editor` crates
- [x] `.gitignore`, `Makefile`, `README.md`
- [x] GitHub Actions CI: fmt, clippy, test, WASM build check
- [x] Rust 2021 edition throughout
- [x] MIT OR Apache-2.0 license headers

### Definition of Done
- `cargo check --workspace` passes
- `cargo fmt --all -- --check` passes
- `cargo clippy --workspace -- -D warnings` passes

---

## M1 — MapPackage Format v0 ✅

**Goal**: Define and implement the core data types for map packages.

### Checklist
- [x] `Manifest` struct: name, version, author, description, entry_scene
- [x] `MapScene` + `EntityData`: transform, sprite, tags
- [x] `MapPackage::load(path)` reads manifest.toml + scene.ron
- [x] `validate()` function with error types
- [x] Full unit test coverage for all types
- [x] Golden test for manifest serialization
- [x] Fixture-based integration tests
- [x] `igame-shared` crate builds for `wasm32-unknown-unknown`

### Definition of Done
- `cargo test -p igame-shared` passes (all tests green)
- WASM build check passes

---

## M2 — Runtime MVP ✅

**Goal**: Bevy app that loads a map package and renders it.

### Checklist
- [x] `igame-runtime` binary: `igame [path/to/map]`
- [x] RTS-style camera: WASD pan, scroll-wheel zoom
- [x] `SceneSpawnerPlugin`: spawns entities from `MapPackage`
- [x] `AppState` enum: Loading → Playing
- [x] Headless integration tests with `MinimalPlugins`
- [x] Demo map (`assets/maps/demo/`) with 4 entities

### Definition of Done
- `cargo test --workspace` passes
- Demo map renders (manual verification)
- No window required for tests

---

## M3 — Editor Basic UI 🔲

**Goal**: Egui-based editor window with map preview and entity inspector.

### Planned Features
- [ ] Egui integration via `bevy_egui`
- [ ] Map file picker / open dialog
- [ ] Scene viewport with entity selection
- [ ] Entity inspector panel (name, transform, sprite, tags)
- [ ] Save map package to disk
- [ ] Undo/redo stack (basic)

---

## M4 — Tilemap Support 🔲

**Goal**: Replace flat sprite ground with a proper tilemap system.

### Planned Features
- [ ] Tileset definition format (PNG + tile metadata)
- [ ] Tilemap layer in scene format
- [ ] `bevy_ecs_tilemap` integration or custom tilemap renderer
- [ ] Editor tilemap painting tool

---

## M5 — Scripting (Lua) 🔲

**Goal**: Allow map creators to add game logic without writing Rust.

### Planned Features
- [ ] Lua runtime via `mlua` crate
- [ ] Script component in scene format
- [ ] Event hooks: on_spawn, on_tick, on_interact
- [ ] Sandboxed API: move, spawn, despawn, read/write tags
- [ ] Hot-reload scripts in editor

---

## M6 — Asset Pipeline 🔲

**Goal**: Proper asset management for textures, sounds, and fonts.

### Planned Features
- [ ] Asset manifest in map package
- [ ] Bevy `AssetServer` integration for textures
- [ ] Audio playback (background music + SFX)
- [ ] Font rendering for UI labels
- [ ] Asset compression / packing for distribution

---

## M7 — Multiplayer Foundation 🔲

**Goal**: Basic peer-to-peer or server-authoritative multiplayer.

### Planned Features
- [ ] Network transport selection (WebRTC / TCP)
- [ ] Entity replication via `bevy_replicon` or similar
- [ ] Deterministic simulation for RTS logic
- [ ] Lobby system

---

## M8 — Map Marketplace / Sharing 🔲

**Goal**: In-app discovery and download of community maps.

### Planned Features
- [ ] Map package format v1 (versioned, signed)
- [ ] HTTP API for map index
- [ ] In-editor upload/download UI
- [ ] Rating and comments

---

## M9 — Mobile / WASM Export 🔲

**Goal**: Run maps in the browser and on mobile devices.

### Planned Features
- [ ] WASM target for runtime (bevy + wasm-bindgen)
- [ ] Touch controls for camera
- [ ] Responsive canvas sizing
- [ ] Android / iOS build scripts

---

## M10 — 1.0 Release 🔲

**Goal**: Stable, documented, production-ready release.

### Planned Features
- [ ] Complete API documentation
- [ ] Migration guides for format changes
- [ ] Performance benchmarks
- [ ] Security audit of scripting sandbox
- [ ] Distribution packages (Windows, macOS, Linux, Web)
