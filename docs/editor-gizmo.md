# Editor Gizmo 操作说明（移动 Gizmo — War3 风格）

> **适用版本：** editor_gizmo v0.1+  
> **平台：** Desktop（鼠标 + 键盘）

---

## 1. 交互概览

| 操作 | 效果 |
|------|------|
| 鼠标左键拖拽实体（自由区域） | 在 **XZ 地面平面**（Y 轴不变）自由移动 |
| 拖拽 **X 轴手柄**（红色箭头） | 锁定 **X 轴**，只允许 X 方向移动 |
| 拖拽 **Z 轴手柄**（蓝色箭头） | 锁定 **Z 轴**，只允许 Z 方向移动 |
| 拖拽 **XZ 平面手柄**（中心圆盘） | 锁定 **XZ 平面**，Y 轴不变（与自由拖拽相同） |
| 拖拽时按住 **Shift** | 切换为**单轴约束**：自动选择当前拖拽位移中绝对值更大的轴（X 或 Z） |
| 松开 Shift / 手柄 | 恢复 XZ 平面模式 |
| 拖拽时按住鼠标右键（相机控制） | 立即**取消当前拖拽**并进入相机旋转模式（两者互斥） |

---

## 2. War3 设计对比

| War3 World Editor 行为 | iGame Editor 对应 |
|------------------------|-------------------|
| 默认拖拽在地面平面移动 | ✅ 默认 XZ 平面 |
| 按手柄轴锁定单轴 | ✅ X / Z 轴手柄 |
| 可选网格吸附（Snap） | ✅ 可配置 SnapConfig |
| 相机控制与移动不冲突 | ✅ 互斥状态机 |

---

## 3. 网格吸附（Snap）

### 3.1 配置 SnapConfig

```rust
use editor_gizmo::snap::SnapConfig;

// 关闭吸附（默认）
let snap = SnapConfig::DISABLED;

// 1 单位网格
let snap = SnapConfig::new(1.0);

// 0.5 单位网格
let snap = SnapConfig::new(0.5);
```

将 `SnapConfig` 赋给 `GizmoState::snap` 字段即可生效：

```rust
gizmo_state.snap = SnapConfig::new(1.0);
```

### 3.2 吸附行为说明

- **仅 XZ 平面吸附**：Y 轴（高度）不受吸附影响，保留精确高度值。
- 吸附在 `update_drag` 返回最终位置之前完成，调用方无需额外处理。
- `step <= 0` 时，吸附自动禁用（等同于 `SnapConfig::DISABLED`）。

---

## 4. 与相机控制的互斥逻辑

```text
┌─────────────────────────────────────────────────────┐
│ 用户按下鼠标右键（相机控制开始）                          │
│   → GizmoState::notify_camera_start()               │
│   → 正在进行的 Gizmo 拖拽被立即取消（end_drag）         │
│   → camera_active = true                            │
│   → 所有新 begin_drag 调用返回 None（忽略）             │
├─────────────────────────────────────────────────────┤
│ 用户松开鼠标右键（相机控制结束）                          │
│   → GizmoState::notify_camera_end()                 │
│   → camera_active = false                           │
│   → Gizmo 可以再次响应拖拽                              │
└─────────────────────────────────────────────────────┘
```

---

## 5. API 快速参考

### 5.1 完整拖拽生命周期

```rust
use editor_gizmo::gizmo::{GizmoHandle, GizmoState};
use editor_gizmo::math::{Ray, Vec3};
use editor_gizmo::snap::SnapConfig;

let mut gizmo = GizmoState::new();
gizmo.snap = SnapConfig::new(1.0); // 可选：1 单位网格吸附

let entity_pos = Vec3::new(5.0, 0.0, 3.0);

// ① 鼠标按下 → 开始拖拽
let pick_ray = /* 由相机矩阵+鼠标位置生成的世界空间射线 */;
if let Some(mode) = gizmo.begin_drag(GizmoHandle::XzPlane, shift_key_held, entity_pos, pick_ray) {
    println!("开始拖拽，模式: {mode:?}");
}

// ② 鼠标移动 → 更新位置
let current_ray = /* 当前鼠标位置对应的射线 */;
if let Some(new_pos) = gizmo.update_drag(current_ray, shift_key_held, entity_pos) {
    // 将实体移动到 new_pos
}

// ③ 鼠标松开 → 结束拖拽
gizmo.end_drag();
```

### 5.2 悬停高亮

```rust
// 当鼠标悬停在 X 轴手柄上时
gizmo.set_hover(Some(GizmoHandle::AxisX));

// 鼠标离开所有手柄时
gizmo.set_hover(None);
```

高亮状态保存在 `GizmoState::highlight: [HighlightState; 3]`：
- 索引 0 → XZ 平面手柄
- 索引 1 → X 轴手柄
- 索引 2 → Z 轴手柄

---

## 6. 验收标准（DoD）

- [x] 默认拖拽在 XZ 地面平面内移动，Y 轴不变
- [x] 拖拽 X 轴手柄时 Z 轴锁定（Z 坐标不变）
- [x] 拖拽 Z 轴手柄时 X 轴锁定（X 坐标不变）
- [x] Shift 拖拽时自动选择位移最大轴约束为单轴
- [x] 网格吸附可配置步长（step），只对 XZ 生效
- [x] 相机控制与 Gizmo 拖拽互斥，相机开始时取消当前拖拽
- [x] 高亮状态（Normal / Hovered / Active）正确跟踪当前手柄
- [x] 38 个纯逻辑测试（射线-平面求交、位移计算、吸附）全部通过
- [x] `cargo clippy -D warnings` 无告警
