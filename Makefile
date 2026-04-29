.PHONY: run test check fmt clippy wasm-build all

# Default target
all: check fmt clippy test

# Run the demo
run:
	cargo run -p igame-runtime -- assets/maps/demo

# Run all tests
test:
	cargo test --workspace

# Check compilation
check:
	cargo check --workspace

# Check formatting
fmt:
	cargo fmt --all -- --check

# Run clippy linter
clippy:
	cargo clippy --workspace -- -D warnings

# WASM build check for shared crate (no Bevy/system deps needed)
wasm-build:
	rustup target add wasm32-unknown-unknown
	cargo build -p igame-shared --target wasm32-unknown-unknown

# Fix formatting in place
fmt-fix:
	cargo fmt --all

# Fix clippy issues
clippy-fix:
	cargo clippy --workspace --fix
