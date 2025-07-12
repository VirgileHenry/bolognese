use crate::factorio;
use crate::quantic;

pub type Item = Vec<factorio::Item>;

impl quantic::Quantic for Item {
    type Collapsed = factorio::Item;

    fn all(_: crate::common::Position, _: crate::common::Size) -> Self {
        vec![
            factorio::Item::IronOre,
            factorio::Item::CopperOre,
            factorio::Item::Stone,
            factorio::Item::Coal,
            factorio::Item::Water,
            factorio::Item::CrudeOil,
            factorio::Item::IronPlate,
            factorio::Item::CopperPlate,
            factorio::Item::SteelPlate,
            factorio::Item::IronGearWheel,
            factorio::Item::CopperCable,
            factorio::Item::ElectronicCircuit,
            factorio::Item::AdvancedCircuit,
            factorio::Item::ProcessingUnit,
            factorio::Item::EngineUnit,
            factorio::Item::ElectricEngineUnit,
            factorio::Item::PetroleumGas,
            factorio::Item::PlasticBar,
            factorio::Item::Sulfur,
            factorio::Item::SulfuricAcid,
            factorio::Item::Battery,
            factorio::Item::Lubricant,
            factorio::Item::Pipe,
            factorio::Item::IronStick,
            factorio::Item::FlyingRobotFrame,
            factorio::Item::TransportBelt,
            factorio::Item::Inserter,
            factorio::Item::AutomationSciencePack,
            factorio::Item::LogisticSciencePack,
            factorio::Item::ChemicalSciencePack,
            factorio::Item::ProductionSciencePack,
            factorio::Item::UtilitySciencePack,
        ]
    }

    fn entropy(&self) -> f32 {
        self.len() as f32
    }

    fn collapse(&self) -> Result<Self::Collapsed, quantic::Error> {
        self.iter()
            .next()
            .cloned()
            .ok_or(quantic::Error::EmptyCantCollapse)
    }

    fn can_collapse_to(&self, potential_collapse: &Self::Collapsed) -> bool {
        self.iter().any(|item| item == potential_collapse)
    }
}

pub type Liquid = Vec<factorio::Liquid>;

impl quantic::Quantic for Liquid {
    type Collapsed = factorio::Liquid;

    fn all(_: crate::common::Position, _: crate::common::Size) -> Self {
        vec![
            factorio::Liquid::Water,
            factorio::Liquid::CrudeOil,
            factorio::Liquid::PetroleumGas,
            factorio::Liquid::SulfuricAcid,
            factorio::Liquid::Lubricant,
        ]
    }

    fn entropy(&self) -> f32 {
        self.len() as f32
    }

    fn collapse(&self) -> Result<Self::Collapsed, quantic::Error> {
        self.iter()
            .next()
            .cloned()
            .ok_or(quantic::Error::EmptyCantCollapse)
    }

    fn can_collapse_to(&self, potential_collapse: &Self::Collapsed) -> bool {
        self.iter().any(|item| item == potential_collapse)
    }
}
