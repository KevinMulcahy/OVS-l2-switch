# Phase 1 Task Checklist — Foundations

This document tracks all actions required to complete **Phase 1: Foundations** as defined in [ROADMAP.md](../ROADMAP.md).  
It follows the **AIBD-RG guidelines** (documentation-as-code, clear boundaries, PRD-driven development).  

---

## ✅ Architecture
- [ ] Finalize `ARCHITECTURE.md` with:
  - Vision and goals
  - Component breakdown (dataplane in Rust, control & management in Go)
  - Interfaces & communication rules
  - Roadmap alignment (Phases 1–6)
  - Security, dependency, and testing strategy
- [ ] Add Mermaid diagram (inline in `ARCHITECTURE.md`)
- [ ] Store PNG/SVG diagrams in `docs/`

---

## 📄 Documentation
- [ ] Create `docs/PRD_TEMPLATE.md` (per-feature Product Requirement Document template)
- [ ] Add stubs for per-feature PRDs:
  - [ ] `src/features/dataplane/prd.md`
  - [ ] `internal/features/control_plane/prd.md`
  - [ ] `internal/features/management/prd.md`
- [ ] Add `docs/DEPENDENCIES.md` with:
  - Explicit Rust + Go module boundary rules
  - Examples of allowed/forbidden dependencies
  - CI checks for dependency cycles
- [ ] Add `docs/DEVELOPER_SETUP.md` with:
  - Toolchain versions (Rust stable, Go 1.21+)
  - Install commands (rustup, clippy, golangci-lint, gosec)
  - Optional Docker dev environment
- [ ] Add `docs/TEST_STRATEGY.md` mapping:
  - PRD acceptance criteria → unit, integration, e2e tests
  - How to run tests locally
- [ ] Add `INSTALL.md` for supported Linux distros
- [ ] Ensure `README.md` is complete and properly formatted
- [ ] Add `CONTRIBUTING.md` for contribution guidelines
- [ ] Add open-source `LICENSE` file

---

## ⚙️ Repository & CI/CD
- [ ] Create `Makefile` for build/test convenience
- [ ] Add GitHub Actions workflow for Rust:
  - [ ] Formatting (`cargo fmt`)
  - [ ] Linting (`cargo clippy`)
  - [ ] Tests (`cargo test`)
  - [ ] Security scans (`cargo-audit`, `cargo-deny`)
  - [ ] Dependency graph checks (`cargo-deps`)
- [ ] Add GitHub Actions workflow for Go:
  - [ ] Formatting (`gofmt`)
  - [ ] Linting (`golangci-lint`)
  - [ ] Tests (`go test -race -cover`)
  - [ ] Security scans (`gosec`, `govulncheck`)
  - [ ] Import cycle checks

---

## 🧪 Minimal Prototype
- [ ] Implement **basic packet forwarding** in Rust dataplane using Linux networking stack
- [ ] Add at least one **integration test** to validate packet forwarding path

---

## 🎯 Completion Criteria
Phase 1 is complete when:
- Architecture is documented (`ARCHITECTURE.md` + diagrams)
- Repository standards are in place (LICENSE, CONTRIBUTING, README, INSTALL, DEVELOPER_SETUP)
- CI/CD pipelines run for both Rust and Go (lint, test, security, dependency checks)
- Minimal Rust dataplane forwards packets with at least one passing integration test