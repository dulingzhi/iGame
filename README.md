# iGame

[![CI](https://github.com/dulingzhi/iGame/actions/workflows/ci.yml/badge.svg)](https://github.com/dulingzhi/iGame/actions/workflows/ci.yml)

基于 **Rust / Bevy（ECS）** 构建的数据驱动跨平台游戏引擎，面向桌面端（Desktop）与 Web（wasm），配套提供类似《魔兽争霸III世界编辑器》的 **UGC 编辑器**，让玩家自行制作地图、玩法并发布试玩。

A data-driven, cross-platform UGC game engine and editor built with [Bevy](https://bevyengine.org/) (Rust), inspired by Warcraft III World Editor.

## 项目路线图 / Roadmap

👉 **[查看完整路线图 ROADMAP.md](./ROADMAP.md)**

路线图涵盖：
- 项目目标与非目标
- 总体架构建议（Runtime / Editor / UGC 平台分层）
- 里程碑 M0 ~ M10（目标、验收标准 DoD、关键风险）
- 功能清单 Checklist（运行时、地图包、编辑器、触发器、UGC 分发、工程质量）
- 触发器系统 MVP 节点清单（20 个事件/条件/动作节点）
- GitHub 项目管理建议（Milestones + Labels）
- 垂直切片验收样例地图

---

## Quick Start / 快速开始

### Prerequisites / 前提条件

- Rust (stable, 1.75+): https://rustup.rs
- On Linux: `sudo apt-get install libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev`

### Run the Demo Map / 运行 Demo 地图

```bash
cargo run -p igame-runtime -- assets/maps/demo
```

Use **WASD** or **Arrow keys** to pan the camera, **mouse scroll** to zoom.
Press **Escape** to quit.

### Run All Tests / 运行全部测试

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

---

## Project Structure / 项目结构

```
iGame/
├── crates/
│   ├── shared/     # Core types: MapPackage, Manifest, Scene, validation
│   ├── runtime/    # Bevy app: loads maps, RTS camera, entity spawning
│   └── editor/     # Editor (stub – M3 planned)
├── assets/
│   └── maps/
│       └── demo/   # Example map package
│           ├── manifest.toml
│           └── scene.ron
├── ROADMAP.md
└── Makefile
```

---

## Map Package Format v0 / 地图包格式

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
            tags: ["ground"],
        ),
    ],
)
```

---

## Definition of Done (M0 + M1) / 验收标准

- [x] Workspace compiles (`cargo check --workspace`)
- [x] All tests pass (`cargo test --workspace`)
- [x] Formatting OK (`cargo fmt --all -- --check`)
- [x] Clippy clean (`cargo clippy --workspace -- -D warnings`)
- [x] WASM build for shared crate passes
- [x] Demo map loads and renders (manual verification)

---

## CI & Auto-merge / 持续集成与自动合并

Every pull request automatically runs four checks (fmt, clippy, tests, wasm
build). When all checks pass **auto-merge is enabled automatically** — no label
required.

**Opt out of auto-merge:**
- Mark the PR as a **draft**, or
- Add the **`do-not-merge`** label.

See [docs/CI_AND_AUTOMERGE.md](docs/CI_AND_AUTOMERGE.md) for the full policy
and the one-time repository settings required (Allow auto-merge, branch
protections).

---

## Testing Guide / 测试指南

See [TESTING.md](TESTING.md) for the full test strategy, how to run specific
test suites, and conventions for adding new tests.
