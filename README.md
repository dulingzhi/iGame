# iGame

基于 **Rust / Bevy（ECS）** 构建的数据驱动跨平台游戏引擎，面向桌面端（Desktop）与 Web（wasm）。
配套提供类似《魔兽争霸III世界编辑器》的 **UGC 编辑器**，让玩家自行制作地图、玩法并发布试玩。

👉 **[完整路线图 ROADMAP.md](./ROADMAP.md)**

## 快速开始 / Quick Start

```bash
# 桌面运行（含示例地图）
cargo run -p igame-runtime

# 运行所有测试
cargo test --workspace

# 检查代码风格
cargo fmt --all -- --check

# Lint
cargo clippy --all-targets -- -D warnings

# Wasm 构建验证（仅 shared crate）
cargo build -p igame-shared --target wasm32-unknown-unknown
```

## 工程结构 / Project Structure

```
crates/
├── shared/    # 共享数据结构：MapPackage 格式、序列化、校验
├── runtime/   # Bevy 运行时：场景加载、RTS 相机、实体生成
├── editor/    # 编辑器（stub，待实现）
└── ugc/       # UGC 包管理（stub，待实现）
maps/
└── example_map/  # 示例地图包
    ├── manifest.toml
    └── scene/main.json
```

## MapPackage 格式 / MapPackage Format

**manifest.toml** — 地图元信息：

```toml
schema_version = 1

[map]
id = "my_map"
name = "我的地图"
version = "1.0.0"
author = "作者名"
engine_min_version = "0.1.0"
entry_scene = "scene/main.json"
```

**scene/main.json** — 场景实体列表：

```json
{
  "schema_version": 1,
  "entities": [
    {
      "id": "my_entity",
      "name": "My Entity",
      "transform": {
        "translation": [0.0, 0.5, 0.0],
        "scale": [1.0, 1.0, 1.0],
        "rotation": [0.0, 0.0, 0.0, 1.0]
      }
    }
  ]
}
```

👉 更多细节见 [DEVELOPMENT.md](./DEVELOPMENT.md) · [TESTING.md](./TESTING.md)
