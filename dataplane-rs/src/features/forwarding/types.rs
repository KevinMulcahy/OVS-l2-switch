// src/features/forwarding/types.rs
#[derive(Debug, Clone)]
pub struct NetIf {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct EthernetFrame {
    pub dst_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ethertype: u16,
    pub payload: Vec<u8>,
}
