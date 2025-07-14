use crate::common;
use crate::library;
use crate::quantic;
use quantic::Quantic;

/// a collapsed tile within the grid.
pub struct Tile<'lib> {
    building: &'lib library::Building,
    properties: Vec<library::BuildingPropertyValue>,
    offset: common::Position,
}

impl<'lib> Tile<'lib> {
    pub fn new(
        building: &'lib library::Building,
        properties: Vec<library::BuildingPropertyValue>,
        offset: common::Position,
    ) -> Self {
        Tile {
            building,
            properties,
            offset,
        }
    }

    pub fn building(&self) -> &library::Building {
        self.building
    }

    pub fn offset(&self) -> crate::common::Position {
        self.offset
    }
}

pub enum TileState<'lib> {
    Collapsed(Tile<'lib>),
    Supperposed(crate::quantic::Tile<'lib>),
}

impl<'lib> TileState<'lib> {
    pub fn pixel(&self) -> common::Pixel {
        match self {
            TileState::Collapsed(tile) => tile.building.display_char(tile.offset),
            TileState::Supperposed(_) => {
                common::Pixel::new('?', common::Color::Red, common::Color::Reset)
            }
        }
    }

    pub fn collapsed(&self) -> bool {
        match self {
            TileState::Collapsed(_) => true,
            TileState::Supperposed(_) => false,
        }
    }

    pub fn entropy(&self) -> f32 {
        match self {
            TileState::Collapsed(_) => 0.0,
            TileState::Supperposed(quantic) => quantic.entropy(),
        }
    }

    pub fn collapse(&mut self) -> Result<(), crate::quantic::Error> {
        match self {
            TileState::Collapsed(_) => Err(crate::quantic::Error::AlreadyCollapsed),
            TileState::Supperposed(quantic) => {
                *self = TileState::Collapsed(quantic.collapse()?);
                Ok(())
            }
        }
    }

    pub fn constrain_to_complete_building(
        &mut self,
        offset: common::Position,
        building: &crate::library::Building,
    ) -> Result<(), crate::quantic::Error> {
        match self {
            TileState::Collapsed(_) => Ok(()), // Mmh ?
            TileState::Supperposed(quantic) => {
                quantic.constrain_to_complete_building(offset, building)
            }
        }
    }
}

pub struct Grid<'lib> {
    width: usize,
    height: usize,
    cells: Vec<TileState<'lib>>,
}

impl<'lib> Grid<'lib> {
    pub fn from_fn<F: Fn(crate::common::Position) -> TileState<'lib>>(
        width: usize,
        height: usize,
        f: F,
    ) -> Self {
        let mut cells = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                cells.push(f(common::Position::new(x, y)));
            }
        }
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn collapse_once(&mut self) -> Result<bool, quantic::Error> {
        let min_entropy_index = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| !cell.collapsed())
            .min_by(|(_, c1), (_, c2)| c1.entropy().total_cmp(&c2.entropy()));

        let (collapse_index, _) = match min_entropy_index {
            Some(cell) => cell,
            None => return Ok(false),
        };

        let (first, rest) = self.cells.split_at_mut(collapse_index);
        let (to_collapse, end) = rest.split_at_mut(1);

        to_collapse[0].collapse()?;
        let (collapsed_building, collapsed_offset) = match &to_collapse[0] {
            TileState::Collapsed(building) => (building.building(), building.offset()),
            _ => unreachable!(),
        };

        let collapsed_building_origin = common::Position::new(
            collapse_index % self.width - collapsed_offset.x,
            collapse_index / self.width - collapsed_offset.y,
        );
        let size = collapsed_building.size();

        let other_cell_iter = first
            .iter_mut()
            .enumerate()
            .chain(end.iter_mut().enumerate().map(|(i, c)| (i + 1, c)))
            .map(|(index, cell)| {
                (
                    common::Position::new(index % self.width, index / self.width),
                    cell,
                )
            });

        for (other_position, other_cell) in other_cell_iter {
            let offset = collapsed_building_origin + other_position;
            if other_position == collapsed_offset {
                continue;
            }
            match other_cell.constrain_to_complete_building(other_position, collapsed_building) {
                Ok(_) => {}
                Err(_) => break,
            }
        }

        Ok(true)
    }

    pub fn display(&self) {
        use crate::common::Color;
        use std::fmt::Write;

        let mut result = String::new();
        let mut foreground = Color::White;
        let mut background = Color::Black;

        write!(
            result,
            "{}",
            crossterm::style::SetBackgroundColor(background)
        )
        .unwrap();
        write!(
            result,
            "{}",
            crossterm::style::SetForegroundColor(foreground)
        )
        .unwrap();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let pixel = match self.cells.get(index) {
                    Some(data) => data.pixel(),
                    None => unreachable!(),
                };
                if pixel.foreground() != foreground {
                    foreground = pixel.foreground();
                    write!(
                        result,
                        "{}",
                        crossterm::style::SetForegroundColor(foreground)
                    )
                    .unwrap();
                }
                if pixel.background() != background {
                    background = pixel.background();
                    write!(
                        result,
                        "{}",
                        crossterm::style::SetBackgroundColor(background)
                    )
                    .unwrap();
                }
                result.push(pixel.value());
            }
            result.push('\n');
        }

        write!(
            result,
            "{}",
            crossterm::style::SetBackgroundColor(Color::Reset)
        )
        .unwrap();
        write!(
            result,
            "{}",
            crossterm::style::SetForegroundColor(Color::Reset)
        )
        .unwrap();

        println!("{result}");
    }
}
