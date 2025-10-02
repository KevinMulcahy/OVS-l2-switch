use crate::features::forwarding::internal::pipeline::Pipeline;
use crate::features::forwarding::types::{ForwardingEntry, MacAddress, NetIf, VlanId};
use anyhow::Result;

pub struct ForwardingService;

impl ForwardingService {
    pub fn new() -> Self {
        ForwardingService
    }

    pub fn setup_pipeline(&self, input: &str, output: &str) -> Result<Pipeline> {
        let in_if = NetIf::new(input, 0);
        let out_if = NetIf::new(output, 1);

        // ✅ Cast index (u32 → u16) when creating ForwardingEntry
        let _entry = ForwardingEntry::new(
            MacAddress([0, 1, 2, 3, 4, 5]),
            VlanId(1),
            out_if.index as u16,
        );

        println!("Pipeline setup: {:?} -> {:?}", in_if, out_if);

        // ✅ Actually return a Pipeline instead of ()
        Ok(Pipeline {
            input: in_if,
            output: out_if,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_pipeline_creates_pipeline() {
        let svc = ForwardingService::new();
        let pipeline = svc
            .setup_pipeline("eth0", "eth1")
            .expect("pipeline should build");

        assert_eq!(pipeline.input.name, "eth0");
        assert_eq!(pipeline.input.index, 0);
        assert_eq!(pipeline.output.name, "eth1");
        assert_eq!(pipeline.output.index, 1);
    }
}
