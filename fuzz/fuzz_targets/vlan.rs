#![no_main]
use libfuzzer_sys::fuzz_target;
use dataplane::features::forwarding::types::VlanTag;

fuzz_target!(|data: &[u8]| {
    let _ = VlanTag::try_from(data);
});
