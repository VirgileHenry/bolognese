use super::Quantic;
use crate::common::{Position, Size};

pub type TileOffset = Vec<Position>;

pub fn all_in_rect(rect: Size, pos: Position, grid_size: Size) -> TileOffset {
    let width = rect.width.get();
    let height = rect.width.get();
    let mut result = Vec::with_capacity(width * height);

    let start_x = (pos.x + rect.width.get()).saturating_sub(grid_size.width.get());
    let start_y = (pos.y + rect.height.get()).saturating_sub(grid_size.height.get());
    let end_x = width.min(pos.x + 1);
    let end_y = height.min(pos.y + 1);

    for x in start_x..end_x {
        for y in start_y..end_y {
            result.push(Position::new(x, y));
        }
    }

    result
}

impl Quantic for TileOffset {
    type Collapsed = Position;

    fn all(_: crate::common::Position, _: crate::common::Size) -> Self {
        unreachable!("Use all_in_rect instead!")
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
