// src/features/vlan/mod.rs

pub mod service;
/// VLAN feature module.
/// Provides tagging and untagging logic for IEEE 802.1Q.
pub mod types;

// Public API surface
pub use service::VlanService;
