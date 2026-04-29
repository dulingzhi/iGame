# 如何统一查看 Agent / 任务进度

iGame 的自动化由 **Copilot Coding Agent** 驱动。以下三处是你需要了解的"统一进度入口"。

---

## 1. Copilot Tasks 链接（最统一）

每次 Copilot Agent 开始执行时，都会在聊天中生成一个任务链接，格式为：

```
https://github.com/copilot/tasks/pull/<task-id>?session_id=<session>
```

这里你可以看到：
- 任务当前状态（`queued` / `in_progress` / `completed` / `failed`）
- 关联的 PR（任务完成后会自动链接）
- Agent 执行过程中的日志与备注

**推荐**：把最新任务链接保存在书签或钉在聊天窗口里，随时刷新即可查看当前进度。

---

## 2. Pull Request 页面（看代码 + 审查）

任务完成后，Agent 会开启（或更新）一个 PR。你可以在这里：

- 看到每一次 `git commit`（对应"做了什么"）
- 查看 Agent 在 PR 描述中写的**清单（checklist）**（哪些已完成、哪些待完成）
- 查看 Code Review 意见与 Agent 的回应
- 确认是否满足合并标准（CI 全绿 + 无 `do-not-merge` + 非 Draft）

**直达入口：**

```
https://github.com/dulingzhi/iGame/pulls
```

---

## 3. GitHub Actions 页面（看 CI 状态与日志）

每次向 PR 推送 commit，CI 工作流会自动触发。你可以在这里：

- 看到最新 CI 运行是否全绿（✅）或失败（❌）
- 点开具体的 Job 查看详细编译 / 测试日志
- 确认 `fmt` / `clippy -D warnings` / `test` / 其他检查的结果

**直达入口：**

```
https://github.com/dulingzhi/iGame/actions
```

> **Tip：** 在 Actions 页面选择 `Pull requests` 过滤器，可以只看与你当前 PR 相关的运行记录。

---

## 4. 快速判断"当前是否可以合并"

满足以下所有条件时，PR 会自动进行 **rebase auto-merge**（无需人工操作）：

| 条件 | 说明 |
|------|------|
| ✅ CI 全绿 | Actions 页面所有 required checks 通过 |
| ✅ 非 Draft | PR 不是草稿状态 |
| ✅ 同仓库分支 | 不是 fork 的 PR |
| ✅ 作者白名单 | PR 作者是 `dulingzhi` 或 Agent 账号 |
| ✅ 无阻止标签 | 没有 `do-not-merge` 标签 |
| ✅ 标题无 WIP | PR 标题不以 `WIP` 开头 |

想**阻止**自动合并，任选一个：
- 将 PR 改为 **Draft**
- 加上 `do-not-merge` 标签
- 在标题前加 `WIP:`

---

## 5. 本地开发快速命令参考

```bash
# 运行所有测试
cargo test

# 代码格式检查（CI 会用）
cargo fmt --check

# Lint（CI 会用，-D warnings 视任何 warning 为错误）
cargo clippy -- -D warnings

# 格式化代码
cargo fmt
```
