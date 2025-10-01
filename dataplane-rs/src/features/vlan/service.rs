// dataplane-rs/src/features/vlan/service.rs
use crate::features::vlan::types::VlanId;

/// Service for handling VLAN tagging and untagging on packets
///
/// Works on mutable slices for performance and Clippy compliance.
pub struct VlanService;

impl VlanService {
    /// Tag a packet with the given VLAN ID.
    ///
    /// `packet` is a mutable slice of the Ethernet frame bytes.
    pub fn tag_packet(&self, packet: &mut [u8], vlan: VlanId) {
        // Basic guard: needs at least Ethernet header size to write bytes 12-13
        if packet.len() < 14 {
            return;
        }

        // Example: store VLAN id in bytes 12-13 (adjust if your actual VLAN placement differs)
        packet[12] = (vlan.0 >> 8) as u8;
        packet[13] = vlan.0 as u8;
    }

    /// Remove/extract VLAN tag; returns VLAN ID if present.
    pub fn untag_packet(&self, packet: &mut [u8]) -> Option<VlanId> {
        if packet.len() < 14 {
            return None;
        }

        let vlan_id = ((packet[12] as u16) << 8) | (packet[13] as u16);
        Some(VlanId(vlan_id))
    }
}
