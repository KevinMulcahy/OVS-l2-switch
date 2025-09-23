//! Types for forwarding feature

/// Represents a MAC address in forwarding context.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacAddress(pub [u8; 6]);

/// VLAN identifier (0–4095).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VlanId(pub u16);