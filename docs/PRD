# Phase 1 PRD — Foundations

## Objective
Establish the foundational components, architecture, and workflows for the open-source Layer 2 Switching Platform.  
Phase 1 delivers a minimal, functional baseline with clean build pipelines, CI/CD automation, and initial packet-forwarding capability.

---

## Business Context
This phase lays the technical and procedural groundwork for all subsequent development.  
Its goals are reliability, reproducibility, and adherence to the AIBD-RG coding, documentation, and security standards.  
Phase 1 must result in a system that compiles, runs, and tests cleanly with complete automation and without manual intervention.

---

## User Stories
- As a **developer**, I can clone the repo and build all components on a supported Linux host using documented steps.  
- As a **maintainer**, I can run automated CI/CD pipelines that lint, test, and audit code with no manual fixes required.  
- As a **network engineer**, I can verify that a packet entering one interface exits another according to basic forwarding logic.

---

## Requirements

### Functional Requirements
1. Implement a working **dataplane** service in Rust that initializes and performs simple packet forwarding using the Linux network stack.  
2. Implement a **control plane** stub in Go with an HTTP health-check endpoint.  
3. Define foundational **CI/CD** pipelines for linting, testing, security scanning, and formatting.  
4. Provide **Docker Compose** configuration for multi-service local testing (dataplane + controlplane + integration tests).  
5. Ensure all code passes **clippy/gofmt/golangci-lint** and **cargo-audit** security checks.

### Non-Functional Requirements
| Quality | Requirement |
|----------|-------------|
| **Performance** | Basic forwarding latency < 5 ms per frame on standard Linux vSwitch path |
| **Security** | No unsafe Rust code; all dependencies pass `cargo audit`; Go HTTP handlers follow secure defaults |
| **Reliability** | CI pipelines must complete successfully on every PR |
| **Scalability** | Architecture must support modular addition of MAC learning, VLAN, and API features in later phases |
| **Maintainability** | Code must conform to AIBD-RG standards for structure, one-declaration-per-file, and documentation |

---

## Acceptance Criteria
- [x] Repository builds cleanly via `make build` or `docker compose build`.  
- [x] `cargo fmt`, `clippy`, `cargo audit`, `go fmt`, and `golangci-lint` pass with 0 warnings.  
- [x] CI pipeline executes `fast-checks`, `integration-tests`, and `fuzz-smoke` successfully.  
- [x] Dataplane container outputs *“Dataplane service started successfully”* and healthcheck returns 200 OK.  
- [x] Control plane container logs *“switchapi: listening on :8080”*.  
- [x] All documentation (`README.md`, `INSTALL.md`, `ROADMAP.md`, `phase1_prd.md`) committed under `docs/`.  
- [x] `cargo audit` reports **0 vulnerabilities**.  

---

## Out of Scope
- MAC table learning, VLAN tagging, and STP logic (Phase 2).  
- gRPC/gRPC-Gateway API integration (Phase 3).  
- GUI/CLI interfaces (Phase 4).  
- DPDK/eBPF acceleration (Phase 5).

---

## Dependencies
| Type | Description |
|------|--------------|
| **Internal** | None; this phase establishes initial modules. |
| **External** | Rust toolchain 1.79+, Go 1.21+, Docker 24+, Linux kernel ≥ 5.10. |
| **Infrastructure** | GitHub Actions CI with caching; Docker Compose network `switchnet`. |

---

## Data Model
Phase 1 does not yet define persistent data structures.  
Temporary placeholders:
```rust
// dataplane_rs/src/types.rs (Phase 1 baseline)
pub struct PacketFrame {
    pub src_mac: [u8;6],
    pub dst_mac: [u8;6],
    pub eth_type: u16,
    pub payload: Vec<u8>,
}
````

This struct will evolve into a fully managed `Frame` type with VLAN and STP fields in Phase 2.

---

## Security Considerations

* **Rust**: `unsafe_code = "forbid"` enforced in `Cargo.toml`.
* **Go**: Secure HTTP server with explicit read/write timeouts.
* **CI**: Automated `cargo audit` and `cargo deny` checks.
* **Containers**: Non-root user execution; healthcheck endpoint limited to localhost in CI.

---

## Testing Strategy

| Level                 | Focus                                                        | Framework                         |
| --------------------- | ------------------------------------------------------------ | --------------------------------- |
| **Unit Tests**        | Dataplane forwarding logic                                   | Rust `cargo test --lib`           |
| **Integration Tests** | End-to-end control plane ↔ dataplane health and network init | Docker Compose tests              |
| **Fuzz Tests**        | Ethernet frame parsing                                       | `cargo fuzz`                      |
| **Security Tests**    | Dependency vulnerability audit                               | `cargo audit`, `gosec` (Phase 3+) |

All tests must execute in CI with `--nocapture` and use `test-threads=$(nproc)` for performance.

---

## Rollout Plan

* [x] Development environment setup validated on Ubuntu 22.04.
* [x] GitHub Actions workflow enabled for pushes, PRs, and nightly runs.
* [x] Docker Compose healthchecks validated.
* [ ] Merge Phase 1 branch → `develop`.
* [ ] Tag `v0.1.0-foundation` release.

---

## Success Metrics

| Metric                     | Target                          | Measurement                        |
| -------------------------- | ------------------------------- | ---------------------------------- |
| CI Pass Rate               | 100 %                           | All workflows green on main branch |
| Code Lint Score            | 0 warnings                      | Clippy / GolangCI-Lint reports     |
| Security Findings          | 0 critical CVEs                 | cargo-audit output                 |
| Build Reproducibility      | < 10 min CI runtime             | GitHub Actions timings             |
| Documentation Completeness | 100 % of mandatory docs present | Manual review                      |

---

## Compliance Mapping

| AIBD-RG Section                | Implementation Evidence                           |
| ------------------------------ | ------------------------------------------------- |
| **§1 Project Structure**       | Separate Rust & Go feature modules + CI config    |
| **§2 Architecture Guidelines** | No cross-feature dependencies / clear boundaries  |
| **§3 File Organization**       | One primary type per file (plan Phase 2 refactor) |
| **§4 Code Style Rules**        | rustfmt + clippy + go fmt + golangci-lint pass    |
| **§5 Documentation Standards** | PRD created (✓ this file)                         |
| **§6 Testing Guidelines**      | Unit + Integration + Fuzz tests implemented       |
| **§7 AI Interaction Rules**    | AIBD-RG-compliant prompting followed              |
| **§8 CI/CD Integration**       | GitHub Actions workflow matches template          |
| **§9 Security and Compliance** | cargo-audit + deny + no unsafe code               |
| **§10 Anti-Pattern Avoidance** | Verified no feature boundary violations           |

---

## Sign-Off

| Role              | Name | Date | Approval |
| ----------------- | ---- | ---- | -------- |
| Project Owner     | Kevin Mulcahy   | 10/6/25    | ☐        |

---

## Next Phase

Proceed to **Phase 2: Core Switching Features** after approval of this document and tagging `v0.1.0-foundation`.
