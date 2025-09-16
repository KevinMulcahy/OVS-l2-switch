#![no_main]
use libfuzzer_sys::fuzz_target;
use dataplane_rs::features::forwarding::types::VlanTag;

fuzz_target!(|data: &[u8]| {
    let _ = VlanTag::try_from(data);
});