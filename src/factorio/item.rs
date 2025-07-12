#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    IronOre,
    CopperOre,
    Stone,
    Coal,
    Water,
    CrudeOil,
    IronPlate,
    CopperPlate,
    SteelPlate,
    IronGearWheel,
    CopperCable,
    ElectronicCircuit,
    AdvancedCircuit,
    ProcessingUnit,
    EngineUnit,
    ElectricEngineUnit,
    PetroleumGas,
    PlasticBar,
    Sulfur,
    SulfuricAcid,
    Battery,
    Lubricant,
    Pipe,
    IronStick,
    FlyingRobotFrame,
    TransportBelt,
    Inserter,
    AutomationSciencePack,
    LogisticSciencePack,
    ChemicalSciencePack,
    ProductionSciencePack,
    UtilitySciencePack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Liquid {
    Water,
    CrudeOil,
    PetroleumGas,
    SulfuricAcid,
    Lubricant,
}

impl From<Liquid> for Item {
    fn from(value: Liquid) -> Self {
        match value {
            Liquid::Water => Item::Water,
            Liquid::CrudeOil => Item::CrudeOil,
            Liquid::PetroleumGas => Item::PetroleumGas,
            Liquid::SulfuricAcid => Item::SulfuricAcid,
            Liquid::Lubricant => Item::Lubricant,
        }
    }
}
