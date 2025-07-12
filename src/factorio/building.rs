use crate::common::Position as Offset;
use crate::factorio::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Building {
    ElectricMiningDrill {
        orientation: Orientation,
        offset: Offset,
    },
    Pumpjack {
        orientation: Orientation,
        offset: Offset,
    },
    OffshorePump {
        orientation: Orientation,
        offset: Offset,
    },
    StoneFurnace {
        recipe: FurnaceRecipe,
        offset: Offset,
    },
    OilRefinery {
        orientation: Orientation,
        recipe: RefineryRecipe,
        offset: Offset,
    },
    ChemicalPlant {
        orientation: Orientation,
        recipe: ChemicalRecipe,
        offset: Offset,
    },
    AssemblingMachine {
        recipe: AssemblerRecipe,
        offset: Offset,
    },
    TransportBelt {
        orientation: Orientation,
        item: Item,
    },
    UndergroundBelt {
        orientation: Orientation,
        direction: Direction,
        item: Item,
    },
    Pipe {
        liquid: Liquid,
    },
    PipeToGround {
        orientation: Orientation,
        liquid: Liquid,
    },
    Inserter {
        orientation: Orientation,
        item: Item,
    },
    LongHandedInserter {
        orientation: Orientation,
        item: Item,
    },
    Lab {
        offset: Offset,
    },
}

impl Building {
    pub fn size(&self) -> crate::common::Size {
        match self {
            Building::ElectricMiningDrill { .. } => crate::common::Size::new(3, 3).unwrap(),
            Building::Pumpjack { .. } => crate::common::Size::new(3, 3).unwrap(),
            Building::OffshorePump { .. } => crate::common::Size::new(1, 1).unwrap(),
            Building::StoneFurnace { .. } => crate::common::Size::new(2, 2).unwrap(),
            Building::OilRefinery { .. } => crate::common::Size::new(4, 4).unwrap(),
            Building::ChemicalPlant { .. } => crate::common::Size::new(2, 2).unwrap(),
            Building::AssemblingMachine { .. } => crate::common::Size::new(3, 3).unwrap(),
            Building::TransportBelt { .. } => crate::common::Size::new(1, 1).unwrap(),
            Building::UndergroundBelt { .. } => crate::common::Size::new(1, 1).unwrap(),
            Building::Pipe { .. } => crate::common::Size::new(1, 1).unwrap(),
            Building::PipeToGround { .. } => crate::common::Size::new(1, 1).unwrap(),
            Building::Inserter { .. } => crate::common::Size::new(1, 1).unwrap(),
            Building::LongHandedInserter { .. } => crate::common::Size::new(1, 1).unwrap(),
            Building::Lab { .. } => crate::common::Size::new(3, 3).unwrap(),
        }
    }

    pub fn offset(&self) -> crate::common::Position {
        match self {
            Building::ElectricMiningDrill { offset, .. } => *offset,
            Building::Pumpjack { offset, .. } => *offset,
            Building::OffshorePump { offset, .. } => *offset,
            Building::StoneFurnace { offset, .. } => *offset,
            Building::OilRefinery { offset, .. } => *offset,
            Building::ChemicalPlant { offset, .. } => *offset,
            Building::AssemblingMachine { offset, .. } => *offset,
            Building::TransportBelt { .. } => crate::common::Position::new(0, 0),
            Building::UndergroundBelt { .. } => crate::common::Position::new(0, 0),
            Building::Pipe { .. } => crate::common::Position::new(0, 0),
            Building::PipeToGround { .. } => crate::common::Position::new(0, 0),
            Building::Inserter { .. } => crate::common::Position::new(0, 0),
            Building::LongHandedInserter { .. } => crate::common::Position::new(0, 0),
            Building::Lab { offset, .. } => *offset,
        }
    }

    pub fn display_char(&self) -> char {
        use crate::common::Position;
        match self {
            Building::ElectricMiningDrill { .. } => 'D',
            Building::Pumpjack { .. } => 'P',
            Building::OffshorePump { .. } => 'p',
            Building::StoneFurnace { offset, .. } => match offset {
                Position { x: 0, y: 0 } => '🭅',
                Position { x: 0, y: 1 } => '🯮',
                Position { x: 1, y: 0 } => '🭐',
                Position { x: 1, y: 1 } => '🯭',
                _ => unreachable!(),
            },
            Building::OilRefinery { .. } => 'R',
            Building::ChemicalPlant { .. } => 'C',
            Building::AssemblingMachine { .. } => 'A',
            Building::TransportBelt { .. } => 'T',
            Building::UndergroundBelt { .. } => 'U',
            Building::Pipe { .. } => '=',
            Building::PipeToGround { .. } => '-',
            Building::Inserter { .. } => 'i',
            Building::LongHandedInserter { .. } => 'I',
            Building::Lab { .. } => 'L',
        }
    }

    pub fn set_offset(&mut self, new_offset: crate::common::Position) {
        match self {
            Building::ElectricMiningDrill { offset, .. } => *offset = new_offset,
            Building::Pumpjack { offset, .. } => *offset = new_offset,
            Building::OffshorePump { offset, .. } => *offset = new_offset,
            Building::StoneFurnace { offset, .. } => *offset = new_offset,
            Building::OilRefinery { offset, .. } => *offset = new_offset,
            Building::ChemicalPlant { offset, .. } => *offset = new_offset,
            Building::AssemblingMachine { offset, .. } => *offset = new_offset,
            Building::Lab { offset, .. } => *offset = new_offset,
            _ => { /* we're chilling */ }
        }
    }
}
