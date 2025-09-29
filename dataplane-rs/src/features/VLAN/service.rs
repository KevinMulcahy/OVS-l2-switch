// src/features/vlan/service.rs
use super::types::VlanId;

/// Service for VLAN tagging/untagging.
pub struct VlanService;

impl VlanService {
    pub fn new() -> Self {
        VlanService
    }

    pub fn tag_packet(&self, _packet: &mut Vec<u8>, _vlan: VlanId) {
        // TODO: implement tagging
    }

    pub fn untag_packet(&self, _packet: &mut Vec<u8>) -> Option<VlanId> {
        // TODO: implement untagging
        None
    }
}
