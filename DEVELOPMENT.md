# 开发指南 / Development Guide

## 本地环境要求

- Rust stable（1.75+）
- `wasm32-unknown-unknown` target（`rustup target add wasm32-unknown-unknown`）
- Linux 需要：`pkg-config libudev-dev libasound2-dev libwayland-dev libxkbcommon-dev`

## 常用命令

| 命令 | 说明 |
|------|------|
| `cargo run -p igame-runtime` | 运行桌面 demo（含示例地图）|
| `cargo test --workspace` | 运行全部测试 |
| `cargo test -p igame-shared` | 仅运行 shared 单元/集成测试 |
| `cargo fmt --all` | 格式化代码 |
| `cargo clippy --all-targets -- -D warnings` | Lint 检查 |
| `cargo build -p igame-shared --target wasm32-unknown-unknown` | wasm 构建验证 |

## Crate 说明

### `igame-shared`
- MapPackage 数据结构（`manifest.rs`、`scene.rs`）
- 序列化：manifest=TOML，scene=JSON
- 校验器（`validation.rs`）：结构校验、字段非空、重复ID检测
- `MapPackage` 加载器（`map_package.rs`）

### `igame-runtime`
- 入口：`main.rs` → Bevy `App`
- `camera.rs`：RTS 相机（WASD 平移 + 滚轮缩放）
- `map_loader.rs`：加载 `maps/example_map/` 并生成实体
- `scene_setup.rs`：地面平面 + 方向光

### `igame-editor` / `igame-ugc`
- 当前为 stub，未来实现 egui 编辑器与 UGC 分发

## MapPackage 格式规范 v0

### manifest.toml 字段

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `schema_version` | u32 | ✅ | 当前必须为 `1` |
| `map.id` | String | ✅ | snake_case 唯一标识符 |
| `map.name` | String | ✅ | 显示名称 |
| `map.version` | String | ✅ | SemVer 版本号 |
| `map.author` | String | ✅ | 作者名 |
| `map.engine_min_version` | String | ✅ | 最低引擎版本 |
| `map.entry_scene` | String | ✅ | 入口场景路径 |
| `map.description` | String | ❌ | 描述（可选）|
| `map.preview_image` | String? | ❌ | 预览图路径（可选）|

### scene JSON 字段

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `schema_version` | u32 | ✅ | 当前必须为 `1` |
| `entities` | Array | ✅ | 实体列表（可为空）|
| `entities[].id` | String | ✅ | 场景内唯一 ID |
| `entities[].name` | String? | ❌ | 显示名称 |
| `entities[].transform` | Object | ❌ | 默认恒等变换 |
| `entities[].components` | Object | ❌ | 附加组件（任意 JSON）|

## CI/CD

PR 必须通过以下检查才能自动合并：
- `fmt`：`cargo fmt --all -- --check`
- `clippy`：`cargo clippy --all-targets -- -D warnings`
- `test`：`cargo test --workspace`
- `wasm-build`：`cargo build -p igame-shared --target wasm32-unknown-unknown`

自动合并条件（作者白名单策略）：
- PR 非 Draft
- PR 非 fork
- 作者在白名单（`dulingzhi`、GitHub Actions bot）
- 标题不含 `WIP` / `DO NOT MERGE`
- 无 `do-not-merge` / `blocked` 标签
- CI 全绿 → 自动 Rebase Merge

## 新增地图包

1. 在 `maps/` 下创建目录（如 `maps/my_map/`）
2. 添加 `manifest.toml`（参照格式规范）
3. 添加 `scene/main.json`（参照格式规范）
4. 在 `crates/shared/tests/integration_test.rs` 添加对应测试
