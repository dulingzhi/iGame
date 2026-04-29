# Development Guide

## Local Development Commands

### Format

```bash
cargo fmt --all
```

### Lint (Clippy)

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

### Test

```bash
cargo test --workspace
```

### Run (desktop)

```bash
cargo run -p igame-runtime
```

### Build for WebAssembly

```bash
rustup target add wasm32-unknown-unknown
cargo build --workspace --target wasm32-unknown-unknown
```

---

## CI Checks

Every push and pull request automatically runs the following checks:

| Check | Command |
|---|---|
| Format | `cargo fmt --all -- --check` |
| Clippy (deny warnings) | `cargo clippy --workspace --all-targets -- -D warnings` |
| Tests | `cargo test --workspace` |
| WASM build | `cargo build --workspace --target wasm32-unknown-unknown` |

All checks must pass before a PR can be merged.

---

## Auto-merge

PRs that meet **all** of the following conditions are automatically merged using the **rebase** strategy once all required CI checks pass:

1. The PR is **not a draft**.
2. The PR originates from **this repository** (forks are excluded).
3. The PR has the **`automerge`** label.

### How to trigger auto-merge

Add the `automerge` label to your PR (via the GitHub UI or `gh` CLI):

```bash
gh pr edit <PR-number> --add-label automerge
```

Once the label is applied and all CI checks turn green, the PR will be rebased onto `main` and merged automatically.

---

## Recommended Branch Protection Settings (main)

Configure these under **Settings → Branches → Branch protection rules → main**:

| Setting | Value |
|---|---|
| Require a pull request before merging | ✅ enabled |
| Required status checks | `fmt / clippy / test`, `wasm32 build` |
| Require branches to be up to date before merging | ✅ enabled |
| Require conversation resolution before merging | optional |
| Require approvals | **0** (no manual reviews required) |
| Allow rebase merging | ✅ enabled |
| Allow squash merging | optional |
| Allow merge commits | optional |
| Allow auto-merge | ✅ **must be enabled** (Settings → General) |
| Automatically delete head branches | ✅ recommended |

> **Important:** GitHub's native auto-merge feature must be enabled in **Settings → General → Allow auto-merge** for the auto-merge workflow to work.
