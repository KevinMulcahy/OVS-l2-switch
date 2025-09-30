// dataplane-rs/src/features/vlan/service.rs

use crate::features::vlan::types::VlanId;

pub struct VlanService;

impl VlanService {
    /// Tags a packet with the given VLAN ID
    ///
    /// # Arguments
    /// * `packet` - Mutable slice of the packet bytes
    /// * `vlan` - VLAN ID to insert
    pub fn tag_packet(&self, packet: &mut [u8], vlan: VlanId) {
        // Example: prepend VLAN tag (implementation depends on your VLAN format)
        // Note: packet must have enough capacity to hold VLAN tag
        if packet.len() < 4 {
            return; // or handle error
        }

        // Example: insert VLAN ID at bytes 12-13 (Ethernet header)
        packet[12] = (vlan.0 >> 8) as u8;
        packet[13] = vlan.0 as u8;
    }

    /// Removes VLAN tag from a packet
    ///
    /// # Arguments
    /// * `packet` - Mutable slice of the packet bytes
    ///
    /// # Returns
    /// * `Option<VlanId>` - Extracted VLAN ID if present
    pub fn untag_packet(&self, packet: &mut [u8]) -> Option<VlanId> {
        if packet.len() < 14 {
            return None;
        }

        // Example: read VLAN ID from bytes 12-13
        let vlan_id = ((packet[12] as u16) << 8) | packet[13] as u16;
        Some(VlanId(vlan_id))
    }
}
