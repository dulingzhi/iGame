# iGame 项目路线图（ROADMAP）

> **目标**：构建一个**数据驱动（ECS）**的跨平台游戏运行时（Desktop + Web/wasm），并提供类似《魔兽争霸III世界编辑器》的 **UGC 编辑器**，让玩家自行制作地图/玩法并发布试玩。  
> 运行时与编辑器共享同一套数据格式、资产规范与校验体系。  
> 技术栈：**Rust / Bevy（ECS）**，支持 Desktop（Windows/macOS/Linux）与 Web（wasm）。

---

## 当前实现状态 / Current Implementation Status

| 里程碑 | 名称                      | 状态        |
|--------|---------------------------|-------------|
| M0     | Project Scaffold          | ✅ 已完成    |
| M1     | MapPackage Format v0      | ✅ 已完成    |
| M2     | Runtime MVP               | ✅ 已完成    |
| M3     | Editor Stub → Basic UI    | 🔲 计划中   |
| M4     | Tilemap Support           | 🔲 计划中   |
| M5     | Scripting (Lua/Rhai)      | 🔲 计划中   |
| M6     | Asset Pipeline            | 🔲 计划中   |
| M7     | Multiplayer Foundation    | 🔲 计划中   |
| M8     | Map Marketplace / Sharing | 🔲 计划中   |
| M9     | Mobile / WASM Export      | 🔲 计划中   |
| M10    | 1.0 Release               | 🔲 计划中   |

**M0–M2 验收清单（已完成）：**
- [x] Cargo workspace: `shared` / `runtime` / `editor` crates
- [x] GitHub Actions CI: fmt → clippy → test → WASM build
- [x] `Manifest` + `MapScene` + `EntityData` 数据结构，完整 serde 序列化
- [x] `MapPackage::load(path)` 加载 manifest.toml + scene.ron
- [x] `validate()` 校验器，带类型化错误
- [x] 27 个单元/集成/金测 全部通过
- [x] Bevy 2D 运行时：RTS 相机（WASD + 滚轮）、场景实体生成
- [x] `assets/maps/demo/` 示例地图包（4 个实体）

---

## 目录

1. [产品范围与非目标](#1-产品范围与非目标)
2. [总体架构建议](#2-总体架构建议)
3. [里程碑（M0 ~ M10）](#3-里程碑m0--m10)
4. [功能清单（Checklist）](#4-功能清单checklist)
5. [触发器系统 MVP 节点清单](#5-触发器系统-mvp-节点清单)
6. [GitHub 项目管理建议](#6-github-项目管理建议)
7. [垂直切片验收样例地图](#7-垂直切片验收样例地图)

---

## 1. 产品范围与非目标

### 1.1 核心范围（必须交付）

| 层次 | 说明 |
|------|------|
| **Runtime（运行时）** | 加载地图包 → 实例化世界（ECS）→ 运行规则与触发器 → 桌面 + Web 均可运行 |
| **Editor（编辑器，桌面优先）** | 场景编辑、资源管理、Inspector、触发器节点图、Play-In-Editor（PIE） |
| **MapPackage（地图包体系）** | 地图包格式规范、依赖管理、版本迁移、完整校验与错误提示 |
| **UGC 平台基础** | 内容索引、打包发布、Web 试玩入口、基础安全与沙箱（脚本路线时必须） |
| **可扩展性** | 插件/模块化架构（引擎侧），后续支持脚本或更安全的扩展机制 |

### 1.2 非目标（第一阶段明确不做）

- 复杂 AAA 渲染特性（全局光照、电影级后处理）——优先级低，后期可选
- 全功能 Web 编辑器——先做 Web 试玩/预览；Web 编辑器放后期
- "允许玩家写 Rust 并编译"——跨平台与安全/分发成本过高，不作为主路线
- 移动端（iOS/Android）——本期不做，架构上不阻断

---

## 2. 总体架构建议

### 2.1 分层结构

```
┌──────────────────────────────────────────┐
│             UGC Platform（内容分发层）      │
│  内容索引 / 打包发布 / Web 试玩 / 签名      │
├──────────────────────────────────────────┤
│             Editor（编辑器层）              │
│  Viewport / Inspector / Gizmo            │
│  触发器图编辑器 / 资源浏览器 / PIE           │
├──────────────────────────────────────────┤
│             Runtime（运行时层）             │
│  ECS（Components/Resources/Events）       │
│  Schedules/Systems / 触发器解释器           │
│  Asset Server / MapPackage Loader         │
└──────────────────────────────────────────┘
         ↕ 共享：数据格式 / 序列化 / 校验
```

### 2.2 建议 Workspace 结构

```
iGame/
├── crates/
│   ├── runtime/      # 游戏运行时（Bevy App、ECS、规则、触发器解释器）
│   ├── editor/       # 编辑器（Bevy App + UI、场景视图、触发器图 UI）
│   ├── shared/       # 共享数据结构、序列化、校验、版本迁移
│   └── ugc/          # 包管理、依赖解析、内容索引（计划中）
├── assets/
│   └── maps/demo/    # 示例地图包
├── docs/             # 设计文档
├── ROADMAP.md
├── README.md
└── Cargo.toml        # workspace 根
```

### 2.3 地图包（Map Package）规范（v0 已实现，v1 规划中）

建议以目录或 zip 为载体：
- `manifest.toml`：地图名、作者、版本、引擎最小版本、依赖列表、入口场景、预览图等
- `scene/`（或 `scene.ron`）：场景实体与组件（Transform、Sprite、Name、Tags）
- `rules/`：单位/技能/物品/数值表（规划中）
- `triggers/`：触发器图（规划中）
- `assets/`：贴图/模型/音频（规划中）
- `localization/`：本地化文本（规划中）

---

## 3. 里程碑（M0 ~ M10）

> 每个里程碑包含：目标、验收标准（DoD）、关键风险/依赖。

### M0 — 仓库初始化与工程基建 ✅

**目标**：可持续开发的工程骨架与规范落地。

**范围**
- workspace 结构：`crates/runtime/`、`crates/editor/`、`crates/shared/`
- CI：fmt / clippy / test（GitHub Actions）
- 基础文档：README + ROADMAP + Makefile

**DoD**
- [x] 本地 `cargo test`、`cargo fmt`、`cargo clippy` 均通过
- [x] CI 通过，README 可运行一个最小 demo
- [x] `igame-shared` 可编译至 `wasm32-unknown-unknown`

---

### M1 — MapPackage 格式 v0 ✅

**目标**：定义并实现地图包核心数据结构。

**范围**
- `Manifest` 结构体：name、version、author、description、entry_scene 等
- `MapScene` + `EntityData`：Transform、Sprite、Name、Tags
- `MapPackage::load(path)` 读取 manifest.toml + scene.ron
- `validate()` 函数与类型化错误

**DoD**
- [x] 运行时可以加载 `MapPackage` 并生成实体
- [x] 校验失败有清晰错误报告（文件、字段、原因）
- [x] 单元测试、集成测试、金测（golden test）全部通过

**风险/依赖**
- 序列化格式一旦发布给 UGC 作者，后续迁移成本会放大 → 需要"版本迁移"机制

---

### M2 — Runtime MVP（最小可运行闭环）✅

**目标**：能在桌面跑一个"地图"并互动。

**范围**
- 基础 App 初始化、日志、状态机（Loading/Playing）
- RTS 相机控制（WASD 平移 + 滚轮缩放）
- 最小 2D 渲染（地面、单位实体）
- 加载并实例化 `MapPackage` 实体

**DoD**
- [x] Desktop：加载示例地图，可视化呈现 4 个实体
- [x] 头less Bevy 集成测试（`MinimalPlugins`）全部通过

**风险/依赖**
- Web 平台限制（线程、文件系统、音频、网络）在后续里程碑踩坑

---

### M3 — Editor MVP（桌面）🔲

**目标**：具备 War3 编辑器"放东西、改属性、保存"的最小闭环。

**范围**
- Viewport：相机/网格/选中高亮
- 基础 Gizmo：移动/旋转/缩放（先移动）
- Hierarchy：实体列表、分组、搜索
- Inspector：编辑组件字段（位置、旋转、缩放、资源引用、基础自定义组件）
- 资源浏览器：导入资产、预览、引用
- Save/Load：输出 `MapPackage v1`
- PIE：在编辑器内启动运行时

**DoD**
- 作者可以制作一个简单地图并保存，运行时可加载并一致呈现

**风险/依赖**
- UI 框架选型（egui/bevy_ui/其他）影响编辑器效率与可维护性

---

### M4 — 触发器系统 MVP 🔲

**目标**：实现 War3 核心：事件-条件-动作（ECA）触发器，作者无需写代码即可做玩法逻辑。

**范围**
- Trigger Graph 数据结构：节点、端口、连线、变量、常量
- 运行时解释器：按事件触发执行，支持条件分支
- 调试：日志、执行步进（最简）、错误定位
- 编辑器：节点图 UI（拖拽连线、节点搜索、参数编辑）

**DoD**
- 用触发器实现 2–3 个"玩法示例"（刷怪波次、胜利条件、对话/提示）
- 触发器错误能提示到具体节点与原因

---

### M5 — Web 试玩与内容分发 MVP 🔲

**目标**：用户能在 Web 上选择地图并试玩；为后续 UGC 社区铺路。

**范围**
- Web 运行时打包与加载策略（assets bundling / 远程拉取）
- 地图索引（本地 JSON 或远程服务均可，先静态）
- 下载/缓存（浏览器缓存策略）
- 最小"地图选择"UI

**DoD**
- Web 页面可列出地图 → 点击进入加载 → 成功游玩

---

### M6 — 资产管线增强 + 依赖与共享资源包 🔲

**目标**：UGC 扩展到"模块化内容"：地图可依赖公共资源包、版本可控。

**范围**
- 资源包（Asset Pack）概念：与地图包分离
- 依赖解析与冲突处理
- 校验器增强：依赖缺失/版本不兼容提示
- 可选：热重载（桌面）

---

### M7 — 扩展机制 v1：插件/脚本与沙箱 🔲

**目标**：允许高级作者做更复杂逻辑，同时保证跨平台与安全。

**范围（路线二选一或组合）**
- 路线 A：脚本（Lua/Rhai）—— API 白名单、资源访问限制、执行时间限制
- 路线 B：WASM 脚本——更强隔离，工具链/性能/调试体验需评估

**风险/依赖**
- 沙箱与权限模型是 UGC 平台的核心风险点（安全、作弊、资源滥用）

---

### M8 — 联机/回放（可选）🔲

**目标**：支持多人或回放，提升作品上限。

**范围**
- 回放：输入记录或事件记录
- 联机：权威服务器/锁步/预测回滚（根据玩法选）

---

### M9 — Beta（稳定性与内容生产）🔲

**目标**：让一批作者真正用起来，收集反馈并修正工作流。

**范围**
- 编辑器稳定性（崩溃恢复、自动保存、错误提示）
- 示例内容与模板地图
- 文档：入门、触发器手册、资源规范
- 性能 Profiling 与关键路径优化

---

### M10 — 1.0 发布 🔲

**目标**：冻结核心格式与 API，形成可持续迭代的版本体系。

**范围**
- MapPackage v1 格式冻结
- 触发器节点集 v1 冻结（保证向后兼容策略）
- 版本迁移工具链（至少支持 v1.x 迁移）
- 发布流程与版本说明（Changelog）

---

## 4. 功能清单（Checklist）

### 4.1 Runtime（桌面 + Web）
- [x] App 生命周期：Boot/Loading/Playing
- [x] 渲染：基础 2D（Bevy Sprite）
- [x] 摄像机：RTS 相机（平移/缩放）
- [x] 输入：键鼠（Web 兼容）
- [ ] UI：HUD、提示、菜单
- [ ] 音频：BGM/SFX（Web 注意限制）
- [ ] 物理/碰撞：简单 AABB/射线拾取
- [ ] 动画：基础播放
- [ ] 寻路（可选）：Navmesh（偏 RTS）
- [ ] 性能监控：帧时间、实体数、系统耗时（debug overlay）

### 4.2 数据与地图包
- [x] MapPackage v0：manifest + scene + validation
- [ ] MapPackage v1：rules / triggers / assets / localization
- [ ] 依赖管理：资源包、版本范围、冲突策略
- [ ] 版本迁移：migrator 接口、迁移测试用例

### 4.3 编辑器（桌面优先）
- [ ] 项目/地图管理：新建/打开/最近项目
- [ ] Viewport：网格、坐标轴、选中高亮
- [ ] Gizmo：移动/旋转/缩放 + 吸附
- [ ] Hierarchy：分组、搜索、锁定、隐藏
- [ ] Inspector：组件编辑
- [ ] 资源浏览器：导入、预览、引用
- [ ] 触发器编辑器：节点搜索、连线、变量面板、错误提示
- [ ] PIE：一键运行/停止

### 4.4 触发器系统（ECA）
- [ ] 事件系统：内置事件 + 自定义事件
- [ ] 条件系统：逻辑组合、比较、随机
- [ ] 动作系统：生成/销毁/改属性/播放效果/UI/胜负
- [ ] 变量系统：全局/地图内、类型（int/float/bool/string/entity）
- [ ] 调试：执行日志、节点定位

### 4.5 UGC 分发与安全
- [ ] 打包：zip/目录
- [ ] 内容索引：本地/远程
- [ ] 下载与缓存
- [ ] 权限/沙箱（脚本路线时必须）

### 4.6 工程与质量
- [x] CI：fmt/clippy/test + wasm build
- [x] 测试：序列化金测、校验器用例、运行时集成测试
- [x] 示例地图包（`assets/maps/demo/`）
- [ ] 文档：作者手册、触发器节点参考、资源规范
- [ ] 更多示例地图与模板

---

## 5. 触发器系统 MVP 节点清单

**事件（Events）**
1. Game Start（游戏开始）
2. Timer Every N Seconds（周期计时器）
3. Unit Created（单位创建）
4. Unit Died（单位死亡）
5. Unit Enter Region（进入区域）
6. Custom Event Fired（自定义事件）

**条件（Conditions）**
7. Compare Int/Float（数值比较）
8. And / Or / Not（逻辑组合）
9. Unit Has Tag / Faction Is（阵营/标签判断）
10. Random Chance（随机概率）
11. Is In Region（是否在区域）

**动作（Actions）**
12. Spawn Unit（在点/区域生成单位）
13. Destroy Entity（销毁实体）
14. Set Variable（变量赋值）
15. Add/Remove Component（加/删状态：如 Buff/可交互标记）
16. Play Sound / Play VFX（音效/特效）
17. Show Message / UI Toast（提示文字）
18. Set Victory / Defeat（胜利/失败）
19. Fire Custom Event（触发自定义事件）
20. Wait / Delay（延迟执行）

---

## 6. GitHub 项目管理建议

### 6.1 Milestones
- M0 基建 ✅
- M1 MapPackage v0 ✅
- M2 Runtime MVP ✅
- M3 Editor MVP
- M4 Triggers MVP
- M5 Web Play MVP
- M6 Asset/Deps
- M7 Extensibility/Sandbox
- M9 Beta
- M10 1.0

### 6.2 Labels（建议）
- `area:runtime` `area:editor` `area:ugc` `area:triggers` `area:assets` `area:web`
- `type:bug` `type:feature` `type:refactor` `type:doc` `type:test`
- `priority:p0` `priority:p1` `priority:p2`
- `platform:desktop` `platform:web`
- `good-first-issue`

---

## 7. 垂直切片验收样例地图

建议尽早定义一个"垂直切片地图"作为验收基准（贯穿所有里程碑的防回归基线）：

- 一张地图（地形+资源）
- 1 个单位类型（移动+死亡）
- 1 个刷怪触发器（周期生成）
- 1 个胜利条件触发器（击杀计数达标）
- 桌面与 Web 都能跑

每个里程碑都要求这张地图仍可运行（防回归、保证兼容性）。

当前已实现基础：`assets/maps/demo/` 包含地面 + 玩家单位 + 敌方单位 + 地标，可作为切片地图的起点。
