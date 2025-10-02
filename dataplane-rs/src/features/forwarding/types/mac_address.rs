// src/features/forwarding/types/mac_address.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacAddress(pub [u8; 6]);

impl MacAddress {
    pub fn new(addr: [u8; 6]) -> Self {
        Self(addr)
    }
}
