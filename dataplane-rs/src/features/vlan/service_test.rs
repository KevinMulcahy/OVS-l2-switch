// dataplane-rs/src/features/vlan/service_test.rs
use crate::features::vlan::{service::VlanService, types::VlanId};

#[test]
fn test_tag_packet() {
    let service = VlanService;
    let mut packet = vec![0u8; 64];

    service.tag_packet(&mut packet[..], VlanId(100));

    let actual = ((packet[12] as u16) << 8) | (packet[13] as u16);
    assert_eq!(actual, 100);
}

#[test]
fn test_untag_packet() {
    let service = VlanService;
    let mut packet = vec![0u8; 64];

    packet[12] = 0;
    packet[13] = 100;

    let vlan = service.untag_packet(&mut packet[..]);
    assert_eq!(vlan, Some(VlanId(100)));
}

#[test]
fn test_untag_too_short() {
    let service = VlanService;
    let mut small = [0u8; 10];
    assert!(service.untag_packet(&mut small[..]).is_none());
}
