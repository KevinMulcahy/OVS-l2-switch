# Contributing

Thanks for wanting to contribute!

1. Fork the repo and make a feature branch: `git checkout -b feat/your-feature`
2. Write tests for new behavior.
3. Keep changes small and focused; one feature per PR.
4. Run the build & tests locally:
   - Rust dataplane: `cd dataplane-rs && cargo build && cargo test`
   - Go control plane: `cd control-plane-go && go build ./... && go test ./...`
5. Format code before submitting:
   - Rust: `cargo fmt`
   - Go: `gofmt` / `go fmt`
6. Open a PR, describe motivation and testing steps.

We use the following standards:
- Rust: `rustfmt`, `clippy`
- Go: `gofmt`, `go vet`, optional `golangci-lint`