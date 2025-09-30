#![no_main]

use libfuzzer_sys::fuzz_target;
use dataplane_rs::features::forwarding::types::VlanTag;

fuzz_target!(|data: &[u8]| {
    // construct a VLAN tag from the first byte
    let _tag = VlanTag(data.get(0).copied().unwrap_or(0) as u16);
});
