//! Types for forwarding feature

/// Represents a MAC address in forwarding context.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacAddress(pub [u8; 6]);

/// VLAN identifier (0–4095).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VlanId(pub u16);

// src/features/forwarding/types.rs

/// Minimal representation of an Ethernet frame for fuzzing.
/// Will be extended with full parsing/serialization later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EthernetFrame {
    pub dst_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ethertype: u16,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    pub fn new(dst_mac: [u8; 6], src_mac: [u8; 6], ethertype: u16, payload: Vec<u8>) -> Self {
        EthernetFrame {
            dst_mac,
            src_mac,
            ethertype,
            payload,
        }
    }
}
