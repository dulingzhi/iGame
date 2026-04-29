# iGame

A Bevy 0.18 game editor with an egui-based UI.

## Quick start

```bash
# Run unit tests
cargo test --package igame_runtime

# Launch the editor (requires a display / GPU)
cargo run --package igame_editor
```

## Documentation

See [`docs/editor.md`](docs/editor.md) for a full feature overview, crate
structure, and control reference.

## Workspace

| Crate | Description |
|---|---|
| `crates/igame_runtime` | Pure math utilities (no rendering deps) |
| `crates/igame_editor` | Editor binary – Bevy 0.18 + bevy_egui 0.39 |
