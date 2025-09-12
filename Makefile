.PHONY: all build test fmt lint clean

all: build

build:
	@echo "Building dataplane (Rust)..."
	cd dataplane-rs && cargo build --release
	@echo "Building control plane (Go)..."
	cd control-plane-go && go build ./...

test:
	@echo "Testing dataplane (Rust)..."
	cd dataplane-rs && cargo test
	@echo "Testing control plane (Go)..."
	cd control-plane-go && go test ./...

fmt:
	@echo "Formatting Rust..."
	cd dataplane-rs && cargo fmt || true
	@echo "Formatting Go..."
	cd control-plane-go && go fmt ./... || true

lint:
	@echo "Linting Rust (clippy)..."
	cd dataplane-rs && cargo clippy -- -D warnings || true
	@echo "Linting Go (golangci-lint if installed)..."
	cd control-plane-go && golangci-lint run || true

clean:
	cd dataplane-rs && cargo clean
	cd control-plane-go && rm -f ./cmd/switchapi ./cmd/switchctl || true