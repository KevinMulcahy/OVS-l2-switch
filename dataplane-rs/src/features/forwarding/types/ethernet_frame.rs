use crate::shared::error::AppError;
use std::convert::TryFrom;

/// Minimal representation of an Ethernet frame.
/// For fuzzing and basic parsing; will be extended with
/// full serialization/deserialization later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EthernetFrame {
    pub dst_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ethertype: u16,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    /// Construct a new Ethernet frame manually.
    pub fn new(dst_mac: [u8; 6], src_mac: [u8; 6], ethertype: u16, payload: Vec<u8>) -> Self {
        EthernetFrame {
            dst_mac,
            src_mac,
            ethertype,
            payload,
        }
    }

    /// Return the total length of the frame in bytes.
    pub fn len(&self) -> usize {
        14 + self.payload.len()
    }

    /// Check if the frame has no payload.
    pub fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }
}

/// Try to construct an Ethernet frame from raw bytes.
///
/// # Errors
/// Returns `AppError::InvalidFrameLength` if the slice is shorter than
/// the minimum Ethernet frame header (14 bytes).
impl TryFrom<&[u8]> for EthernetFrame {
    type Error = AppError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 14 {
            return Err(AppError::InvalidFrameLength);
        }

        let dst_mac = data[0..6].try_into().expect("slice length checked");
        let src_mac = data[6..12].try_into().expect("slice length checked");
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
