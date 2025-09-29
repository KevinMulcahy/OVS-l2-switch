use std::convert::TryFrom;
use crate::shared::error::AppError; // replace with your crate error type

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
