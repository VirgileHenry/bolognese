#[derive(Debug)]
pub struct Item {
    name: String,
    kind: crate::factorio::ItemKind,
}

impl Item {
    pub fn new(name: String, kind: crate::factorio::ItemKind) -> Self {
        Item { name, kind }
    }
}

impl Item {
    pub fn name(&self) -> &str {
        &self.name
    }
}
