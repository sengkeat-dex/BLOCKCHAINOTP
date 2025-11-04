.PHONY: test backend frontend-build frontend-check frontend-serve dev

# Run full Rust test suite (backend + integration)
test:
	cargo test

backend:
	cargo clippy --all-targets --all-features

# Build the Yew frontend to WebAssembly; falls back to cargo build if wasm-pack is missing
frontend-build:
	@command -v wasm-pack >/dev/null 2>&1 \
		&& wasm-pack build frontend --target web --out-dir dist --out-name otp_frontend \
		|| cargo build -p otp-frontend --target wasm32-unknown-unknown

# Quickly check the frontend by running trunk if available
frontend-serve:
	@command -v trunk >/dev/null 2>&1 \
		&& cd frontend && trunk serve --open \
		|| echo "Install trunk (https://trunkrs.dev) to use frontend-serve"

dev:
	@command -v trunk >/dev/null 2>&1 || { echo "trunk not found; install from https://trunkrs.dev"; exit 1; }
	@trap 'kill 0' EXIT; \
		cargo run & \
		cd frontend && trunk serve --open
