# 测试指南 / Testing Guide

## 运行测试

```bash
# 全部测试
cargo test --workspace

# 仅 shared crate（含单元测试 + 集成测试 + golden 测试）
cargo test -p igame-shared

# 只跑集成测试
cargo test -p igame-shared --test integration_test

# 只跑 golden 测试
cargo test -p igame-shared --test golden_test

# 只跑 validation 单元测试
cargo test -p igame-shared validation
```

## 测试分类

### 单元测试（`crates/shared/src/validation.rs`）
- `valid_manifest_has_no_errors`：合法 manifest 不产生错误
- `empty_id_is_an_error`：id 为空时报 `EmptyRequiredField`
- `schema_mismatch_is_an_error`：版本不匹配时报 `SchemaMismatch`
- `duplicate_entity_id_detected`：重复 ID 报 `DuplicateEntityId`

### 集成测试（`crates/shared/tests/integration_test.rs`）
- `load_example_map_succeeds`：加载示例地图包，无错误
- `example_map_has_correct_entity_count`：示例地图有 3 个实体
- `example_map_entity_ids_are_unique`：实体 ID 无重复
- `example_map_manifest_id_is_correct`：manifest 字段值正确
- `invalid_json_returns_error`：非法 JSON 返回 Err
- `invalid_toml_returns_error`：非法 TOML 返回 Err

### Golden 测试（`crates/shared/tests/golden_test.rs`）
- `manifest_roundtrip`：manifest 序列化后反序列化结果不变
- `scene_roundtrip`：scene 序列化后反序列化结果不变
- `transform_defaults_are_identity`：默认 transform 为单位变换

## 验收标准 / DoD

- [ ] `cargo test --workspace` 全部通过（0 失败）
- [ ] `cargo clippy --all-targets -- -D warnings` 无警告
- [ ] `cargo fmt --all -- --check` 无变更
- [ ] `cargo build -p igame-shared --target wasm32-unknown-unknown` 成功
- [ ] `cargo run -p igame-runtime` 可运行（显示 3D 场景 + 3 个方块）
- [ ] 示例地图加载日志正常（无 ERROR/WARN）
