use crate::common::Size;
use crate::constraint::Constraint;
use crate::factorio;
use crate::quantic;
use crate::quantic::Quantic;

#[derive(Debug, Clone)]
pub enum BuildingTile {
    ElectricMiningDrill {
        orientation: quantic::Orientation,
        offset: quantic::TileOffset,
    },
    Pumpjack {
        orientation: quantic::Orientation,
        offset: quantic::TileOffset,
    },
    OffshorePump {
        orientation: quantic::Orientation,
        offset: quantic::TileOffset,
    },
    StoneFurnace {
        recipe: quantic::FurnaceRecipe,
        offset: quantic::TileOffset,
    },
    OilRefinery {
        orientation: quantic::Orientation,
        recipe: quantic::RefineryRecipe,
        offset: quantic::TileOffset,
    },
    ChemicalPlant {
        orientation: quantic::Orientation,
        recipe: quantic::ChemicalRecipe,
        offset: quantic::TileOffset,
    },
    AssemblingMachine {
        recipe: quantic::AssemblerRecipe,
        offset: quantic::TileOffset,
    },
    TransportBelt {
        orientation: quantic::Orientation,
        item: quantic::Item,
    },
    UndergroundBelt {
        orientation: quantic::Orientation,
        direction: quantic::Direction,
        item: quantic::Item,
    },
    Pipe {
        liquid: quantic::Liquid,
    },
    PipeToGround {
        orientation: quantic::Orientation,
        liquid: quantic::Liquid,
    },
    Inserter {
        orientation: quantic::Orientation,
        item: quantic::Item,
    },
    LongHandedInserter {
        orientation: quantic::Orientation,
        item: quantic::Item,
    },
    Lab {
        offset: quantic::TileOffset,
    },
}

impl BuildingTile {
    fn constrain_ground(&mut self, ground: factorio::Tile) -> Result<(), quantic::Error> {
        match (self, ground) {
            (BuildingTile::OffshorePump { .. }, factorio::Tile::Water) => Ok(()),
            (_other_building, factorio::Tile::Water) => Err(quantic::Error::TooMuchConstraints),
            (BuildingTile::OffshorePump { .. }, _other_tile) => {
                Err(quantic::Error::TooMuchConstraints)
            }
            (BuildingTile::Pumpjack { .. }, factorio::Tile::CrudeOil) => Ok(()),
            (_other_building, factorio::Tile::CrudeOil) => Err(quantic::Error::TooMuchConstraints),
            (BuildingTile::Pumpjack { .. }, _other_tile) => Err(quantic::Error::TooMuchConstraints),
            (
                BuildingTile::ElectricMiningDrill { .. },
                factorio::Tile::Stone
                | factorio::Tile::CoalOre
                | factorio::Tile::IronOre
                | factorio::Tile::CopperOre,
            ) => Ok(()),
            (BuildingTile::ElectricMiningDrill { .. }, _other_tile) => {
                Err(quantic::Error::TooMuchConstraints)
            }
            _other => Ok(()),
        }
    }

    fn constrain_building_placed(
        &mut self,
        building: &factorio::Building,
        at: crate::common::Position,
        own_pos: crate::common::Position,
    ) -> Result<(), super::Error> {
        // if the building being placed forced me to be of this building kind, I shall ensure I am
        let building_offset = building.offset();
        let building_origin =
            crate::common::Position::new(at.x - building_offset.x, at.y - building_offset.y);
        let building_box = crate::common::Rect::new(building_origin, building.size());
        if building_box.contains(own_pos) {
            let required_offset = crate::common::Position::new(
                own_pos.x - building_origin.x,
                own_pos.y - building_origin.y,
            );
            let mut target_building = building.clone();
            target_building.set_offset(required_offset);
            if self.can_collapse_to(&target_building) {
                Ok(())
            } else {
                Err(super::Error::TooMuchConstraints)
            }
        } else {
            Ok(())
        }
    }
}

impl quantic::Quantic for BuildingTile {
    type Collapsed = factorio::Building;

    fn all(_: crate::common::Position, _: crate::common::Size) -> Self {
        panic!("Use quantic::Building::all")
    }

    fn entropy(&self) -> f32 {
        match self {
            BuildingTile::ElectricMiningDrill {
                orientation,
                offset,
            } => orientation.entropy() + offset.entropy(),
            BuildingTile::Pumpjack {
                orientation,
                offset,
            } => orientation.entropy() + offset.entropy(),
            BuildingTile::OffshorePump {
                orientation,
                offset,
            } => orientation.entropy() + offset.entropy(),
            BuildingTile::StoneFurnace { recipe, offset } => recipe.entropy() + offset.entropy(),
            BuildingTile::OilRefinery {
                orientation,
                recipe,
                offset,
            } => orientation.entropy() + recipe.entropy() + offset.entropy(),
            BuildingTile::ChemicalPlant {
                orientation,
                recipe,
                offset,
            } => orientation.entropy() + recipe.entropy() + offset.entropy(),
            BuildingTile::AssemblingMachine { recipe, offset } => {
                recipe.entropy() + offset.entropy()
            }
            BuildingTile::TransportBelt { orientation, item } => {
                orientation.entropy() + item.entropy()
            }
            BuildingTile::UndergroundBelt {
                orientation,
                direction,
                item,
            } => orientation.entropy() + direction.entropy() + item.entropy(),
            BuildingTile::Pipe { liquid } => liquid.entropy(),
            BuildingTile::PipeToGround {
                orientation,
                liquid,
            } => orientation.entropy() + liquid.entropy(),
            BuildingTile::Inserter { orientation, item } => orientation.entropy() + item.entropy(),
            BuildingTile::LongHandedInserter { orientation, item } => {
                orientation.entropy() + item.entropy()
            }
            BuildingTile::Lab { offset } => offset.entropy(),
        }
    }

    fn constrain(&mut self, constraint: Constraint<Self>) -> Result<(), quantic::Error> {
        match constraint {
            Constraint::Ground(ground) => self.constrain_ground(ground)?,
            Constraint::Forced(value) => match (self, value) {
                (
                    BuildingTile::ElectricMiningDrill {
                        orientation,
                        offset,
                    },
                    factorio::Building::ElectricMiningDrill {
                        orientation: collapse_orientation,
                        offset: collapse_offset,
                    },
                ) => {
                    orientation.constrain(Constraint::Forced(collapse_orientation))?;
                    offset.constrain(Constraint::Forced(collapse_offset))?;
                }
                (
                    BuildingTile::Pumpjack {
                        orientation,
                        offset,
                    },
                    factorio::Building::Pumpjack {
                        orientation: collapse_orientation,
                        offset: collapse_offset,
                    },
                ) => {
                    orientation.constrain(Constraint::Forced(collapse_orientation))?;
                    offset.constrain(Constraint::Forced(collapse_offset))?;
                }
                (
                    BuildingTile::OffshorePump {
                        orientation,
                        offset,
                    },
                    factorio::Building::OffshorePump {
                        orientation: collapse_orientation,
                        offset: collapse_offset,
                    },
                ) => {
                    orientation.constrain(Constraint::Forced(collapse_orientation))?;
                    offset.constrain(Constraint::Forced(collapse_offset))?;
                }
                (
                    BuildingTile::StoneFurnace { recipe, offset },
                    factorio::Building::StoneFurnace {
                        recipe: collapse_recipe,
                        offset: collapse_offset,
                    },
                ) => {
                    recipe.constrain(Constraint::Forced(collapse_recipe))?;
                    offset.constrain(Constraint::Forced(collapse_offset))?;
                }
                (
                    BuildingTile::OilRefinery {
                        orientation,
                        recipe,
                        offset,
                    },
                    factorio::Building::OilRefinery {
                        orientation: collapse_orientation,
                        recipe: collapse_recipe,
                        offset: collapse_offset,
                    },
                ) => {
                    orientation.constrain(Constraint::Forced(collapse_orientation))?;
                    recipe.constrain(Constraint::Forced(collapse_recipe))?;
                    offset.constrain(Constraint::Forced(collapse_offset))?;
                }
                (
                    BuildingTile::ChemicalPlant {
                        orientation,
                        recipe,
                        offset,
                    },
                    factorio::Building::ChemicalPlant {
                        orientation: collapse_orientation,
                        recipe: collapse_recipe,
                        offset: collapse_offset,
                    },
                ) => {
                    orientation.constrain(Constraint::Forced(collapse_orientation))?;
                    recipe.constrain(Constraint::Forced(collapse_recipe))?;
                    offset.constrain(Constraint::Forced(collapse_offset))?;
                }
                (
                    BuildingTile::AssemblingMachine { recipe, offset },
                    factorio::Building::AssemblingMachine {
                        recipe: collapse_recipe,
                        offset: collapse_offset,
                    },
                ) => {
                    recipe.constrain(Constraint::Forced(collapse_recipe))?;
                    offset.constrain(Constraint::Forced(collapse_offset))?;
                }
                (
                    BuildingTile::TransportBelt { orientation, item },
                    factorio::Building::TransportBelt {
                        orientation: collapse_orientation,
                        item: collapse_item,
                    },
                ) => {
                    orientation.constrain(Constraint::Forced(collapse_orientation))?;
                    item.constrain(Constraint::Forced(collapse_item))?;
                }
                (
                    BuildingTile::UndergroundBelt {
                        orientation,
                        direction,
                        item,
                    },
                    factorio::Building::UndergroundBelt {
                        orientation: collapse_orientation,
                        direction: collapse_direction,
                        item: collapse_item,
                    },
                ) => {
                    orientation.constrain(Constraint::Forced(collapse_orientation))?;
                    direction.constrain(Constraint::Forced(collapse_direction))?;
                    item.constrain(Constraint::Forced(collapse_item))?;
                }
                (
                    BuildingTile::Pipe { liquid },
                    factorio::Building::Pipe {
                        liquid: collapse_liquid,
                    },
                ) => {
                    liquid.constrain(Constraint::Forced(collapse_liquid))?;
                }
                (
                    BuildingTile::PipeToGround {
                        orientation,
                        liquid,
                    },
                    factorio::Building::PipeToGround {
                        orientation: collapse_orientation,
                        liquid: collapse_liquid,
                    },
                ) => {
                    orientation.constrain(Constraint::Forced(collapse_orientation))?;
                    liquid.constrain(Constraint::Forced(collapse_liquid))?;
                }
                (
                    BuildingTile::Inserter { orientation, item },
                    factorio::Building::Inserter {
                        orientation: collapse_orientation,
                        item: collapse_item,
                    },
                ) => {
                    orientation.constrain(Constraint::Forced(collapse_orientation))?;
                    item.constrain(Constraint::Forced(collapse_item))?;
                }
                (
                    BuildingTile::LongHandedInserter { orientation, item },
                    factorio::Building::LongHandedInserter {
                        orientation: collapse_orientation,
                        item: collapse_item,
                    },
                ) => {
                    orientation.constrain(Constraint::Forced(collapse_orientation))?;
                    item.constrain(Constraint::Forced(collapse_item))?;
                }
                (
                    BuildingTile::Lab { offset },
                    factorio::Building::Lab {
                        offset: collapse_offset,
                    },
                ) => {
                    offset.constrain(Constraint::Forced(collapse_offset))?;
                }
                _ => {}
            },
        };

        Ok(())
    }

    fn collapse(&self) -> Result<factorio::Building, quantic::Error> {
        Ok(match self {
            BuildingTile::ElectricMiningDrill {
                orientation,
                offset,
            } => factorio::Building::ElectricMiningDrill {
                orientation: orientation.collapse()?,
                offset: offset.collapse()?,
            },
            BuildingTile::Pumpjack {
                orientation,
                offset,
            } => factorio::Building::Pumpjack {
                orientation: orientation.collapse()?,
                offset: offset.collapse()?,
            },
            BuildingTile::OffshorePump {
                orientation,
                offset,
            } => factorio::Building::OffshorePump {
                orientation: orientation.collapse()?,
                offset: offset.collapse()?,
            },
            BuildingTile::StoneFurnace { recipe, offset } => factorio::Building::StoneFurnace {
                recipe: recipe.collapse()?,
                offset: offset.collapse()?,
            },
            BuildingTile::OilRefinery {
                orientation,
                recipe,
                offset,
            } => factorio::Building::OilRefinery {
                orientation: orientation.collapse()?,
                recipe: recipe.collapse()?,
                offset: offset.collapse()?,
            },
            BuildingTile::ChemicalPlant {
                orientation,
                recipe,
                offset,
            } => factorio::Building::ChemicalPlant {
                orientation: orientation.collapse()?,
                recipe: recipe.collapse()?,
                offset: offset.collapse()?,
            },
            BuildingTile::AssemblingMachine { recipe, offset } => {
                factorio::Building::AssemblingMachine {
                    recipe: recipe.collapse()?,
                    offset: offset.collapse()?,
                }
            }
            BuildingTile::TransportBelt { orientation, item } => {
                factorio::Building::TransportBelt {
                    orientation: orientation.collapse()?,
                    item: item.collapse()?,
                }
            }
            BuildingTile::UndergroundBelt {
                orientation,
                direction,
                item,
            } => factorio::Building::UndergroundBelt {
                orientation: orientation.collapse()?,
                direction: direction.collapse()?,
                item: item.collapse()?,
            },
            BuildingTile::Pipe { liquid } => factorio::Building::Pipe {
                liquid: liquid.collapse()?,
            },
            BuildingTile::PipeToGround {
                orientation,
                liquid,
            } => factorio::Building::PipeToGround {
                orientation: orientation.collapse()?,
                liquid: liquid.collapse()?,
            },
            BuildingTile::Inserter { orientation, item } => factorio::Building::Inserter {
                orientation: orientation.collapse()?,
                item: item.collapse()?,
            },
            BuildingTile::LongHandedInserter { orientation, item } => {
                factorio::Building::LongHandedInserter {
                    orientation: orientation.collapse()?,
                    item: item.collapse()?,
                }
            }
            BuildingTile::Lab { offset } => factorio::Building::Lab {
                offset: offset.collapse()?,
            },
        })
    }
}

pub type Building = Vec<BuildingTile>;

impl quantic::Quantic for Building {
    type Collapsed = factorio::Building;

    fn all(pos: crate::common::Position, grid_size: crate::common::Size) -> Self {
        vec![
            BuildingTile::ElectricMiningDrill {
                orientation: quantic::Orientation::all(pos, grid_size),
                offset: quantic::all_in_rect(Size::new(3, 3).unwrap(), pos, grid_size),
            },
            BuildingTile::Pumpjack {
                orientation: quantic::Orientation::all(pos, grid_size),
                offset: quantic::all_in_rect(Size::new(3, 3).unwrap(), pos, grid_size),
            },
            BuildingTile::OffshorePump {
                orientation: quantic::Orientation::all(pos, grid_size),
                offset: quantic::all_in_rect(Size::new(2, 2).unwrap(), pos, grid_size),
            },
            BuildingTile::StoneFurnace {
                recipe: quantic::FurnaceRecipe::all(pos, grid_size),
                offset: quantic::all_in_rect(Size::new(2, 2).unwrap(), pos, grid_size),
            },
            BuildingTile::OilRefinery {
                orientation: quantic::Orientation::all(pos, grid_size),
                recipe: quantic::RefineryRecipe::all(pos, grid_size),
                offset: quantic::all_in_rect(Size::new(4, 4).unwrap(), pos, grid_size),
            },
            BuildingTile::ChemicalPlant {
                orientation: quantic::Orientation::all(pos, grid_size),
                recipe: quantic::ChemicalRecipe::all(pos, grid_size),
                offset: quantic::all_in_rect(Size::new(2, 2).unwrap(), pos, grid_size),
            },
            BuildingTile::AssemblingMachine {
                recipe: quantic::AssemblerRecipe::all(pos, grid_size),
                offset: quantic::all_in_rect(Size::new(3, 3).unwrap(), pos, grid_size),
            },
            BuildingTile::TransportBelt {
                orientation: quantic::Orientation::all(pos, grid_size),
                item: quantic::Item::all(pos, grid_size),
            },
            BuildingTile::UndergroundBelt {
                orientation: quantic::Orientation::all(pos, grid_size),
                direction: quantic::Direction::all(pos, grid_size),
                item: quantic::Item::all(pos, grid_size),
            },
            BuildingTile::Pipe {
                liquid: quantic::Liquid::all(pos, grid_size),
            },
            BuildingTile::PipeToGround {
                orientation: quantic::Orientation::all(pos, grid_size),
                liquid: quantic::Liquid::all(pos, grid_size),
            },
            BuildingTile::Inserter {
                orientation: quantic::Orientation::all(pos, grid_size),
                item: quantic::Item::all(pos, grid_size),
            },
            BuildingTile::LongHandedInserter {
                orientation: quantic::Orientation::all(pos, grid_size),
                item: quantic::Item::all(pos, grid_size),
            },
            BuildingTile::Lab {
                offset: quantic::all_in_rect(Size::new(3, 3).unwrap(), pos, grid_size),
            },
        ]
    }

    fn entropy(&self) -> f32 {
        self.iter().map(|b| b.entropy()).sum()
    }

    fn constrain(&mut self, constraint: Constraint<Self>) -> Result<(), quantic::Error> {
        match constraint {
            Constraint::Ground(ground) => {
                self.retain_mut(|building| building.constrain(constraint).is_ok())
            }
            Constraint::Forced(values) => self.iter().zip(values),
        };

        if self.is_empty() {
            Err(quantic::Error::TooMuchConstraints)
        } else {
            Ok(())
        }
    }

    fn collapse(&self) -> Result<Self::Collapsed, quantic::Error> {
        let kind = self.iter().next().ok_or(super::Error::EmptyCantCollapse)?;
        kind.collapse()
    }

    fn can_collapse_to(&self, potential_collapse: &Self::Collapsed) -> bool {
        self.iter()
            .any(|item| item.can_collapse_to(potential_collapse))
    }
}
