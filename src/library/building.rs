#[derive(Debug)]
pub enum BuildingPropertyKind {
    Orientation,
    Direction,
    Recipe,
    Item,
    Liquid,
}

#[derive(Debug)]
pub enum BuildingPropertyValue {
    Orientation(crate::factorio::Orientation),
    Direction(crate::factorio::Direction),
    Recipe(String),
    Item(String),
    Liquid(String),
}

#[derive(Debug)]
pub struct Building {
    name: String,
    properties: Vec<BuildingPropertyKind>,
    size: crate::common::Size,
    graphics: Vec<crate::common::Pixel>,
}

impl Building {
    pub fn new(
        name: String,
        properties: Vec<BuildingPropertyKind>,
        size: crate::common::Size,
        graphics: Vec<crate::common::Pixel>,
    ) -> Self {
        Self {
            name,
            properties,
            size,
            graphics,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn properties(&self) -> &[BuildingPropertyKind] {
        &self.properties
    }

    pub fn size(&self) -> crate::common::Size {
        self.size
    }

    pub fn display_char(&self, offset: crate::common::Position) -> crate::common::Pixel {
        let index = offset.y * self.size.width.get() + offset.x;
        self.graphics[index]
    }
}
