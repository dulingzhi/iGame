# iGame

A data-driven (ECS) cross-platform game engine and UGC editor built with Rust/Bevy, targeting Desktop and Web (WASM).

## Quick Start

```bash
# Run tests
cargo test --workspace

# Format code
cargo fmt --all

# Lint
cargo clippy --workspace --all-targets -- -D warnings

# Build for WebAssembly
cargo build --workspace --target wasm32-unknown-unknown
```

## Documentation

- [Development Guide](docs/DEVELOPMENT.md) — local dev commands, CI checks, auto-merge, branch protection
