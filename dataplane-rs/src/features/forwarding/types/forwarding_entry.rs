use crate::features::forwarding::types::{MacAddress, Vlan};

/// A single entry in the forwarding table
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardingEntry {
    /// MAC address associated with this entry
    pub mac: MacAddress,

    /// VLAN ID for this entry
    pub vlan: Vlan,

    /// Port index where the MAC address was learned
    pub port: u16,
}

impl ForwardingEntry {
    /// Create a new forwarding entry
    pub fn new(mac: MacAddress, vlan: Vlan, port: u16) -> Self {
        Self { mac, vlan, port }
    }

    /// Display the entry in a readable format
    pub fn display(&self) -> String {
        format!(
            "ForwardingEntry {{ mac: {:?}, vlan: {:?}, port: {} }}",
            self.mac, self.vlan, self.port
        )
    }
}
