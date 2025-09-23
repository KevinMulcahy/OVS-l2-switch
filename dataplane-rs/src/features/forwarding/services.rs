//! Forwarding service placeholder

use crate::features::forwarding::types::{MacAddress, VlanId};

/// Stub ForwardingService for CI compilation
pub struct ForwardingService;

impl ForwardingService {
    pub fn new() -> Self {
        Self
    }

    pub fn forward(&self, _mac: MacAddress, _vlan: VlanId) -> Result<(), &'static str> {
        // Placeholder: will implement actual logic later
        Ok(())
    }
}
