.PHONY: all build check test clean fmt lint dev-server dev-agent dev-web docs

all: build

# ── Build ──────────────────────────────────────────────
build:
	cargo build --release

build-server:
	cargo build --release -p podmesh-server

build-agent:
	cargo build --release -p podmesh-agent

build-cli:
	cargo build --release -p podmesh-cli

# ── Development ────────────────────────────────────────
dev-server:
	cargo watch -x 'run -p podmesh-server'

dev-agent:
	cargo watch -x 'run -p podmesh-agent'

dev-web:
	cd web && npm run dev

# ── Quality ────────────────────────────────────────────
check:
	cargo check --workspace

test:
	cargo test --workspace

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets -- -D warnings

# ── Docs ───────────────────────────────────────────────
docs:
	cargo doc --workspace --no-deps --open

# ── Clean ──────────────────────────────────────────────
clean:
	cargo clean
	rm -rf web/.next web/node_modules

# ── Container ──────────────────────────────────────────
container-build:
	podman build -t podmesh-server -f Containerfile.server .
	podman build -t podmesh-agent -f Containerfile.agent .

# ── Install ────────────────────────────────────────────
install-cli:
	cargo install --path crates/podmesh-cli
