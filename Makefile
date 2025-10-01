# Makefile for OVS-L2-Switch project
# Convenience targets for Rust dataplane and Go control plane

.PHONY: all build test clean fmt lint security

# --- Global targets ---
all: build
build: build-rust build-go
test: test-rust test-go
fmt: fmt-rust fmt-go
lint: lint-rust lint-go
security: security-rust security-go
clean: clean-rust clean-go

# --- Rust (Dataplane) ---
build-rust:
	cargo build --workspace --all-features

test-rust:
	cargo test --workspace --all-features --verbose

fmt-rust:
	cargo fmt --all

lint-rust:
	cargo clippy --all-targets --all-features -- -D warnings

security-rust:
	cargo audit || true
	cargo deny check || true

clean-rust:
	cargo clean

# --- Go (Control Plane / Management Plane) ---
build-go:
	go build ./...

test-go:
	go test -race -coverprofile=coverage.out ./...

fmt-go:
	go fmt ./...

lint-go:
	golangci-lint run ./...

security-go:
	gosec ./...
	govulncheck ./...

clean-go:
	go clean -testcache
