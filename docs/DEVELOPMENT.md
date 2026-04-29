# Development Guide

This document provides an in-depth technical reference for developing iGame: architecture decisions, build targets, testing strategy, and the iterative development loop.

---

## Table of Contents

1. [Architecture overview](#architecture-overview)
2. [Workspace crates](#workspace-crates)
3. [Building for different targets](#building-for-different-targets)
4. [Testing strategy](#testing-strategy)
5. [Development loop](#development-loop)
6. [Roadmap reference](#roadmap-reference)

---

## Architecture overview

iGame is structured as a **Bevy-based UGC platform**:

```
┌─────────────────────────────────────────────────────────┐
│                    Editor (desktop)                      │
│  egui UI  │  Viewport  │  Trigger Graph  │  Asset Browser│
└──────────────────────────┬──────────────────────────────┘
                           │ loads / saves
                           ▼
┌─────────────────────────────────────────────────────────┐
│                  Map Package (data)                      │
│  manifest.toml │ scene/*.json │ triggers/*.json          │
│  rules/*.json  │ assets/      │ localization/            │
└──────────────────────────┬──────────────────────────────┘
                           │ parsed by igame-shared
                           ▼
┌─────────────────────────────────────────────────────────┐
│               Runtime  (desktop + wasm32)                │
│  World (ECS)  │  Systems  │  Trigger Interpreter         │
└─────────────────────────────────────────────────────────┘
```

Key design decisions:

- **Data-driven**: all game content lives in JSON/TOML files, not Rust code.
- **ECS**: the runtime uses an Entity-Component-System model (Bevy in later milestones).
- **ECA triggers**: a visual Event-Condition-Action graph drives gameplay logic without requiring players to write code.
- **Desktop-first editor, Web-compatible runtime**: the editor targets native desktop; the runtime compiles to `wasm32-unknown-unknown` for browser play.

---

## Workspace crates

| Crate | Path | wasm32 | Purpose |
|-------|------|--------|---------|
| `igame-shared` | `crates/shared` | ✅ | Data structures, serialisation, validation |
| `igame-runtime` | `crates/runtime` | ✅ | ECS world, systems, trigger interpreter |
| `igame-editor` | `crates/editor` | ❌ | Desktop editor (egui UI, scene tools) |

### Crate dependency graph

```
igame-editor
  ├── igame-runtime
  │     └── igame-shared
  └── igame-shared
```

`igame-shared` has **no** internal dependencies and is the single source of truth for the map package format.

---

## Building for different targets

### Native (desktop)

```bash
# Debug build (all crates)
cargo build --workspace

# Release build
cargo build --workspace --release

# Run the editor (once binary target is added)
cargo run -p igame-editor
```

### Web (`wasm32-unknown-unknown`)

Only `igame-shared` and `igame-runtime` are wasm32-compatible.  
The editor is **excluded** from wasm32 builds.

```bash
# Add the target once
rustup target add wasm32-unknown-unknown

# Build wasm32 crates
cargo build --target wasm32-unknown-unknown -p igame-shared -p igame-runtime

# (Future) bundle with wasm-pack for the browser
wasm-pack build crates/runtime --target web
```

> **wasm32 compatibility rules**:  
> - Never use `std::fs`, `std::net`, or `std::thread` directly in `igame-shared` or `igame-runtime`.  
> - Gate any platform-specific code with `#[cfg(not(target_arch = "wasm32"))]`.  
> - Add integration tests that verify the wasm32 build on every PR (handled by CI).

---

## Testing strategy

### Unit tests

Every module should have a `#[cfg(test)]` block. Focus on:

- Serialisation/deserialisation round-trips (TOML manifests, JSON scenes, trigger graphs).
- Validation logic (empty fields, missing references, version incompatibilities).
- Pure data transformations (component lookups, ECA graph traversal).

```bash
cargo test --workspace
```

### Integration tests

Place integration tests under `crates/<crate>/tests/`.  
These test end-to-end flows: load a fixture map package → spawn entities → run N ticks → assert world state.

```bash
cargo test --workspace --test '*'
```

### Golden / snapshot tests

For stability of serialised formats, keep reference fixtures in `crates/shared/tests/fixtures/` and assert that the parser produces the expected output.

### CI matrix

| Check | Scope | Fail-fast |
|-------|-------|-----------|
| `cargo fmt --check` | all crates | yes |
| `cargo clippy -- -D warnings` | all crates, all targets | yes |
| `cargo test` | all crates | yes |
| `cargo build --target wasm32-unknown-unknown` | shared + runtime | yes |

---

## Development loop

The recommended cycle for each feature:

```
1. Write a failing test (TDD preferred)
       ↓
2. Implement the feature
       ↓
3. cargo test --workspace            ← verify tests pass
4. cargo clippy -- -D warnings       ← no new warnings
5. cargo fmt --all                   ← tidy formatting
       ↓
6. Commit, open PR
       ↓
7. Auto-merge workflow checks eligibility automatically (Safety Mode A):
     • Author in allowlist + same-repo branch + non-draft
     • No "do-not-merge" label + no "WIP" in title
     • If eligible → GitHub native auto-merge (rebase) enabled
       ↓
8. CI green → auto rebase-merge
       ↓
9. Move to the next feature
```

> **No manual label required.** To opt-out of auto-merge: mark the PR as
> **Draft**, add the `do-not-merge` label, or include `WIP` in the title.

### Auto-merge eligibility (Safety Mode A)

| Gate | Rule |
|------|------|
| Author allowlist | `dulingzhi`, `github-actions[bot]`, `copilot-swe-agent[bot]` |
| Same repository | `head.repo == base.repo` (forks excluded) |
| Not a draft | PR must be marked *Ready for review* |
| No WIP in title | Title must not match `\bWIP\b` (case-insensitive) |
| No block label | `do-not-merge` label absent |

### Required repository settings (one-time setup)

1. **Settings → General → Pull Requests → Allow auto-merge** ✅
2. **Settings → General → Pull Requests → Allow rebase merging** ✅
3. **Settings → Branches → Branch protection for `main`**:
   - Require status checks: `Formatting`, `Clippy`, `Tests`, `WASM Build (shared)`
   - Require branches to be up to date before merging
4. **Settings → Actions → General → Workflow permissions**: Read and write ✅

---

## Roadmap reference

See [ROADMAP.md](../ROADMAP.md) for the full milestone plan.  
Key milestones relevant to contributors:

| Milestone | Focus |
|-----------|-------|
| M0 | Engineering foundation (this PR) |
| M1 | Runtime MVP: scene loading, camera, input |
| M2 | Map Package v1: manifest + scene + validation |
| M3 | Editor MVP: viewport, inspector, save/load |
| M4 | Trigger system: ECA node graph + interpreter |
| M5 | Web play: wasm bundle, map index, browser UI |
