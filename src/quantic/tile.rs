use crate::quantic;

/// Supperposed state for a tile within the grid.
pub struct Tile<'lib> {
    buildings: Vec<quantic::Building<'lib>>,
}

impl<'lib> Tile<'lib> {
    pub fn all(
        library: &'lib crate::library::Library,
        grid_pos: crate::common::Position,
        grid_size: crate::common::Size,
    ) -> Self {
        let buildings = library
            .buildings()
            .map(|building| quantic::Building::new(building, grid_pos, grid_size))
            .collect::<Vec<_>>();

        Tile { buildings }
    }

    pub fn constrain_to_complete_building(
        &mut self,
        offset: crate::common::Position,
        building: &crate::library::Building,
    ) -> Result<(), quantic::Error> {
        self.buildings
            .retain_mut(|b| b.constrain_to_complete_building(offset, building).is_ok());

        if self.buildings.is_empty() {
            Err(quantic::Error::TooMuchConstraints)
        } else {
            Ok(())
        }
    }
}

impl<'lib> quantic::Quantic for Tile<'lib> {
    type Collapsed = crate::grid::Tile<'lib>;

    fn entropy(&self) -> f32 {
        self.buildings
            .iter()
            .map(|building| building.entropy())
            .sum()
    }

    fn collapse(&self) -> Result<Self::Collapsed, quantic::Error> {
        Ok(self
            .buildings
            .iter()
            .next()
            .map(|building| building.collapse())
            .ok_or(quantic::Error::EmptyCantCollapse)??)
    }
}
