# TEST_STRATEGY.md

## Purpose
This document defines the testing strategy for the project: what to test, how to test it, which tools to use, CI enforcement rules, and the PR checklist needed to merge code. The goal is predictable quality for a mixed-language (Rust dataplane, Go control/management) system where correctness, performance, and memory safety are critical.

---

## Principles
- **Fail fast in CI**: run linting, format checks, tests and security/fuzz steps before merging.
- **Testing pyramid**: unit tests (fast, many) → integration tests (moderate) → end-to-end (slow, few) → performance & security (specialized).
- **Isolation by module boundary**: each module (Rust crate / Go module) should have its own unit tests and mocks for external dependencies.
- **Reproducible CI**: tests must run deterministically in the CI environment (use containers / pinned toolchains).
- **Automate and measure**: coverage, flaky test detection, and test duration monitoring are part of CI.
- **Security-first for parsers/net code**: prioritize fuzzing and sanitizers for any code that parses or processes network input.

---

## Scope
- **Dataplane (Rust)** — unit tests, property-based tests, fuzzing, integration tests that exercise packet processing paths, and performance benchmarks.
- **Control & Management Planes (Go)** — unit tests, integration tests, e2e tests for API/CLI/agent interactions, and basic performance checks.
- **Cross-plane e2e** — tests exercising the control-plane → dataplane interaction (control messages, flow programming, config push).
- **External integrations** — tests for adapters (Netlink, OVSDB, systemd, etc.) run with mocks where possible, and with live integration in CI job matrix or nightly runs where appropriate.
- **CI & Release** — tests executed on each PR and on scheduled/nightly pipelines for long-running jobs (fuzzing, large perf runs).

---

## Test types & expectations

### Unit tests
- **Who**: module owners and contributors.
- **Where**:
  - Rust: `cargo test --workspace` (per-crate `tests`, `unit` modules).
  - Go: `go test ./...` with module-specific packages.
- **Tools**:
  - Rust: `cargo test`, `cargo test --all-features`, `cargo clippy`, `cargo fmt -- --check`.
  - Go: `go test`, `go vet`, `gofmt -s -l`, `golangci-lint`.
- **Target**: fast, isolated, deterministic. Use mocks for external dependencies.
- **Enforcement**: must pass in PR CI; flaky tests should be fixed or quarantined.

### Integration tests
- **What**: cross-module interactions, persistence, inter-process communication, plugin interfaces.
- **How**:
  - Rust: `#[cfg(feature="integration")]` test crates or `tests/` integration folder; run with `cargo test --package <pkg> --test <name>`.
  - Go: package-level integration tests using Docker or in-memory mocks.
- **Execution**: run in CI as part of a longer job (may be split as separate job).
- **Data**: use fixtures stored under `test/fixtures` or generated fixtures; explicit seed values where randomized inputs are used.

### End-to-end (E2E)
- **What**: full stack flows (e.g., management → control → dataplane forwarding behaviour).
- **How**: docker-compose or container-based scenarios that start the real binaries (or near-real builds), and exercise APIs and forwarding using test traffic generators (e.g., `pktgen`, `scapy`, or test clients).
- **Where to run**: CI (if quick), or nightly pipeline for longer runs.

### Performance & Benchmarks
- **Dataplane (Rust)**:
  - microbenchmarks (`cargo bench` or `criterion`) for packet-path hot spots.
  - system-level performance (throughput/latency) in dedicated perf jobs using containers/VMs.
- **Control/Management (Go)**:
  - benchmark key APIs with `go test -bench`.
- **Enforcement**:
  - baseline numbers recorded; regression detection on PRs (optional, implement for hotspots that are performance-critical).

### Security & Fuzzing
- **Targets**: any parser, packet processing logic, external inputs, RPC handlers.
- **Rust tools**:
  - `cargo-fuzz` (libFuzzer) for fuzzing crates that parse or handle external packets.
  - `MIRI` and sanitizer builds where applicable (address/undefined behavior).
  - property-based testing with `proptest` for invariants.
- **Go tools**:
  - `go-fuzz` or use `go test -fuzz` feature (if available in toolchain).
  - property tests via `gotest.tools` or `quick.Check`.
- **CI policy**:
  - run quick smoke fuzz targets in PRs (short runs).
  - nightly/weekly dedicated job runs long-running fuzzers reporting findings.
- **Responsiveness**: crashes/unique hangs must open issues automatically with reproducer (CI artifact).

### Regression & Mutation Testing
- **Code coverage**: track via coverage tooling (e.g., `grcov`/codecov for Rust, `go test -coverprofile` for Go). Aim for meaningful thresholds (see thresholds section), but do not allow gaming.
- **Mutation testing** (optional/periodic): use mutation tools or services if helpful for critical modules.

---

## Tooling (recommended)
- **Rust**:
  - `rustup` (pinned toolchain), `cargo`, `cargo clippy`, `cargo fmt`, `cargo test`, `cargo bench`, `cargo-fuzz`, `proptest`, `criterion`.
- **Go**:
  - pinned `go` toolchain (go1.x), `go test`, `go vet`, `gofmt`, `golangci-lint`, `gotestsum`.
- **CI**:
  - GitHub Actions with matrix builds.
- **Test harness**:
  - Dockerfiles for integration/e2e jobs, `docker-compose` for local e2e scenarios.
- **Reporting**:
  - Codecov / GitHub coverage reports, unit test summaries, JUnit XML artifacts.
- **Flaky test detection**:
  - rerun failing tests up to N times in CI for transient failures but mark and quarantine if persistent.

---

## CI enforcement and example jobs

### Standard PR checks (fast)
- `format-check`:
  - Rust: `cargo fmt --all -- --check`
  - Go: `gofmt -s -l .`
- `lint`:
  - Rust: `cargo clippy --all-targets -- -D warnings`
  - Go: `golangci-lint run`
- `unit-tests`:
  - Rust: `cargo test --workspace --locked --all-features --tests -- --nocapture`
  - Go: `go test ./... -coverprofile=coverage.out`
- `build`:
  - ensure `cargo build --release` and `go build ./...` succeed

### Extended checks (merged branch or nightly)
- `integration-tests`:
  - Spin docker-compose; run integration suites.
- `fuzz-smoke` (PR): run a 30–120s fuzz run on critical targets; fail on crashes.
- `fuzz-nightly`: long-running fuzz circuits with crash triage artifacts.
- `perf-nightly`: run performance harness and store results.

---

## Coverage & thresholds
- **Baseline guidance**:
  - Unit tests: aim for ≥ 80% on critical modules (dataplane & protocol parsers).
  - Integration + E2E combined: coverage will be lower; ensure high-risk code paths are tested elsewhere.
- **Enforcement**:
  - CI should fail PRs that reduce coverage for critical modules beyond a small delta (e.g., >1–2% regression).
  - For new code, tests must be added to maintain or improve coverage.

---

## Handling flaky tests
- If a test flakes, DO NOT suppress it permanently.
- Steps:
  1. Reproduce locally using same CI environment.
  2. If intermittent, mark as `#[ignore]` (Rust) or skip in Go with build tags and open a follow-up issue to fix.
  3. Add automatic rerun logic in CI but require a fix if instability persists.
- Track flaky tests in a `TESTS-QUARANTINE.md` with owner and expected remediation timeframe.

---

## PR checklist (enforced by CI + review)
- [ ] All formatting checks pass.
- [ ] Lint checks pass.
- [ ] Unit tests pass locally and in CI.
- [ ] Integration tests pass if changed code touches integration boundaries.
- [ ] Added/updated tests for new functionality or bug fixes.
- [ ] Coverage for changed modules did not regress beyond allowed delta.
- [ ] Any new parser/input code has a fuzz target (or documented reason why not).
- [ ] Performance-critical changes include benchmark results or note to run perf job.
- [ ] Tests have owners or author assigned to remediate potential flakiness.
- [ ] CHANGELOG/PR description documents test additions or notable test-related changes.

---

## Onboarding & local workflow
- **Rust**:
  ```bash
  rustup install <pinned-version>
  cargo fmt --all
  cargo clippy --all-targets -- -D warnings
  cargo test
  ```
- **Go**:
  ```bash
  gofmt -s -w .
  golangci-lint run
  go test ./...
  ```
- Provide local `docker-compose.test.yml` for e2e integration.

---

## Appendix: Recommended commands

**Rust**
```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --workspace --locked --all-features --tests
cargo fuzz run packet_parser_fuzz_target -- -max_total_time=60
```

**Go**
```bash
gofmt -s -l .
golangci-lint run ./...
go test ./... -v -coverprofile=coverage.out
go test -bench . ./pkg/somepkg -benchmem
```
