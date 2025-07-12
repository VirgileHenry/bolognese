#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

impl std::ops::Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: core::num::NonZeroUsize,
    pub height: core::num::NonZeroUsize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Option<Self> {
        Some(Size {
            width: core::num::NonZeroUsize::new(width)?,
            height: core::num::NonZeroUsize::new(height)?,
        })
    }

    pub fn bounding_square(&self) -> Self {
        Size {
            width: self.width.max(self.height),
            height: self.width.max(self.height),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub top_left: Position,
    pub size: Size,
}

impl Rect {
    pub fn new(top_left: Position, size: Size) -> Rect {
        Rect { top_left, size }
    }

    pub fn contains(&self, pos: Position) -> bool {
        self.top_left.x <= pos.x
            && pos.x < self.top_left.x + self.size.width.get()
            && self.top_left.y <= pos.y
            && pos.y < self.top_left.y + self.size.height.get()
    }
}
