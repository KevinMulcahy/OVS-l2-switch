# Feature Name: Control Plane Core Logic

## Objective
Implement the control plane in Go to manage switching protocols, compute forwarding policies, and interact with the dataplane. This layer ensures correct, efficient, and dynamic updates to the dataplane while enforcing feature boundaries.

## Business Context
The control plane orchestrates the behavior of the switch. It handles MAC aging, STP computation, VLAN configuration, and interfaces with the management plane for policy enforcement. A robust control plane ensures network stability, consistency, and compliance with operator intents.

## User Stories
- As a network operator, I want the switch to prevent loops using STP so that the network remains stable.
- As a control plane, I want to update the dataplane forwarding tables dynamically so that traffic routing reflects the latest policies.
- As a developer, I want clear separation between control plane logic and dataplane to maintain modularity and testability.

## Requirements

### Functional Requirements
- Compute MAC aging and initiate removal of stale entries.
- Implement VLAN and STP logic.
- Push forwarding table updates to the dataplane via IPC/RPC.
- Persist configuration using systemd service integration.
- Expose an internal API for management plane interactions (REST/gRPC in future phases).

### Non-Functional Requirements
- Performance: Minimal latency in propagating control plane updates to dataplane.
- Security: Only authorized management plane components can trigger updates.
- Reliability: Resilient to invalid or conflicting updates; robust error handling.
- Scalability: Support growing MAC table sizes and VLANs without performance degradation.
- Maintainability: Clear module boundaries; features isolated per AIBD-RG guidelines.

## Acceptance Criteria
- [ ] MAC aging and flushing works correctly under load.
- [ ] VLAN and STP computations are correct per test scenarios.
- [ ] Forwarding table updates are accurately reflected in the dataplane.
- [ ] Configuration persistence works across system restarts.
- [ ] Internal API responds with correct status and validation errors.

## Out of Scope
- Northbound REST API (Phase 3, Management Plane responsibility).
- Packet-level forwarding logic (handled in dataplane).
- GUI, CLI, or mobile integration.

## Dependencies
- Internal: `shared` utilities (logging, types), `dataplane` feature for table updates.
- External: IPC/RPC transport (UNIX sockets initially).
- Infrastructure: Go 1.21+, CI/CD pipelines, systemd for persistence.

## API Design (if applicable)
- `func AddMACEntry(mac MACAddress, port PortID)`  
- `func RemoveMACEntry(mac MACAddress)`  
- `func UpdateVLANConfig(vlanID VLANID, ports []PortID)`  
- `func ComputeSTP()`  
- `func PushForwardingTable()`  

## Data Model (if applicable)
```
MACEntry {
    MAC: MACAddress
    Port: PortID
    LastSeen: Timestamp
}

VLANEntry {
    VLANID: uint16
    Ports: []PortID
}

STPState {
    RootBridgeID: BridgeID
    PortStates: map[PortID]PortState
}
```

## Security Considerations
- Authentication: Only management plane and internal trusted components can trigger updates.
- Authorization: Verify roles for configuration changes.
- Data Protection: Validate inputs to prevent inconsistent forwarding state.
- Input Validation: MAC addresses, VLAN IDs, and STP parameters.

## Testing Strategy
- Unit Tests: VLAN handling, MAC aging, STP computation.
- Integration Tests: Forwarding table updates sent to dataplane and validated.
- End-to-End Tests: Control plane correctly responds to management plane commands.
- Performance Tests: Stress test with high rate of MAC updates and topology changes.

## Rollout Plan
- [ ] Development environment: test local IPC/RPC updates with simulated dataplane.
- [ ] Staging environment: integrate with real dataplane instance.
- [ ] Production deployment: systemd-managed control plane service.
- [ ] Monitoring and alerts: logging for errors, policy conflicts, and update latency.

## Success Metrics
- Accuracy: 100% of forwarding table updates correctly applied in dataplane.
- Timeliness: STP convergence and MAC updates applied within target latency.
- Reliability: No crashes or inconsistent states under load.
- Observability: All updates logged and traceable for audit.
