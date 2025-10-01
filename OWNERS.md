# OWNERS.md

This file defines **test ownership** for each module in the Layer 2 Switching Platform.  
Currently, all modules are owned and maintained by @KevinMulcahy.  

Responsibilities include:
- Reviewing test coverage
- Fixing flaky tests
- Maintaining fuzzing targets
- Triaging CI failures

---

## Dataplane (Rust)
- **Owner:** @KevinMulcahy  
- **Scope:** Packet I/O, MAC learning, VLAN logic, forwarding pipeline  

---

## Control Plane (Go)
- **Owner:** @KevinMulcahy  
- **Scope:** Switching protocols (STP, VLAN, MAC aging), config persistence  

---

## Management Plane (Go)
- **Owner:** @KevinMulcahy  
- **Scope:** REST API, RBAC, logging, monitoring integrations  

---

## Notes
- As new contributors join, update this file to assign module-level responsibilities.  
- CI failures should be triaged first by the assigned owner(s).  
