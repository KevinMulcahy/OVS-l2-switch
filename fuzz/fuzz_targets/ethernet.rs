#![no_main]
use libfuzzer_sys::fuzz_target;
use dataplane::features::forwarding::types::EthernetFrame;

fuzz_target!(|data: &[u8]| {
    // Try parsing as an Ethernet frame.
    let _ = EthernetFrame::try_from(data);
});
