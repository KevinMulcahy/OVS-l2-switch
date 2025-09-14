# Standard Operating Procedure (SOP): GitHub Issue Lifecycle Management

## 1. Purpose
This SOP defines how issues are reported, triaged, tracked, and closed for the Open-Source Layer 2 Switch project.  
The process ensures **traceability**, **consistency**, and **compliance** with AIBD-RG principles.

---

## 2. Scope
Applies to all project contributors and maintainers.  
Covers:  
- Bug reports  
- Feature requests (tied to Roadmap & PRDs)  
- Documentation tasks  
- CI/CD and testing tasks  

---

## 3. Roles & Responsibilities
- **Reporter**: Opens issues, provides required details using templates.  
- **Maintainer**: Triages, assigns labels/milestones, ensures adherence to AIBD-RG.  
- **Contributor**: Works on issues, references PRDs and Roadmap.  
- **Reviewer**: Reviews PRs linked to issues, checks acceptance criteria.  

---

## 4. Issue Reporting
1. Use GitHub **issue templates**:
   - `bug_report.md`  
   - `feature_request.md`  
   - `documentation.md`  
   - `ci_task.md`  
2. Required fields:
   - **Description** (clear and concise).  
   - **Expected behavior / outcome**.  
   - **Roadmap phase linkage** (`Phase 1 — Foundations`, etc.).  
   - **References** (PRDs, ARCHITECTURE.md, specs).  
   - **Acceptance criteria** (testable conditions).  

3. If relevant, attach logs, screenshots, or failing test outputs.

---

## 5. Issue Triage
Performed by maintainers within **48 hours**:  
- Assign **labels**: `bug`, `enhancement`, `documentation`, `ci`, `phase1`, etc.  
- Assign **milestone** (e.g., `Phase 1 — Foundations`).  
- Add to **GitHub Project Board** (column = *To Do*).  
- Cross-reference:
  - Related PRD (`src/features/.../prd.md`)  
  - Architecture/roadmap sections  

---

## 6. Issue Tracking
- Issues progress through **Kanban workflow**:  
  - **To Do → In Progress → Review → Done**.  
- Contributors must:  
  - Reference issue in commits (`Fixes #123`).  
  - Ensure implementation aligns with **PRD + ARCHITECTURE.md**.  
  - Update documentation (`README.md`, `INSTALL.md`, specs) as needed.  
- CI/CD automatically validates:
  - Linting/formatting  
  - Unit & integration tests  
  - Dependency boundaries  

---

## 7. Issue Closure
An issue may be closed when:  
- PR(s) addressing the issue are merged.  
- Acceptance criteria are verified via tests.  
- Docs are updated (if required).  
- Maintainer confirms milestone alignment.  

Closure must include:  
- Reference to merged PR(s).  
- Checklist of acceptance criteria.  
- Links to updated docs/PRDs.  

---

## 8. Escalation
If issues are **blocked**:  
- Add `blocked` label.  
- Comment with details on what is blocking.  
- Review blockers during weekly triage.  

---

## 9. Continuous Improvement
- Review open issues during milestone retrospectives.  
- Refine templates and labels as the project evolves.  
- Capture lessons learned into updated AIBD-RG practices.  

---

## 10. References
- [ARCHITECTURE.md](./ARCHITECTURE.md)  
- [ROADMAP.md](./ROADMAP.md)  
- [AI-Based Coding Rules for Rust and Go Development (AIBD-RG)](./AI%20Based%20coding%20rules%20for%20Rust%20and%20Go%20development.md)  
