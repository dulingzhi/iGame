# Contributing to iGame

Thank you for your interest in contributing to **iGame**!  
This document covers everything you need to start contributing: setting up your local environment, running the CI checks locally, and getting your PR merged automatically.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Repository layout](#repository-layout)
3. [Local development commands](#local-development-commands)
4. [CI checks](#ci-checks)
5. [Auto-merge with the `automerge` label](#auto-merge-with-the-automerge-label)
6. [Branch protection recommendations](#branch-protection-recommendations)
7. [Code style](#code-style)
8. [Submitting a PR](#submitting-a-pr)

---

## Prerequisites

| Tool | Version | Notes |
|------|---------|-------|
| Rust | stable (≥ 1.75) | Install via [rustup](https://rustup.rs) |
| `rustfmt` | bundled with stable | `rustup component add rustfmt` |
| `clippy` | bundled with stable | `rustup component add clippy` |
| `wasm32-unknown-unknown` target | bundled | `rustup target add wasm32-unknown-unknown` |
| `gh` CLI (optional) | ≥ 2.40 | For manual PR operations |

---

## Repository layout

```
iGame/
├── Cargo.toml              # Workspace manifest (Rust 2021, resolver v2)
├── crates/
│   ├── shared/             # Shared data structures, serialisation, validation
│   ├── runtime/            # Game runtime (desktop + wasm32)
│   └── editor/             # Desktop-only editor (egui-based, M3+)
├── docs/
│   ├── CONTRIBUTING.md     # This file
│   └── DEVELOPMENT.md      # Architecture & in-depth dev guide
└── .github/
    └── workflows/
        ├── ci.yml          # Rustfmt / Clippy / Tests / wasm32 build
        └── auto-merge.yml  # Automatic squash-merge on CI green + label
```

---

## Local development commands

Run these from the **repository root** before opening a PR:

```bash
# 1. Check formatting (no changes applied)
cargo fmt --all -- --check

# 2. Apply formatting
cargo fmt --all

# 3. Lint with Clippy (all warnings are errors, same as CI)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 4. Run all workspace tests
cargo test --workspace

# 5. Build wasm32-compatible crates
cargo build --target wasm32-unknown-unknown -p igame-shared -p igame-runtime

# 6. Run the desktop runtime (once a binary is wired up)
cargo run -p igame-runtime
```

> **Tip:** Create a `Makefile` shortcut or alias `alias ci='cargo fmt --all && cargo clippy --workspace -- -D warnings && cargo test --workspace'` to run the full local CI in one step.

---

## CI checks

Every push and pull request runs four jobs defined in `.github/workflows/ci.yml`:

| Job | Command | Must pass |
|-----|---------|-----------|
| **Rustfmt** | `cargo fmt --all -- --check` | ✅ Required |
| **Clippy** | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | ✅ Required |
| **Tests** | `cargo test --workspace` | ✅ Required |
| **WASM Build** | `cargo build --target wasm32-unknown-unknown -p igame-shared -p igame-runtime` | ✅ Required |

All four jobs must be green before a PR is eligible for merging.

Cargo's dependency cache is managed by [`Swatinem/rust-cache`](https://github.com/Swatinem/rust-cache), which caches the `~/.cargo/registry`, `~/.cargo/git`, and `target/` directories keyed on the `Cargo.lock`.

---

## Auto-merge with the `automerge` label

iGame uses **GitHub's native auto-merge** to automatically squash-merge a PR once every required status check is green.

### How it works

1. A maintainer (or the PR author, if they have write access) adds the **`automerge`** label to the PR.
2. The `Auto Merge` GitHub Actions workflow fires and calls:
   ```
   gh pr merge <PR> --auto --squash --delete-branch
   ```
3. GitHub queues the PR for auto-merge.  
   The PR will be merged automatically the moment all required status checks report success.
4. The source branch is deleted automatically after merging.

### Safety constraints

| Condition | Behaviour |
|-----------|-----------|
| PR comes from a **fork** | ❌ Auto-merge is **not** triggered |
| PR is a **draft** | ❌ Auto-merge is **not** triggered |
| `automerge` label is **absent** | ❌ Auto-merge is **not** triggered |
| Any CI job **fails** | ⏸ Merge is paused until the check recovers |

### Enabling auto-merge on the repository

Auto-merge requires a one-time setting change in the GitHub UI:

1. Go to **Settings → General → Pull Requests**.
2. Tick **"Allow auto-merge"**.
3. Optionally tick **"Automatically delete head branches"** (the workflow already passes `--delete-branch`, so this is redundant but harmless).

> Only repository administrators can change this setting.

### Creating the `automerge` label

Run once (requires write access):

```bash
gh label create automerge \
  --repo dulingzhi/iGame \
  --description "Squash-merge automatically once CI is green" \
  --color 0075ca
```

Or create it manually under **Settings → Labels → New label**.

---

## Branch protection recommendations

For the `main` branch, we recommend enabling the following rules under **Settings → Branches → Add rule**:

| Rule | Recommended setting |
|------|---------------------|
| Require status checks to pass before merging | ✅ Enabled |
| Required status checks | `Rustfmt`, `Clippy`, `Tests`, `WASM Build` |
| Require branches to be up to date before merging | ✅ Enabled |
| Require pull request reviews before merging | Optional (1 approval recommended) |
| Dismiss stale pull request approvals when new commits are pushed | ✅ Enabled |
| Restrict pushes that create matching branches | Optional |
| Do not allow bypassing the above settings | ✅ Recommended for public repos |
| Allow force pushes | ❌ Disabled |
| Allow deletions | ❌ Disabled |

> With these rules in place, direct pushes to `main` are blocked and every change must go through a PR that passes CI.

---

## Code style

- **Rust edition**: 2021 (set in `Cargo.toml`).
- **Formatting**: enforced by `rustfmt` with default settings.
- **Linting**: all `clippy` warnings are treated as errors.
- **Comments**: write doc-comments (`///`) for all public items.
- **Tests**: every new module should have a `#[cfg(test)]` block with at least one meaningful test.

---

## Submitting a PR

1. Fork the repository (or create a branch directly if you have write access).
2. Make your changes on a feature branch (`feat/my-feature`, `fix/issue-42`, etc.).
3. Run the full local CI suite (`cargo fmt`, `cargo clippy`, `cargo test`, wasm build).
4. Open a PR targeting `main`.
5. Add the **`automerge`** label if you want the PR to merge automatically once CI passes.
6. Address any review feedback.
7. CI green + label = automatic squash-merge. 🎉
