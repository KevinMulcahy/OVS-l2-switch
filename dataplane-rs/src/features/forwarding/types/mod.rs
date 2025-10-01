// src/features/forwarding/types/mod.rs

pub mod forwarding_entry;
pub mod mac_address;
pub mod vlan;
// pub mod other_type; // add more as needed
pub mod net_if;

// Re-export for easier access
pub use forwarding_entry::ForwardingEntry;
pub use mac_address::MacAddress;
pub use vlan::Vlan;
// pub use other_type::OtherType; // add more as needed
pub use net_if::NetIf;
