# Feature Name: Management Plane Core API

## Objective
Provide a robust management plane in Go to expose northbound APIs, enforce RBAC, and monitor/control the switch. This layer enables secure and auditable access to switch functionality and operational data.

## Business Context
The management plane centralizes administrative control of the switch. It serves as the northbound interface for operators, automation scripts, and external systems. It must be secure, reliable, and scalable, while remaining decoupled from the dataplane and control plane implementations.

## User Stories
- As a network operator, I want to configure VLANs, view MAC tables, and monitor traffic so that I can manage the network efficiently.
- As a system administrator, I want RBAC enforced so that only authorized users can make configuration changes.
- As a developer, I want a clear API contract so that integrations with scripts, CLI, or GUI remain consistent.

## Requirements

### Functional Requirements
- Expose REST APIs for configuration, monitoring, and operational queries.
- Enforce Role-Based Access Control (RBAC) for all API endpoints.
- Provide logging and metrics integration (syslog, Graylog, Prometheus).
- Validate and sanitize all incoming requests.
- Forward validated commands to the control plane.
- Maintain audit trails for all management actions.

### Non-Functional Requirements
- Performance: Handle concurrent API requests with minimal latency.
- Security: TLS 1.2+ for all endpoints, enforce authentication and authorization.
- Reliability: High availability; fail gracefully on downstream errors.
- Scalability: Support hundreds of concurrent API clients.
- Maintainability: Clear modular separation, following AIBD-RG guidelines.

## Acceptance Criteria
- [ ] All API endpoints validate inputs and enforce RBAC.
- [ ] Management plane updates are correctly propagated to control plane.
- [ ] Metrics and logs are produced for all actions.
- [ ] Unauthorized requests are rejected with proper error codes.
- [ ] System recovers gracefully from downstream failures.

## Out of Scope
- Packet-level processing (handled by dataplane).
- Control plane computation of policies (handled in control plane).
- User interface (GUI, CLI) implementation (future phases).

## Dependencies
- Internal: `control_plane` feature for command propagation, `shared` utilities (logging, types, validation).
- External: TLS certificates, metrics collectors (Prometheus), syslog/Graylog services.
- Infrastructure: Go 1.21+, CI/CD pipelines, containerization or systemd deployment.

## API Design (if applicable)
- `POST /api/v1/vlans` – Create VLAN  
- `GET /api/v1/mac_table` – Retrieve MAC table  
- `PUT /api/v1/stp/config` – Update STP parameters  
- `DELETE /api/v1/vlans/{id}` – Delete VLAN  
- RBAC enforced via `Authorization` header or token-based authentication

## Data Model (if applicable)
```
VLANConfig {
    VLANID: uint16
    Ports: []PortID
}

MACTableView {
    MAC: MACAddress
    Port: PortID
    VLAN: VLANID
    LastSeen: Timestamp
}

User {
    UserID: UUID
    Role: RoleEnum
    Token: AuthToken
}
```

## Security Considerations
- Authentication: Token-based or certificate-based.
- Authorization: RBAC enforced for all operations.
- Data Protection: TLS encryption; input validation to prevent injection attacks.
- Audit: Log all management actions with timestamp, user, and outcome.
- Rate Limiting: Prevent API abuse.

## Testing Strategy
- Unit Tests: Validate API input/output, RBAC enforcement, logging.
- Integration Tests: Ensure commands are propagated to control plane.
- End-to-End Tests: Simulate operator actions through REST API; verify control plane updates and metrics.
- Performance Tests: Concurrent API requests, failover scenarios.

## Rollout Plan
- [ ] Development environment: local REST API with mocked control plane.
- [ ] Staging environment: integrated control plane and dataplane instances.
- [ ] Production deployment: containerized or systemd-managed management plane.
- [ ] Monitoring and alerts: API errors, failed authentication attempts, command failures.

## Success Metrics
- API correctness: 100% of validated requests processed accurately.
- Security compliance: No unauthorized access allowed.
- Latency: API response time below target threshold under load.
- Observability: Logs and metrics available for all management actions.
