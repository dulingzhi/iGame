# iGame

[![CI](https://github.com/dulingzhi/iGame/actions/workflows/ci.yml/badge.svg)](https://github.com/dulingzhi/iGame/actions/workflows/ci.yml)

A Bevy-based UGC game editor and runtime.

## Development

### Prerequisites

- Rust stable toolchain
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

### Quick-start

```bash
# Run all tests
cargo test --workspace

# Check formatting and lints
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Build for WebAssembly
cargo build --target wasm32-unknown-unknown --workspace
```

## CI & Auto-merge

Every pull request automatically goes through four checks (fmt, clippy, tests,
wasm build). When all checks pass, **auto-merge is enabled automatically** — no
label required.

To opt out of auto-merge:
- Mark the PR as a **draft**, or
- Add the **`do-not-merge`** label.

See [docs/CI_AND_AUTOMERGE.md](docs/CI_AND_AUTOMERGE.md) for the full policy
and the one-time repository settings required.
