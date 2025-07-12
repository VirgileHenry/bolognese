#[derive(Debug)]
pub struct Item {
    id: usize,
    name: String,
}

impl Item {
    pub fn new(name: String) -> Self {
        Item { id: 0, name }
    }
}
