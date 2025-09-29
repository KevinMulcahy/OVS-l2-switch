// src/features/vlan/mod.rs

/// VLAN feature module.
/// Provides tagging and untagging logic for IEEE 802.1Q.
pub mod types;
pub mod service;

// Public API surface
pub use service::VlanService;
