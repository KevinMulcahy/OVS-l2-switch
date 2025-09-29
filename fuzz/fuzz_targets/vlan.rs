use dataplane_rs::features::forwarding::types::VlanTag;

fuzz_target!(|data: &[u8]| {
    let _tag = VlanTag(data.get(0).cloned().unwrap_or(0) as u16);
});
