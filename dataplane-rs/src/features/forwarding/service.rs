use crate::features::forwarding::types::{NetIf, ForwardingEntry, MacAddress, VlanId};
use crate::features::forwarding::internal::pipeline::Pipeline;
use anyhow::Result;

pub struct ForwardingService;

impl ForwardingService {
    pub fn new() -> Self {
        Self
    }

    pub fn setup_pipeline(&self, input: &str, output: &str) -> Result<()> {
        // ✅ Use the constructor instead of raw struct literal
        let in_if = NetIf::new(input, 0);
        let out_if = NetIf::new(output, 1);

        // Example ForwardingEntry just for testing wiring
        let _entry = ForwardingEntry::new(
            MacAddress([0, 1, 2, 3, 4, 5]),
            VlanId(1),
            out_if.index,
        );

        println!("Pipeline setup: {:?} -> {:?}", in_if, out_if);
        Ok(())
    }
}

