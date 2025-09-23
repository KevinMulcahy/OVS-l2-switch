# Feature: Forwarding

## Objective
Implement core Layer 2 packet forwarding (flooding, unicast, VLAN-aware).

## Business Context
This feature provides the dataplane’s essential capability: deciding how packets are forwarded between interfaces.

## User Stories
- As a switch operator, I want packets to be forwarded to the correct port so that devices can communicate.
- As a network engineer, I want unknown MACs to be flooded until learned, so that switching behaves correctly.
- As an operator, I want VLAN tags respected, so that traffic separation works.

## Requirements
### Functional
- Support flooding for unknown destinations
- Support unicast forwarding using learned MAC addresses
- Respect VLAN tags (802.1Q)

### Non-Functional
- Safe Rust (`#![forbid(unsafe_code)]`)
- Modular design with clear feature boundaries
- Unit tests aligned with acceptance criteria

## Acceptance Criteria
- [ ] Forward packet with known MAC → correct output port
- [ ] Unknown MAC → flood to all except ingress
- [ ] VLAN separation enforced
- [ ] MAC table aging handled

## Out of Scope
- Spanning Tree Protocol (covered in later phase)

## Dependencies
- Uses `shared::types` for error/result handling
- Relies on `shared::utils::logging`

## Security Considerations
- Input validation for VLAN IDs
- Avoid unsafe Rust code

## Testing Strategy
- Unit tests for MAC table
- Integration tests for forwarding logic

