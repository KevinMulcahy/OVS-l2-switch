# Product Requirement Document (PRD) Template

This template defines the structure for all feature PRDs in this project.  
Each PRD must live alongside the feature code (e.g., `src/features/forwarding/prd.md`)  
and must be **kept up to date** with implementation and tests.  

---

## 1. Feature Overview
**Feature Name:**  
**Owner(s):**  
**Status:** Draft / In Progress / Complete  

---

## 2. Problem Statement
- What problem does this feature solve?  
- Why is it needed in the switch architecture?  
- Which Roadmap phase does it belong to? (link to [ROADMAP.md](../ROADMAP.md))  

---

## 3. Goals & Non-Goals
**Goals**
-  

**Non-Goals**
-  

---

## 4. Functional Requirements
List requirements in detail. Each requirement must be testable.  

Example:  
- The forwarding feature MUST forward Ethernet frames between interfaces.  
- The forwarding feature MUST support flooding for unknown MACs.  
- The feature MUST expose a stats API for packets forwarded/dropped.  

---

## 5. Non-Functional Requirements
- Performance (throughput, latency targets)  
- Reliability (graceful failure, retries, error handling)  
- Security (input validation, isolation)  
- Compatibility (Linux kernel version, supported distros)  

---

## 6. Architecture & Design
- Describe the design approach.  
- Include diagrams if needed.  
- Show module layout (files, internal vs. shared code).  
- Define how it interacts with other components (dataplane, control plane, management).  

---

## 7. API / Interface Contract
- Define public APIs for this feature.  
- If applicable, link to `specs/` files (protobuf, OpenAPI).  
- Show input/output formats and validation rules.  

---

## 8. Dependencies
- Internal dependencies (shared modules).  
- External dependencies (crates, Go modules).  
- CI/CD checks required.  

---

## 9. Test Plan
Each functional requirement must map to tests.  

**Unit Tests**
- What core logic is tested?  

**Integration Tests**
- How does this feature interact with others?  

**End-to-End Tests**
- Full switch behavior validation.  

---

## 10. Acceptance Criteria
- [ ] Criteria 1 (linked to requirement/test)  
- [ ] Criteria 2  
- [ ] Criteria 3  

---

## 11. Risks & Mitigations
- Potential risks (performance bottlenecks, library issues, security).  
- Mitigations planned.  

---

## 12. References
- Related PRDs  
- Links to `ARCHITECTURE.md`, `ROADMAP.md`  
- External specs (RFCs, IEEE standards)  

---