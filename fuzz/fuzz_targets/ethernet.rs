#![no_main]
use libfuzzer_sys::fuzz_target;
use dataplane_rs::features::forwarding::types::EthernetFrame;

fuzz_target!(|data: &[u8]| {
    if let Ok(_frame) = EthernetFrame::try_from(data) {
        // optionally perform property checks
    }
});
