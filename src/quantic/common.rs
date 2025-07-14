use super::Quantic;
use crate::factorio;

pub type Direction = Vec<factorio::Direction>;

impl Quantic for Direction {
    type Collapsed = factorio::Direction;

    fn entropy(&self) -> f32 {
        self.len() as f32
    }

    fn collapse(&self) -> Result<Self::Collapsed, super::Error> {
        self.iter()
            .next()
            .cloned()
            .ok_or(super::Error::EmptyCantCollapse)
    }
}

pub type Orientation = Vec<factorio::Orientation>;

impl Quantic for Orientation {
    type Collapsed = factorio::Orientation;

    fn entropy(&self) -> f32 {
        self.len() as f32
    }

    fn collapse(&self) -> Result<Self::Collapsed, super::Error> {
        self.iter()
            .next()
            .cloned()
            .ok_or(super::Error::EmptyCantCollapse)
    }
}
