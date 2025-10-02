pub mod forwarding_entry;
pub mod mac_address;
pub mod net_if;
pub mod vlan;
pub mod ethernet_frame;

pub use forwarding_entry::ForwardingEntry;
pub use mac_address::MacAddress;
pub use net_if::NetIf;
pub use vlan::{VlanId, VlanTag}; // ✅ matches your vlan.rs
pub use ethernet_frame::EthernetFrame;
