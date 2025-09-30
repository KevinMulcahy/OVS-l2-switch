// src/features/forwarding/service.rs

/// Service layer for forwarding feature.
/// Handles MAC learning and packet forwarding logic.
pub struct ForwardingService;

impl Default for ForwardingService {
    fn default() -> Self {
        Self::new()
    }
}

impl ForwardingService {
    pub fn new() -> Self {
        ForwardingService
    }

    pub fn process_packet(&self, _packet: &[u8]) {
        // TODO: implement forwarding logic
    }
}
