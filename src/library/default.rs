use super::*;

impl Default for Library {
    fn default() -> Self {
        let items = default_items();
        let recipes = default_recipes();
        let buildings = default_buildings();

        Library {
            data_path: "default".to_string(),
            items,
            recipes,
            buildings,
        }
    }
}

fn default_items() -> Vec<item::Item> {
    let items = vec![
        item::Item::new("IronOre".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new("CopperOre".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new("Stone".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new("Coal".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new("IronPlate".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new("CopperPlate".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new("SteelPlate".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new(
            "IronGearWheel".to_string(),
            crate::factorio::ItemKind::Solid,
        ),
        item::Item::new("CopperCable".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new(
            "ElectronicCircuit".to_string(),
            crate::factorio::ItemKind::Solid,
        ),
        item::Item::new(
            "AdvancedCircuit".to_string(),
            crate::factorio::ItemKind::Solid,
        ),
        item::Item::new(
            "ProcessingUnit".to_string(),
            crate::factorio::ItemKind::Solid,
        ),
        item::Item::new("EngineUnit".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new(
            "ElectricEngineUnit".to_string(),
            crate::factorio::ItemKind::Solid,
        ),
        item::Item::new("PlasticBar".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new("Sulfur".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new("Battery".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new("Pipe".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new("IronStick".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new(
            "FlyingRobotFrame".to_string(),
            crate::factorio::ItemKind::Solid,
        ),
        item::Item::new(
            "TransportBelt".to_string(),
            crate::factorio::ItemKind::Solid,
        ),
        item::Item::new("Inserter".to_string(), crate::factorio::ItemKind::Solid),
        item::Item::new(
            "AutomationSciencePack".to_string(),
            crate::factorio::ItemKind::Solid,
        ),
        item::Item::new(
            "LogisticSciencePack".to_string(),
            crate::factorio::ItemKind::Solid,
        ),
        item::Item::new(
            "ChemicalSciencePack".to_string(),
            crate::factorio::ItemKind::Solid,
        ),
        item::Item::new(
            "ProductionSciencePack".to_string(),
            crate::factorio::ItemKind::Solid,
        ),
        item::Item::new(
            "UtilitySciencePack".to_string(),
            crate::factorio::ItemKind::Solid,
        ),
        item::Item::new("Water".to_string(), crate::factorio::ItemKind::Liquid),
        item::Item::new("CrudeOil".to_string(), crate::factorio::ItemKind::Liquid),
        item::Item::new(
            "PetroleumGas".to_string(),
            crate::factorio::ItemKind::Liquid,
        ),
        item::Item::new(
            "SulfuricAcid".to_string(),
            crate::factorio::ItemKind::Liquid,
        ),
        item::Item::new("Lubricant".to_string(), crate::factorio::ItemKind::Liquid),
    ];

    items
}

fn default_recipes() -> Vec<recipe::Recipe> {
    let recipes = vec![
        recipe::Recipe::new(
            "StoneFurnace".into(),
            vec![("IronPlate".into(), 1)],
            vec![("IronOre".into(), 1)],
            3.2,
        ),
        recipe::Recipe::new(
            "StoneFurnace".into(),
            vec![("CopperPlate".into(), 1)],
            vec![("CopperOre".into(), 1)],
            3.2,
        ),
        recipe::Recipe::new(
            "StoneFurnace".into(),
            vec![("SteelPlate".into(), 1)],
            vec![("IronPlate".into(), 5)],
            17.5,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("IronGearWheel".into(), 1)],
            vec![("IronPlate".into(), 2)],
            0.5,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("CopperCable".into(), 2)],
            vec![("CopperPlate".into(), 1)],
            0.5,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("ElectronicCircuit".into(), 1)],
            vec![("IronPlate".into(), 1), ("CopperCable".into(), 3)],
            0.5,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("AdvancedCircuit".into(), 1)],
            vec![
                ("ElectronicCircuit".into(), 2),
                ("CopperCable".into(), 4),
                ("PlasticBar".into(), 2),
            ],
            6.0,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("ProcessingUnit".into(), 1)],
            vec![
                ("ElectronicCircuit".into(), 20),
                ("AdvancedCircuit".into(), 2),
                ("SulfuricAcid".into(), 5),
            ],
            10.0,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("EngineUnit".into(), 1)],
            vec![
                ("SteelPlate".into(), 2),
                ("IronGearWheel".into(), 1),
                ("Pipe".into(), 2),
            ],
            10.0,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("ElectricEngineUnit".into(), 1)],
            vec![
                ("EngineUnit".into(), 1),
                ("ElectronicCircuit".into(), 2),
                ("Lubricant".into(), 5),
            ],
            10.0,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("Battery".into(), 1)],
            vec![
                ("IronPlate".into(), 1),
                ("CopperPlate".into(), 1),
                ("SulfuricAcid".into(), 20),
            ],
            8.0,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("TransportBelt".into(), 2)],
            vec![("IronPlate".into(), 1), ("IronGearWheel".into(), 1)],
            0.5,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("Inserter".into(), 1)],
            vec![
                ("IronPlate".into(), 1),
                ("IronGearWheel".into(), 1),
                ("IronStick".into(), 1),
                ("ElectronicCircuit".into(), 1),
            ],
            0.5,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("FlyingRobotFrame".into(), 1)],
            vec![
                ("ElectricEngineUnit".into(), 1),
                ("AdvancedCircuit".into(), 3),
                ("SteelPlate".into(), 1),
            ],
            15.0,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("AutomationSciencePack".into(), 1)],
            vec![("IronGearWheel".into(), 1), ("CopperPlate".into(), 1)],
            4.0,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("LogisticSciencePack".into(), 1)],
            vec![("TransportBelt".into(), 1), ("Inserter".into(), 1)],
            6.0,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("ChemicalSciencePack".into(), 2)],
            vec![
                ("AdvancedCircuit".into(), 3),
                ("EngineUnit".into(), 2),
                ("Sulfur".into(), 1),
            ],
            24.0,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("ProductionSciencePack".into(), 2)],
            vec![
                ("Battery".into(), 1),
                ("SteelPlate".into(), 1),
                ("ElectricEngineUnit".into(), 1),
            ],
            24.0,
        ),
        recipe::Recipe::new(
            "AssemblingMachine".into(),
            vec![("UtilitySciencePack".into(), 3)],
            vec![
                ("ProcessingUnit".into(), 1),
                ("Battery".into(), 1),
                ("FlyingRobotFrame".into(), 1),
            ],
            24.0,
        ),
        recipe::Recipe::new(
            "ChemicalPlant".into(),
            vec![("PlasticBar".into(), 2)],
            vec![("PetroleumGas".into(), 20), ("Coal".into(), 1)],
            1.0,
        ),
        recipe::Recipe::new(
            "ChemicalPlant".into(),
            vec![("Sulfur".into(), 2)],
            vec![("PetroleumGas".into(), 30), ("Water".into(), 30)],
            1.0,
        ),
        recipe::Recipe::new(
            "ChemicalPlant".into(),
            vec![("SulfuricAcid".into(), 50)],
            vec![("Sulfur".into(), 5), ("Water".into(), 100)],
            1.0,
        ),
        recipe::Recipe::new(
            "OilRefinery".into(),
            vec![("PetroleumGas".into(), 50)],
            vec![("CrudeOil".into(), 5), ("Water".into(), 100)],
            5.0,
        ),
    ];

    recipes
}

fn default_buildings() -> Vec<building::Building> {
    use crate::common::{Color, Pixel};

    let buildings = vec![
        building::Building::new(
            "StoneFurnace".to_string(),
            vec![building::BuildingPropertyKind::Recipe],
            crate::common::Size::new(2, 2).unwrap(),
            vec![
                Pixel::new('🭅', Color::DarkGrey, Color::Black),
                Pixel::new('🭐', Color::DarkGrey, Color::Black),
                Pixel::new('🯮', Color::Yellow, Color::DarkGrey),
                Pixel::new('🯭', Color::Yellow, Color::DarkGrey),
            ],
        ),
        building::Building::new(
            "ElectricMiningDrill".to_string(),
            vec![building::BuildingPropertyKind::Orientation],
            crate::common::Size::new(3, 3).unwrap(),
            vec![
                Pixel::new('╔', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('╗', Color::White, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new('E', Color::Blue, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new('╚', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('╝', Color::White, Color::Black),
            ],
        ),
        building::Building::new(
            "Pumpjack".to_string(),
            vec![building::BuildingPropertyKind::Orientation],
            crate::common::Size::new(3, 3).unwrap(),
            vec![
                Pixel::new('╔', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('╗', Color::White, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new('P', Color::Green, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new('╚', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('╝', Color::White, Color::Black),
            ],
        ),
        building::Building::new(
            "OffshorePump".to_string(),
            vec![building::BuildingPropertyKind::Orientation],
            crate::common::Size::new(1, 2).unwrap(),
            vec![
                Pixel::new('O', Color::White, Color::Black),
                Pixel::new('P', Color::White, Color::Black),
            ],
        ),
        building::Building::new(
            "OilRefinery".to_string(),
            vec![
                building::BuildingPropertyKind::Orientation,
                building::BuildingPropertyKind::Recipe,
            ],
            crate::common::Size::new(5, 5).unwrap(),
            vec![
                Pixel::new('╔', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('╗', Color::White, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new(' ', Color::White, Color::Black),
                Pixel::new(' ', Color::White, Color::Black),
                Pixel::new(' ', Color::White, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new(' ', Color::White, Color::Black),
                Pixel::new('O', Color::White, Color::Black),
                Pixel::new(' ', Color::White, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new(' ', Color::White, Color::Black),
                Pixel::new(' ', Color::White, Color::Black),
                Pixel::new(' ', Color::White, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new('╚', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('╝', Color::White, Color::Black),
            ],
        ),
        building::Building::new(
            "ChemicalPlant".to_string(),
            vec![
                building::BuildingPropertyKind::Orientation,
                building::BuildingPropertyKind::Recipe,
            ],
            crate::common::Size::new(2, 2).unwrap(),
            vec![
                Pixel::new('C', Color::White, Color::Black),
                Pixel::new('C', Color::White, Color::Black),
                Pixel::new('C', Color::White, Color::Black),
                Pixel::new('C', Color::White, Color::Black),
            ],
        ),
        building::Building::new(
            "AssemblingMachine".to_string(),
            vec![building::BuildingPropertyKind::Recipe],
            crate::common::Size::new(3, 3).unwrap(),
            vec![
                Pixel::new('╔', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('╗', Color::White, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new('A', Color::DarkYellow, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new('╚', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('╝', Color::White, Color::Black),
            ],
        ),
        building::Building::new(
            "TransportBelt".to_string(),
            vec![
                building::BuildingPropertyKind::Orientation,
                building::BuildingPropertyKind::Item,
            ],
            crate::common::Size::new(1, 1).unwrap(),
            vec![Pixel::new('T', Color::White, Color::Black)],
        ),
        building::Building::new(
            "UndergroundBelt".to_string(),
            vec![
                building::BuildingPropertyKind::Orientation,
                building::BuildingPropertyKind::Direction,
                building::BuildingPropertyKind::Item,
            ],
            crate::common::Size::new(1, 1).unwrap(),
            vec![Pixel::new('U', Color::White, Color::Black)],
        ),
        building::Building::new(
            "Pipe".to_string(),
            vec![building::BuildingPropertyKind::Liquid],
            crate::common::Size::new(1, 1).unwrap(),
            vec![Pixel::new('P', Color::White, Color::Black)],
        ),
        building::Building::new(
            "PipeToGround".to_string(),
            vec![
                building::BuildingPropertyKind::Orientation,
                building::BuildingPropertyKind::Liquid,
            ],
            crate::common::Size::new(1, 1).unwrap(),
            vec![Pixel::new('P', Color::White, Color::Black)],
        ),
        building::Building::new(
            "Inserter".to_string(),
            vec![
                building::BuildingPropertyKind::Orientation,
                building::BuildingPropertyKind::Item,
            ],
            crate::common::Size::new(1, 1).unwrap(),
            vec![Pixel::new('I', Color::White, Color::Black)],
        ),
        building::Building::new(
            "LongHandedInserter".to_string(),
            vec![
                building::BuildingPropertyKind::Orientation,
                building::BuildingPropertyKind::Item,
            ],
            crate::common::Size::new(1, 1).unwrap(),
            vec![Pixel::new('L', Color::White, Color::Black)],
        ),
        building::Building::new(
            "Lab".to_string(),
            vec![],
            crate::common::Size::new(3, 3).unwrap(),
            vec![
                Pixel::new('╔', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('╗', Color::White, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new('L', Color::Cyan, Color::Black),
                Pixel::new('║', Color::White, Color::Black),
                Pixel::new('╚', Color::White, Color::Black),
                Pixel::new('═', Color::White, Color::Black),
                Pixel::new('╝', Color::White, Color::Black),
            ],
        ),
    ];

    buildings
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn default_lib_integrity() {
        let default_library = Library::default();
        assert!(default_library.ensure_integrity().is_ok())
    }
}
