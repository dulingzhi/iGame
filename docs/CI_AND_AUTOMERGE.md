# CI & Auto-merge Policy

This document describes the automated CI pipeline and auto-merge behaviour for pull
requests in this repository.

---

## CI Checks

Every push to `main` and every pull request triggers the following checks
(defined in [`.github/workflows/ci.yml`](../.github/workflows/ci.yml)):

| Job | Command | Purpose |
|-----|---------|---------|
| **Format** | `cargo fmt --all -- --check` | Enforces consistent code style |
| **Clippy** | `cargo clippy --all-targets --all-features -- -D warnings` | Static analysis, zero-warning policy |
| **Test** | `cargo test --workspace` | Runs all workspace unit & integration tests |
| **WASM Build** | `cargo build --target wasm32-unknown-unknown --workspace` | Ensures every crate compiles for WebAssembly |

All four checks must pass before a PR can be merged.

---

## Auto-merge

Pull requests that meet all of the criteria below will have **GitHub native
auto-merge** enabled automatically (rebase strategy, no extra commit).  
Once every required status check turns green the PR is rebased and merged
into `main` without any further human action.

### Eligibility criteria (all must be true)

| Condition | Details |
|-----------|---------|
| **Not a draft** | Convert to "Ready for review" to re-enable. |
| **Same-repository branch** | Fork PRs are excluded for security reasons. |
| **Title does not contain `WIP`** | Case-insensitive whole-word match. Rename the PR when it's ready. |
| **No `do-not-merge` label** | Remove the label to re-enable auto-merge. |

### How to opt out

You have two ways to prevent a PR from being merged automatically:

1. **Mark the PR as a draft** – auto-merge is disabled while the PR is in
   draft state. Click "Ready for review" when you want it to proceed.
2. **Add the `do-not-merge` label** – removes the scheduled merge immediately.
   Remove the label whenever you are ready.

> Removing an opt-out (converting from draft, removing the label) re-triggers
> the workflow and re-enables auto-merge if all other criteria are met.

---

## Required repository settings (one-time setup)

For the automation to work end-to-end, a repository administrator must enable
the following settings **once**:

### 1 · Allow auto-merge

`Settings → General → Pull Requests → ☑ Allow auto-merge`

### 2 · Branch protection for `main`

`Settings → Branches → Add rule → Branch name pattern: main`

Recommended settings:

- ☑ **Require a pull request before merging**
- ☑ **Require status checks to pass before merging**
  - Add the following required checks:
    - `Format`
    - `Clippy`
    - `Test`
    - `WASM Build`
- ☑ **Require branches to be up to date before merging**
- ☑ **Do not allow bypassing the above settings** (optional but recommended)

### 3 · Allow rebase merging

`Settings → General → Pull Requests → ☑ Allow rebase merging`

You may disable *merge commits* and *squash merging* if you want to enforce a
linear history exclusively through rebasing.

---

## Local development quick-reference

```bash
# Format code
cargo fmt --all

# Lint (must be zero warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test --workspace

# Check wasm build
cargo build --target wasm32-unknown-unknown --workspace
```
