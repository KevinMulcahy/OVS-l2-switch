use dataplane_rs::features::forwarding::types::EthernetFrame;
use std::convert::TryFrom;

fuzz_target!(|data: &[u8]| {
    let _ = EthernetFrame::try_from(data);
});
