use crate::features::forwarding::types::{MacAddress, VlanId};

/// A single entry in the forwarding table
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForwardingEntry {
    /// MAC address associated with this entry
    pub mac: MacAddress,

    /// VLAN ID for this entry
    pub vlan: VlanId,

    /// Port index where the MAC address was learned
    pub port: u16,
}

impl ForwardingEntry {
    /// Create a new forwarding entry
    pub fn new(mac: MacAddress, vlan: VlanId, port: u16) -> Self {
        Self { mac, vlan, port }
    }

    /// Display the entry in a readable format (for debugging/logging)
    pub fn display(&self) -> String {
        format!(
            "ForwardingEntry {{ mac: {:?}, vlan: {:?}, port: {} }}",
            self.mac, self.vlan, self.port
        )
    }
}

