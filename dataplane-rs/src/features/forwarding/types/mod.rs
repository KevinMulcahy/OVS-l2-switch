pub mod forwarding_entry;
pub mod mac_address;
pub mod vlan;
pub mod net_if;

pub use forwarding_entry::ForwardingEntry;
pub use mac_address::MacAddress;
pub use vlan::{VlanId, VlanTag}; // ✅ matches your vlan.rs
pub use net_if::NetIf;

