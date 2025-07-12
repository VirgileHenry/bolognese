#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Input,
    Output,
}
