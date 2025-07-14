use super::Quantic;
use crate::common::{Position, Size};

pub type Offset = Vec<Position>;

pub fn all_in_rect(rect: Size, grid_pos: Position, grid_size: Size) -> Offset {
    let width = rect.width.get();
    let height = rect.width.get();
    let mut result = Vec::with_capacity(width * height);

    let start_x = (grid_pos.x + rect.width.get()).saturating_sub(grid_size.width.get());
    let start_y = (grid_pos.y + rect.height.get()).saturating_sub(grid_size.height.get());
    let end_x = width.min(grid_pos.x + 1);
    let end_y = height.min(grid_pos.y + 1);

    for x in start_x..end_x {
        for y in start_y..end_y {
            result.push(Position::new(x, y));
        }
    }

    result
}

impl Quantic for Offset {
    type Collapsed = Position;

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
