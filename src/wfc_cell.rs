use crate::common::Position;
use crate::factorio;
use crate::quantic;
use crate::quantic::Quantic;

pub enum BuildingCollapsedState {
    Collapsed(factorio::Building),
    Quantic(quantic::Building),
}

pub struct Cell {
    building: BuildingCollapsedState,
    production: Vec<factorio::Item>,
    demand: Vec<factorio::Item>,
    ground: factorio::Tile,
}

impl Cell {
    pub fn new(
        ground: factorio::Tile,
        pos: crate::common::Position,
        grid_size: crate::common::Size,
    ) -> Self {
        use quantic::Quantic;
        let mut buildings = quantic::Building::all(pos, grid_size);
        buildings
            .constrain(crate::constraint::Constraint::Ground(ground))
            .unwrap();
        Cell {
            building: BuildingCollapsedState::Quantic(buildings),
            production: vec![],
            demand: vec![],
            ground,
        }
    }

    pub fn is_collapsed(&self) -> bool {
        match self.building {
            BuildingCollapsedState::Collapsed(_) => true,
            BuildingCollapsedState::Quantic(_) => false,
        }
    }

    pub fn entropy(&self) -> f32 {
        match &self.building {
            BuildingCollapsedState::Collapsed(_) => 0.0,
            BuildingCollapsedState::Quantic(building) => building.entropy(),
        }
    }

    pub fn collapse(&mut self) -> Result<factorio::Building, quantic::Error> {
        match &self.building {
            BuildingCollapsedState::Collapsed(_) => Err(quantic::Error::AlreadyCollapsed),
            BuildingCollapsedState::Quantic(buildings) => buildings.collapse(),
        }
    }

    pub fn constrain(
        &mut self,
        constraint: crate::constraint::Constraint,
    ) -> Result<(), crate::quantic::Error> {
        match &mut self.building {
            BuildingCollapsedState::Collapsed(_building) => Ok(()), //building.check_constrain(constraint),
            BuildingCollapsedState::Quantic(buildings) => buildings.constrain(constraint),
        }
    }
}

impl crate::grid::GridElement for Cell {
    fn size(&self) -> crate::common::Size {
        match &self.building {
            BuildingCollapsedState::Collapsed(building) => building.size(),
            BuildingCollapsedState::Quantic(_) => crate::common::Size::new(1, 1).unwrap(),
        }
    }
    fn display_char(&self) -> char {
        match &self.building {
            BuildingCollapsedState::Collapsed(building) => building.display_char(),
            BuildingCollapsedState::Quantic(_) => '?',
        }
    }
}

impl crate::grid::Grid<Cell> {
    pub fn display_entropy(&self) {
        let mut result = String::with_capacity(self.width() * self.height());
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Position::new(x, y);
                match self.get(pos) {
                    Some(data) => result.push_str(&format!("{:<5.1} ", data.entropy())),
                    None => result.push(' '),
                }
            }
            result.push('\n');
        }
        println!("{result}");
    }

    pub fn collapse_lowest_entropy(&mut self) -> Result<bool, crate::quantic::Error> {
        let min_entropy = self
            .cells_mut()
            .filter(|(_, cell)| !cell.is_collapsed())
            .min_by(|(_, c1), (_, c2)| c1.entropy().total_cmp(&c2.entropy()));

        let (collapsed_position, min_entropy) = match min_entropy {
            Some(cell) => cell,
            None => return Ok(true),
        };

        let new_building = min_entropy.collapse()?;
        min_entropy.building = BuildingCollapsedState::Collapsed(new_building.clone());

        let constraint_result = self
            .cells_mut()
            .map(|(own_pos, cell)| {
                cell.constrain(crate::constraint::Constraint::BuildingPlaced {
                    building: &new_building,
                    at: collapsed_position,
                    own_pos,
                })
            })
            .collect::<Result<(), _>>();

        constraint_result?;

        Ok(false)
    }
}
