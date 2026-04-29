# iGame

[![CI](https://github.com/dulingzhi/iGame/actions/workflows/ci.yml/badge.svg)](https://github.com/dulingzhi/iGame/actions/workflows/ci.yml)

A **data-driven UGC game platform** built with Rust and Bevy — think *Warcraft III World Editor* for the modern era: desktop + web, 2D/3D, ECS-powered, with a visual trigger/scripting system that lets players build their own maps and games.

---

## Quick start

```bash
# Clone
git clone https://github.com/dulingzhi/iGame.git
cd iGame

# Run all workspace tests
cargo test --workspace

# Run Clippy (same flags as CI)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check

# Build wasm32-compatible crates
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown -p igame-shared -p igame-runtime
```

## Documentation

| Document | Description |
|----------|-------------|
| [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) | Setup, local commands, CI, auto-merge, branch protection |
| [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) | Architecture, build targets, testing strategy |
| [ROADMAP.md](ROADMAP.md) | Full milestone plan (M0 → 1.0) |

## CI & Auto-merge

Every PR runs four checks automatically: **Rustfmt**, **Clippy**, **Tests**, and a **wasm32 build**.

To have a PR merged automatically once all checks are green, add the **`automerge`** label.  
See [docs/CONTRIBUTING.md – Auto-merge](docs/CONTRIBUTING.md#auto-merge-with-the-automerge-label) for full details.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
