format:
	cargo fmt --quiet

format-check:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	@cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

build-release:
	@echo "Building release binary for $(shell uname -s)..."
	cargo build --release

run:
	@echo "Running..."
	cargo run -- dedupe

all: format lint test build-release run
