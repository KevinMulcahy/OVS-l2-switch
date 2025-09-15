# Feature Name: Dataplane Core Switching

## Objective
Provide a high-performance, memory-safe Layer 2 switching dataplane in Rust that handles packet I/O, MAC learning, VLAN tagging, and forwarding logic.

## Business Context
The dataplane is the foundation of the L2 switch. It must be performant, reliable, and safe, exposing APIs for control plane interaction. This feature supports all subsequent functionality including VLANs, STP, LACP, QoS, and ACLs.

## User Stories
- As a network operator, I want packets to be forwarded efficiently so that network performance is optimized.
- As a control plane, I want to update forwarding tables dynamically so that traffic can be rerouted without downtime.
- As a developer, I want the dataplane to enforce memory safety so that runtime crashes and security issues are minimized.

## Requirements

### Functional Requirements
- Receive and transmit Ethernet frames using the Linux networking stack.
- Implement MAC learning table with aging and flooding logic.
- Support VLAN tagging/untagging (802.1Q).
- Provide API for control plane to modify forwarding rules.
- Enforce safe concurrent access to shared resources.

### Non-Functional Requirements
- Performance: Handle line-rate forwarding for 1G/10G interfaces with minimal latency.
- Security: No unsafe Rust code; input validation for control-plane commands.
- Reliability: Zero packet loss on stable paths; proper error handling.
- Scalability: Support at least 1024 MAC addresses initially; easily extendable.

## Acceptance Criteria
- [ ] Packets forwarded correctly according to learned MAC addresses.
- [ ] Flooding occurs when destination MAC is unknown.
- [ ] VLAN-tagged traffic is correctly handled.
- [ ] Control plane can update forwarding table via API.
- [ ] Thread-safe handling of concurrent packet processing.

## Out of Scope
- Advanced features like LACP, ACLs, QoS (Phase 5).
- GUI, CLI, or API integration (handled by management/control planes).
- DPDK/eBPF acceleration (future optimization).

## Dependencies
- Internal: `shared` utilities (logging, types), `forwarding` feature for pipeline.
- External: Linux networking stack (Netlink optional for future phases).
- Infrastructure: Rust 1.70+, tokio runtime, CI/CD pipelines.

## API Design (if applicable)
- `fn add_mac_entry(mac: MacAddress, port: PortId)`  
- `fn remove_mac_entry(mac: MacAddress)`  
- `fn lookup_port(mac: MacAddress) -> Option<PortId>`  
- `fn process_packet(packet: EthernetFrame)`  

## Data Model (if applicable)
```
MacTableEntry {
    mac: MacAddress
    port: PortId
    last_seen: Timestamp
}

VlanEntry {
    vlan_id: u16
    ports: Vec<PortId>
}
```

## Security Considerations
- Authentication: Not applicable at dataplane; controlled via control plane.
- Authorization: Only trusted control plane can update tables.
- Data Protection: Ensure memory safety; avoid buffer overflows.
- Input Validation: Validate MAC addresses, VLAN IDs, and frame length.

## Testing Strategy
- Unit Tests: MAC table operations, VLAN tagging logic.
- Integration Tests: Forwarding behavior under concurrent load.
- End-to-End Tests: Control plane API commands reflected in dataplane behavior.
- Performance Tests: Throughput and latency benchmarks on testbed interfaces.

## Rollout Plan
- [ ] Development environment: local packet injection tests.
- [ ] Staging environment: testbed with multiple interfaces.
- [ ] Production deployment: initial Linux stack deployment with monitoring.
- [ ] Monitoring and alerts: logging for packet drops, errors.

## Success Metrics
- Forwarding correctness: 100% of test packets reach intended ports.
- MAC learning efficiency: learn and age entries accurately.
- CPU utilization: below 50% under simulated load.
- Error rate: zero crashes, safe error handling.
