#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardingEntry {
    pub mac: MacAddress,
    pub vlan: Vlan,
    pub port: u16,
}
