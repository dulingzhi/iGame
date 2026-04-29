# Development Guide

## Overview

This document describes the development workflow, CI pipeline, and auto-merge policy for **iGame**.

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) (for WASM builds)
- `rustfmt` and `clippy` components:

```sh
rustup component add rustfmt clippy
rustup target add wasm32-unknown-unknown
```

### Build

```sh
cargo build
```

### Run Tests

```sh
cargo test --all-features
```

### Lint

```sh
# Format check
cargo fmt --all -- --check

# Static analysis (warnings treated as errors)
cargo clippy --all-targets --all-features
```

### WASM Build

```sh
cargo build --target wasm32-unknown-unknown --all-features
```

---

## CI Pipeline

Every PR and push to `main` runs the following mandatory checks via GitHub Actions (`.github/workflows/ci.yml`):

| Job | Command |
|-----|---------|
| **Rustfmt** | `cargo fmt --all -- --check` |
| **Clippy** | `cargo clippy --all-targets --all-features` (warnings = errors) |
| **Tests** | `cargo test --all-features` |
| **WASM Build** | `cargo build --target wasm32-unknown-unknown --all-features` |

All four checks must pass before a PR can be merged.

---

## Auto-merge Policy (Safety Mode A)

### How it works

A GitHub Actions workflow (`.github/workflows/auto-merge.yml`) automatically enables **GitHub native auto-merge (rebase strategy)** on eligible PRs. Once enabled, the PR merges into `main` as soon as all required status checks pass.

**No manual label is required.** The workflow evaluates eligibility automatically when a PR is opened, updated, or changed from draft to ready.

### Eligibility criteria (all must be met)

| Condition | Details |
|-----------|---------|
| **Allowlisted author** | PR author must be one of: `dulingzhi`, `github-actions[bot]`, or `copilot-swe-agent[bot]` |
| **Same repository** | PR must come from a branch in this repo (forks are excluded) |
| **Not a draft** | PR must be marked *Ready for review* |
| **No WIP in title** | PR title must not contain `WIP` (case-insensitive) |
| **No block label** | PR must not have the `do-not-merge` label |
| **CI passing** | All required status checks must pass (enforced by branch protection) |

### Opting out of auto-merge

Use any of these methods to prevent a PR from being auto-merged:

- Mark the PR as **Draft**
- Add the label **`do-not-merge`**
- Include **`WIP`** in the PR title (e.g. `WIP: refactor engine loop`)

### Allowlist policy

Only PRs authored by users in the hardcoded allowlist are eligible for auto-merge. This prevents accidental merging of external contributions without review. The current allowlist is:

- `dulingzhi` — repository owner
- `github-actions[bot]` — GitHub automation
- `copilot-swe-agent[bot]` — Copilot coding agent

To add more users, update the `ALLOWLIST` array in `.github/workflows/auto-merge.yml`.

---

## Required Repository Settings

To make the auto-merge workflow function correctly, configure the following in **Settings** (one-time setup):

### 1. Allow auto-merge
**Settings → General → Pull Requests → Allow auto-merge** ✅

### 2. Allow rebase merging
**Settings → General → Pull Requests → Allow rebase merging** ✅  
(Optionally disable *Merge commits* and *Squash merging* to enforce rebase-only.)

### 3. Branch protection for `main`
**Settings → Branches → Add rule for `main`:**

- ✅ Require a pull request before merging
- ✅ Require status checks to pass before merging
  - Add the following required checks:
    - `Rustfmt`
    - `Clippy`
    - `Tests`
    - `WASM Build`
- ✅ Require branches to be up to date before merging
- ✅ Do not allow bypassing the above settings

### 4. Actions permissions
**Settings → Actions → General → Workflow permissions:**  
Set to **Read and write permissions** to allow the auto-merge workflow to call the GitHub API.

---

## Branch Naming Convention

| Prefix | Purpose |
|--------|---------|
| `feature/` | New features |
| `fix/` | Bug fixes |
| `docs/` | Documentation only |
| `chore/` | Maintenance / tooling |
| `copilot/` | Automated Copilot agent branches |

---

## Commit Message Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(scope): <short description>

[optional body]
```

Examples:
- `feat(runtime): add map package loader`
- `fix(editor): correct egui panel sizing`
- `docs: update development guide`
