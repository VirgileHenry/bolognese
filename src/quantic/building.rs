use crate::library;
use crate::quantic;

pub struct Building<'lib> {
    building: &'lib library::Building,
    offset: quantic::Offset,
}

impl<'lib> Building<'lib> {
    pub fn new(
        building: &'lib library::Building,
        grid_pos: crate::common::Position,
        grid_size: crate::common::Size,
    ) -> Building<'lib> {
        let offset = quantic::all_in_rect(building.size(), grid_pos, grid_size);
        Building { building, offset }
    }

    pub fn constrain_to_complete_building(
        &mut self,
        offset: crate::common::Position,
        building: &crate::library::Building,
    ) -> Result<(), quantic::Error> {
        if self.building.name() != building.name() {
            return Err(quantic::Error::TooMuchConstraints);
        }
        self.offset.retain(|off| *off == offset);
        if self.offset.is_empty() {
            return Err(quantic::Error::TooMuchConstraints);
        }

        Ok(())
    }
}

impl<'lib> quantic::Quantic for Building<'lib> {
    type Collapsed = crate::grid::Tile<'lib>;

    fn entropy(&self) -> f32 {
        self.offset.entropy()
    }

    fn collapse(&self) -> Result<Self::Collapsed, quantic::Error> {
        Ok(crate::grid::Tile::new(
            self.building,
            vec![],
            self.offset.collapse()?,
        ))
    }
}
