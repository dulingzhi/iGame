# iGame Editor

A lightweight scene editor built with **Bevy 0.18** and **bevy_egui 0.39**.

## Features

- **3-D viewport** rendered to an off-screen texture and embedded in an egui panel.
- **Orbit camera** – right-click drag to orbit, scroll to zoom, middle-click drag to pan.
- **Object selection** – left-click in the viewport to select an object; the selected entity is
  highlighted with a yellow bounding-box gizmo.
- **Translate gizmo** – drag the X (red), Y (green), or Z (blue) axis arrows to move a selected
  object along that axis.
- **Scene panel** – shows every entity in the scene; selected entity name highlighted in yellow.
- **Properties panel** – displays position, rotation and scale of the selected object.
- **Grid** – infinite XZ grid with world-origin axis indicators.

## Crate structure

```
crates/
  igame_runtime/   # Pure math library (bevy_math only, no rendering)
  igame_editor/    # Editor binary (Bevy + bevy_egui)
```

### igame_runtime

| Function | Description |
|---|---|
| `ray_plane_intersection` | Ray × infinite plane |
| `ray_aabb_intersection` | Ray × axis-aligned bounding box |
| `ray_segment_distance` | Shortest distance from ray to line segment |
| `gizmo_axis_project` | Project a world point onto a gizmo axis |

All functions have comprehensive unit tests (`cargo test --package igame_runtime`).

### igame_editor modules

| Module | Responsibility |
|---|---|
| `main.rs` | App setup, `EditorPlugin` |
| `scene.rs` | Scene setup (meshes, lights, grid gizmo) |
| `viewport.rs` | Render-texture creation, `ViewportState` resource |
| `camera.rs` | `OrbitCamera` resource + controller system |
| `selection.rs` | `Selection` resource + click-to-pick system |
| `gizmo.rs` | `GizmoDrag` resource + draw + drag systems |
| `ui.rs` | egui side-panel and central viewport panel |

## Controls

| Input | Action |
|---|---|
| Left-click | Select object |
| Left-drag on axis arrow | Translate along that axis |
| Right-drag | Orbit camera |
| Middle-drag | Pan camera |
| Scroll wheel | Zoom |

## Building

```bash
# Check everything
cargo check

# Run unit tests for the math library
cargo test --package igame_runtime

# Launch the editor (requires a display)
cargo run --package igame_editor
```

## CI

The GitHub Actions workflow (`.github/workflows/ci.yml`) runs:
1. `cargo fmt --check`
2. `cargo clippy -D warnings` on igame_runtime
3. `cargo test` on igame_runtime
4. `cargo check` on igame_editor
5. WASM target check on igame_runtime
