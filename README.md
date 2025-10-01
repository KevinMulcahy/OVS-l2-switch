# Open-Source Layer 2 Switching Platform

  

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[![Rust CI](https://img.shields.io/badge/Rust-CI-green)](https://github.com/YourUsername/your-repo/actions/workflows/rust.yml)

[![Go CI](https://img.shields.io/badge/Go-CI-green)](https://github.com/YourUsername/your-repo/actions/workflows/go.yml)

[![Coverage Status](https://img.shields.io/badge/Coverage-100%25-brightgreen)](#) <!-- placeholder -->

[![Release](https://img.shields.io/github/v/release/YourUsername/your-repo)](https://github.com/YourUsername/your-repo/releases)

  

---

  

## Table of Contents

  

1. [Overview](#overview)

2. [Architecture](#architecture)

3. [Roadmap](#roadmap)

4. [Getting Started](#getting-started)

5. [Quick Start / Demo](#quick-start--demo)

6. [Feature PRDs](#feature-prds)

7. [Testing & Security](#testing--security)

8. [CI/CD & Release](#cicd--release)

9. [Contribution Guidelines](#contribution-guidelines)

10. [License](#license)

11. [Contact & Support](#contact--support)

  

---

  

## Overview

  

This project provides an **open-source, modular Layer 2 switch** for PC architecture. It supports:

  

- High-performance packet forwarding

- Rust-based, memory-safe dataplane

- Go-based control and management planes

- CLI, Web GUI, and mobile user interfaces

- Extensible architecture for ICS/enterprise scenarios

  

**Supported platforms:** Linux (Ubuntu/Debian recommended)

**Minimum requirements:** 4GB RAM, 2 CPU cores, 20GB disk space

  

This project follows **AIBD-RG guidelines** for feature-based modularity, PRD-driven development, and secure, maintainable code:contentReference[oaicite:0]{index=0}.

  

---

  

## Architecture

  

The system uses a **modular, phased architecture**:

  

```mermaid

flowchart TD

UI[User Interfaces: CLI, GUI, Mobile]

MGMT[Management Plane - Go]

CTRL[Control Plane - Go]

DP[Dataplane - Rust]

EXT[External Integrations: Netlink, OVSDB, Systemd]

  

UI <--> MGMT

MGMT <--> CTRL

CTRL <--> DP

CTRL <--> EXT

````

  

### Components

  

| Component | Language | Responsibilities |

| ------------------------- | ---------------- | ----------------------------------------------------------------------------------------- |

| **Dataplane** | Rust | Packet I/O, MAC learning, VLAN tagging, forwarding logic, memory-safe implementation |

| **Control Plane** | Go | Switching protocols (STP, VLAN, MAC aging), policy computation, configuration persistence |

| **Management** | Go | REST API (Northbound), RBAC, logging, monitoring, API versioning |

| **User Interfaces** | CLI, Web, Mobile | Switch management, monitoring, configuration visualization |

| **External Integrations** | Go | Linux Netlink, OVSDB, systemd integration |

  

Full architecture and module boundaries are described in [ARCHITECTURE.md](docs/ARCHITECTURE.md).

  

---

  

## Roadmap

  

This project follows a **phased roadmap**:

  

*  **Phase 1:** Foundations (basic forwarding, repo setup, CI/CD, documentation)

*  **Phase 2:** Core switching (MAC learning, VLAN, STP, systemd persistence)

*  **Phase 3:** Management & APIs (REST API, RBAC, logging, monitoring, integrations)

*  **Phase 4:** User Interfaces (CLI, Web GUI, Mobile)

*  **Phase 5:** Advanced switching (LACP, QoS, ACLs, DPDK/eBPF)

*  **Phase 6:** Ecosystem & Community (SDKs, docs site, demo labs, v1.0 release)

  

See [Roadmap](ROADMAP.md) for detailed timelines and deliverables.

  

---

  

## Getting Started

  

### Prerequisites

  

*  **Rust:** 1.70+ (dataplane)

*  **Go:** 1.21+ (control & management planes)

*  **Linux:** Ubuntu/Debian recommended

* Tools: `make`, `git`, `docker` (optional)

  

**Developer Notes:**

  

* WSL2 recommended on Windows for local builds

* Rust nightly may be needed for fuzzing targets

* Go modules used for dependency management

  

### Clone Repository

  

```bash

git  clone  https://github.com/YourUsername/your-repo.git

cd  your-repo

```

  

### Build Dataplane (Rust)

  

```bash

cd  src

cargo  build  --release

```

  

### Build Control & Management Planes (Go)

  

```bash

cd  internal

go  build  ./features/...

```

  

### Run Tests

  

```bash

# Rust

cargo  test  --all-features

  

# Go

make  test

```

  

### Run Locally

  

```bash

# Start control plane

./internal/features/control_plane/main &

  

# Start management plane

./internal/features/management/main &

  

# Run Rust dataplane in test mode

cargo  run  --bin  dataplane  --  --test-vlan  1

```

  

**Docker Test Environment:**

  

```bash

docker  compose  -f  docker-compose.test.yml  up  --exit-code-from  tests

```

  

---

  

## Quick Start / Demo

  

```bash

# Start dataplane in test mode

cargo  run  --bin  dataplane  --  --test-vlan  1

  

# Start control plane in background

./internal/features/control_plane/main &

  

# Add sample MAC entries (via CLI or API)

curl  -X  POST  http://localhost:8080/api/v1/mac  \

-d '{"mac":"00:11:22:33:44:55","vlan":1}'

```

  

Check logs to see **packet forwarding behavior**.

  

---

  

## Feature PRDs

  

Each feature has a PRD (`prd.md`) defining functional and non-functional requirements, acceptance criteria, and dependencies:

  

* Forwarding: [src/features/forwarding/prd.md](src/features/forwarding/prd.md)

* Management: [internal/features/management/prd.md](internal/features/management/prd.md)

  

---

  

## Testing & Security

  

**PRD-Driven Tests:** Each acceptance criterion maps to unit, integration, and e2e tests.

  

**Security Practices:**

  

* Rust: forbid unsafe code (`#![forbid(unsafe_code)]`), input validation via `validator`

* Go: TLS 1.2+, request timeouts, context propagation, input validation

* Fuzzing targets in `fuzz/fuzz_targets/`

  

**Tools & CI Enforcement:**

  

* Rust: `cargo fmt`, `clippy`, `test`, `cargo-audit`, `cargo-deny`

* Go: `go fmt`, `golangci-lint`, `go test -race`, `gosec`, `govulncheck`

  

---

  

## CI/CD & Release

  

* Separate Rust & Go CI pipelines run linting, formatting, tests, circular dependency checks, and security scans.

* Release automation uses `git-cliff` to generate changelogs and GitHub Actions to create releases.

  

---

  

## Contribution Guidelines

  

Follow **AIBD-RG coding standards**:

  

* Feature-based module boundaries

* One declaration per file

* Type safety and consistent error handling

* CI-enforced formatting, linting, and security checks

  

**GitHub Best Practices:**

  

* Fork and submit PRs

* Use conventional commits (`feat`, `fix`, `chore`, `docs`)

* Branch naming: `feature/<name>` or `bugfix/<name>`

* Maintain PRD-driven tests for new features

  

**Module/Test Ownership:**

Each module has a designated owner responsible for reviewing code, test coverage, and fixing flaky tests (OWNERS.md style).

  

---

  

## License

  

This project is licensed under the [MIT License](LICENSE).

  

---

  

## Contact & Support

  

* Open GitHub Issues / Pull Requests for bugs or features

* Use Discussions for general support

* Urgent queries: contact maintainers as listed in `DEVELOPER_SETUP.md`
