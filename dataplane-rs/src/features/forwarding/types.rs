//! Types for forwarding feature

/// Represents a MAC address in forwarding context.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacAddress(pub [u8; 6]);

/// VLAN identifier (0–4095).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VlanId(pub u16);

/// Represents a VLAN tag (wrapper for type safety)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VlanTag(pub u16);

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
    /// Construct a new Ethernet frame.
    pub fn new(dst_mac: [u8; 6], src_mac: [u8; 6], ethertype: u16, payload: Vec<u8>) -> Self {
        EthernetFrame {
            dst_mac,
            src_mac,
            ethertype,
            payload,
        }
    }
}

use std::convert::TryFrom;
use crate::shared::error::AppError; // Replace with your crate's error type

/// Try converting from raw bytes into an Ethernet frame.
impl TryFrom<&[u8]> for EthernetFrame {
    type Error = AppError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 14 {
            return Err(AppError::InvalidFrameLength);
        }

        let dst_mac = data[0..6].try_into().expect("slice with exact length");
        let src_mac = data[6..12].try_into().expect("slice with exact length");
        let ethertype = u16::from_be_bytes(data[12..14].try_into().unwrap());
        let payload = data[14..].to_vec();

        Ok(EthernetFrame {
            dst_mac,
            src_mac,
            ethertype,
            payload,
        })
    }
}
