# iGame

[![CI](https://github.com/dulingzhi/iGame/actions/workflows/ci.yml/badge.svg)](https://github.com/dulingzhi/iGame/actions/workflows/ci.yml)

基于 **Rust / Bevy（ECS）** 构建的数据驱动跨平台游戏引擎，面向桌面端（Desktop）与 Web（wasm），配套提供类似《魔兽争霸III世界编辑器》的 **UGC 编辑器**，让玩家自行制作地图、玩法并发布试玩。

## 项目路线图

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

## Quick start

```bash
# Clone
git clone https://github.com/dulingzhi/iGame.git
cd iGame

# Run all workspace tests
cargo test --workspace

# Run Clippy (same flags as CI)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check

# Build wasm32-compatible crates
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown -p igame-shared -p igame-runtime
```

## Documentation

| Document | Description |
|----------|-------------|
| [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) | Setup, local commands, CI, auto-merge, branch protection |
| [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) | Architecture, build targets, testing strategy |
| [ROADMAP.md](ROADMAP.md) | Full milestone plan (M0 → 1.0) |

## CI & Auto-merge

Every PR runs four checks automatically: **Rustfmt**, **Clippy**, **Tests**, and a **wasm32 build**.

To have a PR merged automatically once all checks are green, add the **`automerge`** label.  
See [docs/CONTRIBUTING.md – Auto-merge](docs/CONTRIBUTING.md#auto-merge-with-the-automerge-label) for full details.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

