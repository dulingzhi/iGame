# TESTING — iGame Test Strategy

This document describes how tests are organised, how to run them, and the
conventions to follow when adding new tests.

---

## Test Pyramid

```
           ┌──────────────────────────────────────────┐
           │   Integration tests  (crates/runtime/tests/) │  ← load real files, run Bevy app
           ├──────────────────────────────────────────┤
           │     Unit tests  (each crate src/**/*.rs)  │  ← logic, (de)serialisation
           └──────────────────────────────────────────┘
```

### Unit tests

Located inside the crate's source files (`#[cfg(test)]` modules).

- **`igame-shared`** — all types under `crates/shared/src/`:
  - `map_package` — manifest TOML parsing, `MapPackage::from_strings`, error paths
  - `scene` — `SceneData` JSON round-trip, transform defaults, component variants
  - `trigger` — `TriggerGraph` JSON round-trip, event/condition/action parsing
  - `validation` — `Validator` manifest + scene checks, semver detection
- **`igame-runtime`** — inside `crates/runtime/src/`:
  - `map_loader` — entities spawned, names match, transforms match
  - `camera` — camera entity present, settings sane
- **`igame-ugc`** — inside `crates/ugc/src/index.rs` — CRUD on `PackageIndex`

### Integration tests

Located in `crates/runtime/tests/integration_map_load.rs`.

They load the **real** demo map (`examples/demo_map/`) via `include_str!`, create
a headless Bevy `App` with `MinimalPlugins`, run several ticks, and assert:

| Test | What it checks |
|------|----------------|
| `demo_map_manifest_name` | Manifest name == "Demo Map" |
| `demo_map_manifest_version` | Version == "0.1.0" |
| `demo_map_manifest_author` | Author field non-empty |
| `demo_map_has_entities` | At least one entity spawned |
| `demo_map_entity_count_matches_scene` | Spawned count == `scene.entities` length |
| `all_spawned_entities_have_name` | Every entity has `Name` component |
| `all_spawned_entities_have_transform` | Every entity has `Transform` component |
| `demo_map_contains_ground_entity` | "Ground" entity exists |
| `demo_map_contains_player_start` | "PlayerStart" entity exists |
| `multiple_ticks_do_not_duplicate_entities` | Entity count stable after 5 ticks |

---

## Running Tests

```bash
# Everything
cargo test --workspace

# One crate
cargo test -p igame-shared
cargo test -p igame-runtime
cargo test -p igame-ugc

# Integration tests only
cargo test -p igame-runtime --test integration_map_load

# A single test by name
cargo test -p igame-runtime demo_map_manifest_name

# With output (helpful for debugging)
cargo test --workspace -- --nocapture
```

---

## CI Gates

The CI workflow (`.github/workflows/ci.yml`) must pass on every PR:

| Job | Command |
|-----|---------|
| `fmt` | `cargo fmt --all -- --check` |
| `clippy` | `cargo clippy --workspace --all-targets -- -D warnings` |
| `test` | `cargo test --workspace` |
| `wasm-build` | `cargo build -p igame-shared --target wasm32-unknown-unknown` |

---

## Adding New Tests

### Adding a unit test (shared types)

1. Open the relevant `src/*.rs` file in `crates/shared/`.
2. Add a test to the existing `#[cfg(test)]` block at the bottom.
3. Keep tests self-contained — use inline strings, not filesystem reads.

### Adding a map-loader test

- For **entity spawning logic**: add to the `#[cfg(test)] mod tests` inside
  `crates/runtime/src/map_loader.rs` (uses the hardcoded inline `MANIFEST` / `SCENE`).
- For **demo-map file assertions**: add a `#[test]` function to
  `crates/runtime/tests/integration_map_load.rs`.

### Golden / snapshot tests (planned)

Future tests will capture snapshots of serialised scene data and compare them to
committed baselines (`tests/golden/`).  If a serialisation format changes
intentionally, update the golden files along with the code.

---

## Headless Bevy Tests

Runtime tests run without a window.  Use **`MinimalPlugins`** as the base, and
add only the plugins the test needs:

```rust
fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);          // always
    app.add_plugins(bevy::input::InputPlugin); // only if you need keyboard/mouse
    app.add_plugins(MapLoaderPlugin { .. });
    app
}
```

Never use `DefaultPlugins` in tests — it tries to open a window and will panic
in CI environments without a display.
