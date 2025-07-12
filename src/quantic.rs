mod building;
mod common;
mod item;
mod recipe;
mod tile_offset;

pub use building::*;
pub use common::*;
pub use item::*;
pub use recipe::*;
pub use tile_offset::*;

pub trait Quantic: Sized {
    type Collapsed;
    fn all(pos: crate::common::Position, grid_size: crate::common::Size) -> Self;
    fn entropy(&self) -> f32;
    fn constrain(&mut self, _: crate::constraint::Constraint<Self>) -> Result<(), Error> {
        Ok(())
    }
    fn collapse(&self) -> Result<Self::Collapsed, Error>;
}

#[derive(Debug)]
pub enum Error {
    TooMuchConstraints,
    EmptyCantCollapse,
    AlreadyCollapsed,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::TooMuchConstraints => write!(f, "Too much constraints!"),
            Error::EmptyCantCollapse => write!(f, "Can't collapse state with no possible values!"),
            Error::AlreadyCollapsed => write!(f, "Can't collapse from an already collapsed state!"),
        }
    }
}
