// src/features/forwarding/service.rs
use crate::features::forwarding::internal::pipeline::Forwarder;
use crate::features::forwarding::types::NetIf;
use anyhow::Result;

/// Public API for forwarding
pub struct ForwardingService;

impl ForwardingService {
    /// Start basic forwarding between two interfaces
    pub fn start_forwarding(input: &str, output: &str) -> Result<()> {
        let fwd = Forwarder::new(
            NetIf { name: input.to_string() },
            NetIf { name: output.to_string() },
        );
        fwd.run()?;
        Ok(())
    }
}
