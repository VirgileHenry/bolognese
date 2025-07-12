use super::Quantic;
use crate::factorio;

pub type Direction = Vec<factorio::Direction>;

impl Quantic for Direction {
    type Collapsed = factorio::Direction;

    fn all(_: crate::common::Position, _: crate::common::Size) -> Self {
        vec![factorio::Direction::Input, factorio::Direction::Output]
    }

    fn entropy(&self) -> f32 {
        self.len() as f32
    }

    fn collapse(&self) -> Result<Self::Collapsed, super::Error> {
        self.iter()
            .next()
            .cloned()
            .ok_or(super::Error::EmptyCantCollapse)
    }

    fn can_collapse_to(&self, potential_collapse: &Self::Collapsed) -> bool {
        self.iter().any(|item| item == potential_collapse)
    }
}

pub type Orientation = Vec<factorio::Orientation>;

impl Quantic for Orientation {
    type Collapsed = factorio::Orientation;

    fn all(_: crate::common::Position, _: crate::common::Size) -> Self {
        vec![
            factorio::Orientation::North,
            factorio::Orientation::East,
            factorio::Orientation::South,
            factorio::Orientation::West,
        ]
    }

    fn entropy(&self) -> f32 {
        self.len() as f32
    }

    fn collapse(&self) -> Result<Self::Collapsed, super::Error> {
        self.iter()
            .next()
            .cloned()
            .ok_or(super::Error::EmptyCantCollapse)
    }

    fn can_collapse_to(&self, potential_collapse: &Self::Collapsed) -> bool {
        self.iter().any(|item| item == potential_collapse)
    }
}
