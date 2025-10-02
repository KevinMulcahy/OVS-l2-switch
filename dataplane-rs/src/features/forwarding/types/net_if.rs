#[derive(Debug, Clone)]
pub struct NetIf {
    pub name: String,
    pub index: u32,
}

impl NetIf {
    pub fn new(name: &str, index: u32) -> Self {
        Self {
            name: name.to_string(),
            index,
        }
    }
}
