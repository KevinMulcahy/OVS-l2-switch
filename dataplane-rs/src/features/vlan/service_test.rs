// dataplane-rs/src/features/vlan/service_test.rs

use crate::features::vlan::{service::VlanService, types::VlanId};

#[test]
fn test_tag_packet() {
    let vlan_service = VlanService;
    let mut packet = vec![0u8; 64]; // Vec<u8> for easy allocation

    // Pass a mutable slice to tag_packet
    vlan_service.tag_packet(&mut packet[..], VlanId(100));

    // Verify VLAN bytes were inserted at position 12-13
    let expected_vlan = 100u16;
    let actual_vlan = ((packet[12] as u16) << 8) | packet[13] as u16;
    assert_eq!(actual_vlan, expected_vlan);
}

#[test]
fn test_untag_packet() {
    let vlan_service = VlanService;
    let mut packet = vec![0u8; 64];

    // Pre-tag the packet for testing untag
    packet[12] = 0;
    packet[13] = 100;

    // Pass a mutable slice to untag_packet
    let vlan_id = vlan_service.untag_packet(&mut packet[..]);
    assert_eq!(vlan_id, Some(VlanId(100)));
}

#[test]
fn test_untag_packet_too_short() {
    let vlan_service = VlanService;
    let mut short_packet = [0u8; 10]; // fixed-size array shorter than Ethernet header

    // Should return None for short packets
    let vlan_id = vlan_service.untag_packet(&mut short_packet[..]);
    assert!(vlan_id.is_none());
}
