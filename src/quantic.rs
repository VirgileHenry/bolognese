mod building;
mod common;
mod offset;
mod tile;

pub use building::*;
pub use common::*;
pub use offset::*;
pub use tile::*;

pub trait Quantic {
    type Collapsed;
    fn entropy(&self) -> f32;
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
