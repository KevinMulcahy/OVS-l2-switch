# DEPENDENCIES.md

Status: Draft — actionable rules and CI snippets to enforce them.
Applies to: All repositories and packages in this project (Dataplane: Rust; Control/Management: Go).
Referenced design docs: ARCHITECTURE.md and AIBD-RG (AI Based coding rules for Rust and Go development).
(Implementers: follow these rules; CI must enforce where possible.)

⸻

## 1. Purpose

This document defines module boundaries, dependency rules, and Continuous Integration (CI) enforcement for Rust and Go components in the project. The goal is to ensure reproducible builds, strong isolation between layers, predictable runtime behavior, secure dependency use, and clear rules for cross-language interfaces.

Key goals:
•	Keep the dataplane (Rust) memory-safe and high-perf with minimal runtime/unsafe surface.
•	Keep control and management planes (Go) idiomatic, small-surface, and well-versioned.
•	Make dependency updates auditable, testable, and CI-enforced.
•	Provide reproducible, secure builds and a clear path for upgrades.

⸻

## 2. Scope & High-level boundaries

Follow the project’s architecture separation:
•	Dataplane (Rust) — performance- and memory-safety-critical code: packet processing, forwarding, hardware offload integration.
•	Implemented as one or more Cargo workspaces with crates having clearly defined responsibilities (e.g., dp-core, dp-io, dp-proto, dp-tests).
•	Control Plane (Go) — control logic, orchestrator-facing APIs, configuration, and telemetry.
•	Implemented as separate Go modules, one per top-level service (e.g., control, management, cli, integration-adapters).
•	Shared / Integration contracts — protobufs, FlatBuffers, or well-documented C FFI boundary types and/or generated code (placed in a dedicated proto/ or api/ module).

Rules:
•	No cross-cutting circular dependencies between management, control, and dataplane. The architecture doc describes permitted edges (e.g., Control → Dataplane API, Management → Control).
•	Language boundaries are explicit; interactions cross them only through well-defined interfaces (gRPC/protobuf, C FFI with a tiny, well-tested shim, or IPC).

⸻

## 3. Rust module boundaries & rules

### 3.1 Workspace layout (recommended)

```
/rust-workspace
  Cargo.toml        # workspace members
  /crates
    /dp-core
    /dp-io
    /dp-proto        # generated protobufs or FFI bindings
    /dp-helpers
    /dp-tests
```

### 3.2 Crate responsibilities

```
•	dp-core: pure dataplane logic, no OS-specific runtime side-effects. Prefer zero allocations in hot paths.
•	dp-io: NIC, DPDK, AF_XDP, or OS I/O integration. Constrained unsafe usage here with careful review.
•	dp-proto: generated code for APIs shared with Go or external systems; acts as the contract.
•	dp-helpers: non-performance-critical utilities, feature-gated.
```

### 3.3 Dependency rules

**Pinning strategy (updated):**
Use semantic versioning constraints based on criticality:

```toml
[dependencies]
# Security-critical: exact pins for audit control
openssl = "=3.1.2"
ring = "=0.16.20"

# Performance-critical: allow patches only
tokio = "~1.32.0" 
bytes = "~1.4.0"

# Standard libraries: minor updates allowed
serde = "^1.0.188"
log = "^0.4.20"

# Development/test tools: more flexibility
criterion = "^0.5"
```

**Additional rules:**
•	Minimal dependencies in hot-path crates: dp-core should depend on as few external crates as possible; prefer no_std-compatible crates where relevant.
•	Isolate unsafe usage: unsafe blocks allowed only in dp-io or a named ffi crate; every unsafe block must include a structured comment describing invariants and be covered by tests. Use the format:
`rust // SAFETY: [Invariant] + [Why safe] + [Test coverage reference] // SAFETY: ptr is valid for reads of `len` bytes, guaranteed by caller contract. //         Verified in tests/ffi_safety.rs lines 42-67. unsafe { std::slice::from_raw_parts(ptr, len) } `
•	Feature gating: use Cargo features to gate optional dependencies (e.g., tokio / async-std, DPDK). Default feature sets must be small.
•	No transitive surprises: avoid depending on crates that bring unexpected large dependency trees unless justified and reviewed.
•	Approved crate list: maintain a short, reviewed list of accepted high-use crates (e.g., bytes, smoltcp if relevant). New crates require dependency approval process (see section 3.5).

### 3.4 Versioning & publishing

```
•	Crates follow semantic versioning. For internal-only crates, pin versions in the workspace and document breaking changes in CHANGELOGs.
•	Prefer workspace-local development for inter-crate changes; avoid publishing private crates unless needed.
```

### 3.5 New dependency approval process

Any PR adding a new Rust dependency must include:

**New Dependency Proposal (in PR description):**

```markdown
## New Dependency Proposal
- **Crate**: name and version
- **Justification**: why existing alternatives insufficient
- **Maintenance**: last commit, issue response time, contributor count
- **Security**: crates.io audit status, vulnerability history
- **License**: compatible license confirmation
- **Alternatives considered**: brief comparison
- **Impact**: estimated compile time/binary size impact
```

⸻

## 4. Go module boundaries & rules

### 4.1 Module layout (recommended)

```
/go
  /cmd
    /ctrl-svc
    /mgmt-svc
    /cli
  /pkg
    /api         # generated protobufs or shared types
    /controller
    /storage
  go.mod
```

### 4.2 Module responsibilities

```
•	cmd/* — small entrypoints wiring dependencies; business logic lives in pkg/.
•	pkg/api — generated contract code; keep it minimal (no business logic).
•	pkg/* — well-scoped packages with single responsibility.
```

### 4.3 Dependency rules

```
•	Use go modules (go.mod, go.sum). go.sum must be present and committed.
•	Minimal transitive dependencies: evaluate new dependencies for maintenance, security, and size.
•	Pin exact versions in go.mod for critical libraries; let go handle minor fixes but use replace only in development branches with caution.
•	No global state leakage: libraries must avoid package-level mutable state when possible.
•	Approved packages: maintain a short list of standard or already-vetted libraries (e.g., grpc, protobuf, cobra for CLI).
•	Avoid cgo in high-level packages; confine cgo to a narrow shim package if required for FFI.
```

### 4.4 New dependency approval process

Any PR adding a new Go dependency must include the same approval template as Rust (section 3.5), adapted for Go-specific considerations (go.mod impact, build time, etc.).

⸻

## 5. Cross-language / API contract rules

### 5.1 Contracts

**Protobuf / gRPC (enhanced versioning):**
Preferred for RPC and typed contract between Go and Rust. Keep .proto files in a single proto/ directory.

```proto
// Enhanced proto versioning format
syntax = "proto3";
option go_package = "github.com/yourorg/project/pkg/api/v1";
package yourorg.project.v1;

// Contract version must be bumped for breaking changes
message ContractVersion {
  uint32 major = 1;  // Breaking changes
  uint32 minor = 2;  // Backward-compatible additions  
  uint32 patch = 3;  // Bug fixes
}
```

Generate language bindings into appropriate modules (dp-proto for Rust; pkg/api for Go). The proto files are the source of truth.

**C ABI / FFI:** If FFI is required, create a small C-compatible shim layer with rigorous tests and an authoritative header file under api/ffi.h or similar.

**Contract compatibility matrix:** Track supported version ranges:

```markdown
| Rust dp-proto | Go pkg/api | Compatible |
|---------------|------------|------------|
| v1.0.x        | v1.0.x     | ✅         |
| v1.1.x        | v1.0.x     | ✅         |  
| v2.0.x        | v1.x.x     | ❌         |
```

### 5.2 Generated code

```
•	Generated code must be checked into the repo (or CI must be able to generate them deterministically). Prefer committing generated artifacts for reproducibility, unless CI generates and commits them in a deterministic gated job.
•	Generation commands must be recorded in README or Makefile.
```

⸻

## 6. Security & licensing rules

```
•	Dependency scanning: must run cargo audit and govulncheck/OSS vulnerability scanners in CI; PRs with vulnerabilities must be remediated or explicitly approved by a security reviewer.
•	License compatibility: Dependencies must have compatible licenses; avoid viral licenses unless approved. CI should enforce license checks (e.g., cargo-license and an equivalent for Go).
•	Supply chain hygiene: pin transitive dependencies where practical; prefer well-maintained libraries with many users.
```

⸻

## 7. Reproducible builds & lockfiles

**Rust**
•	Always commit Cargo.lock for applications; for libraries commit only if the crate is an application. For workspace apps, commit the workspace Cargo.lock.
•	Use cargo generate-lockfile or cargo update -p as part of update flows.
•	CI must restore Rust toolchain via rustup using explicit toolchain version (e.g., 1.74.0 or stable locked to a specific date) recorded in rust-toolchain.toml.

**Go**
•	Commit go.mod and go.sum. CI should run go mod tidy and fail the job if go.sum or go.mod would change.
•	Use GOMODCACHE caching in CI for performance; use deterministic go version (pin in CI).

⸻

## 8. CI enforcement: checks & automation

Every PR must run the following automated checks. Failures block merges until addressed.

### 8.1 Common checks (both languages)

```
•	Dependency change detection: block PRs that add a dependency without an accompanying justification (PR template should capture rationale).
•	Vulnerability scanning:
•	Rust: cargo audit (fail on High/Critical).
•	Go: govulncheck or Snyk/OSS scanning (fail on High/Critical).
•	License scanning:
•	Rust: cargo-license.
•	Go: go-licenses or CI script.
•	SBOM generation: produce an SBOM (CycloneDX or SPDX) for each artifact and attach to the CI job artifacts.
```

### 8.2 Rust-specific CI steps

```
•	cargo fmt -- --check
•	cargo clippy -- -D warnings (with allowed exceptions for legacy crates via per-crate allow documented).
•	cargo test --workspace --all-features
•	cargo audit (vulnerabilities)
•	cargo deny check (optional advanced policy enforcement)
•	Confirm Cargo.lock presence for apps and that it is not modified by build steps.
```

### 8.3 Go-specific CI steps

```
•	gofmt -l (fail if changes needed)
•	golangci-lint run (fail on configured severity)
•	go vet ./...
•	go test ./... with coverage threshold enforced for non-trivial packages
•	go mod tidy check (CI should fail if go.mod or go.sum would change after running)
•	govulncheck or equivalent scan
```

### 8.4 Cross-language & contract checks

```
•	Proto/lint/generate: CI job must regenerate language bindings from .proto and fail if generated artifacts checked into the repo differ from regenerated outputs (ensures committed generated files are in sync).
•	API contract tests: include at least one integration smoke test that validates compatibility across languages (e.g., a control-plane Go test calling a dataplane Rust process via gRPC or a lightweight unit test that imports generated bindings).
•	Contract compatibility validation: Check that API version changes follow the compatibility matrix rules.
```

### 8.5 PR gating and automation

```
•	For dependency updates:
•	Dependabot-like automated PRs must include test results, SBOM, cargo audit/govulncheck outputs.
•	A human reviewer must approve dependency changes above a certain risk level (security fixes and major version bumps).
•	Auto-merge policy: non-breaking minor/patch updates may be auto-merged only if they pass all checks and an automatic review policy is in place.
•	Manual approvals: major bumps, new native libs, or cgo/cbindgen/unsafe additions require a second reviewer and security reviewer signoff.
```

⸻

## 9. Developer workflow & commands (examples)

Include these commands in developer docs / Makefile / scripts for consistent use.

**Rust**

```bash
# Format, lint, test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace --all-features

# Audit
cargo install cargo-audit || true
cargo audit

# Deny (optional)
cargo install cargo-deny || true
cargo deny check
```

**Go**

```bash
# Format/lint/test
gofmt -l .
golangci-lint run
go vet ./...
go test ./... -cover

# Dependency hygiene
go mod tidy
# check if tidy changed go.mod / go.sum in CI
```

**Regenerate protobufs (example)**

```bash
# from repo root
protoc --go_out=./go/pkg/api --go_opt=paths=source_relative \
       --rust_out=../rust-workspace/crates/dp-proto src/proto/*.proto
# or use the project's makefile target
make proto
```

⸻

## 10. Handling unsafe / native code additions

```
•	Any PR introducing unsafe Rust code, cgo, or native dependencies must:
1.	Include a short design note in the PR describing why native/unsafe code is required.
2.	Include tests demonstrating invariant correctness.
3.	Include a security review tag and at least one additional approver.
4.	Provide a documented mitigation or rollback plan.
5.	Follow the structured SAFETY comment format specified in section 3.3.
```

⸻

## 11. Version upgrade policy

**Enhanced coordination for breaking changes:**
•	Patch updates (x.y.Z): allowed with CI green; auto-merge permitted if low risk.
•	Minor updates (x.Y.z): allowed with passing full CI and automated compatibility checks.
•	Major updates (X.y.z): require design review, an upgrade plan, and coordination across affected services. Update PR must include migration steps and compatibility tests.
•	Cross-language breaking changes: Must include coordinated release plan with rollback strategy. API contract version must be bumped with backward compatibility period defined.

⸻

## 12. Exceptions & emergency process

```
•	Emergency security patches: may bypass some normal gates but require immediate follow-up PRs to harden the change (e.g., revert any temporary relaxations), and must be communicated to project stakeholders.
•	Temporary allow-list: maintain a documented list of temporary exceptions (with expiry and owner). CI should track exceptions and warn when they expire.
```

⸻

## 13. Documentation & traceability

```
•	Every dependency addition or major change must include:
•	Rationale in the PR description (using the template from sections 3.5/4.4).
•	Link to relevant security/maintenance evidence (e.g., maturity, popularity, last commit).
•	Evidence of license compatibility.
•	Keep a simple DEPENDENCY_LOG.md or use PR labels to track accepted dependency additions and their owners.
```

⸻

## 14. Example GitHub Actions snippets

**Rust CI (enhanced with caching)**

```yaml
name: Rust CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          
      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true
          
      - name: Install tools
        run: |
          cargo install cargo-audit || true
          cargo install cargo-deny || true
          
      - name: Cargo fmt
        run: cargo fmt --all -- --check
        
      - name: Clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
        
      - name: Test
        run: cargo test --workspace --all-features
        
      - name: Audit
        run: cargo audit
```

**Go CI (enhanced with caching)**

```yaml
name: Go CI
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Go
        uses: actions/setup-go@v4
        with:
          go-version: '1.21'
          cache: true
          
      - name: Install linter
        run: go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest
        
      - name: Format check
        run: |
          if [ "$(gofmt -l . | wc -l)" -gt 0 ]; then
            echo "Files need formatting:"
            gofmt -l .
            exit 1
          fi
          
      - name: Lint
        run: golangci-lint run
        
      - name: Test
        run: go test ./... -cover
        
      - name: Tidy check
        run: |
          go mod tidy
          git diff --exit-code go.mod go.sum
```

**Contract validation CI**

```yaml
name: Contract Validation
on: [push, pull_request]
jobs:
  proto-sync:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install protoc
        uses: arduino/setup-protoc@v2
        
      - name: Generate bindings
        run: make proto
        
      - name: Check generated files are in sync
        run: |
          if ! git diff --exit-code; then
            echo "Generated files are out of sync. Run 'make proto' and commit."
            exit 1
          fi
          
      - name: Contract compatibility test
        run: |
          # Run cross-language integration smoke test
          go test ./tests/integration -tags=contract_test
```

⸻

## 15. Checklist for maintainers (pre-merge)

```
•	go.mod / go.sum and Cargo.lock are present and unchanged (unless change is intended).
•	cargo audit / govulncheck results acceptable.
•	Lint and formatting pass.
•	Generated bindings (proto) are in sync with committed generated files.
•	PR includes rationale for new dependency, license checks, and security assessment (using template).
•	For unsafe/native changes: tests, structured SAFETY comment, and security reviewer signoff.
•	Contract version compatibility verified if API changes included.
```

⸻

## 16. Implementation roadmap & ownership

**Rollout phases:**

1. **Phase 1 (Week 1-2)**: Basic CI checks (format, lint, test) + caching
1. **Phase 2 (Week 3-4)**: Security scanning (audit, license checks) + dependency approval templates
1. **Phase 3 (Week 5-6)**: Cross-language contract validation + compatibility matrix
1. **Phase 4 (Week 7-8)**: Advanced policies (SBOM generation, enhanced unsafe code rules)

**Ownership:**
•	Assign a dependency steward or security owner for Rust and Go (names or teams) who triage dependency PRs and security alerts.
•	Automations: Enable dependabot (or equivalent) configured to open PRs for dependency updates. Configure auto-labeling for dependency PRs so reviewers are auto-notified.
•	Periodic review: Quarterly dependency review meeting to audit and archive unused dependencies and update the approved-list.
•	Metrics/monitoring: Track dependency health metrics (age, vulnerability count, update frequency) via dashboard.

⸻

## 17. Appendix: Rationale highlights

```
•	Pinning lockfiles for applications ensures reproducible builds and helps with security audits.
•	Small, well-defined crate/module boundaries reduce cognitive load, ease code review, and limit blast radius of breaking changes.
•	Enforcing unsafe confinement to a few modules improves reviewability and reduces risk.
•	CI enforcement is the single biggest lever to keep dependencies healthy—automate what you can and require human review for the highest-risk changes.
•	Structured dependency approval processes prevent technical debt accumulation while maintaining development velocity.
```
