use crate::factorio::Item;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    pub output: Item,
    pub inputs: &'static [(Item, u32)],
    pub output_amount: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FurnaceRecipe {
    inner: Recipe,
}

impl core::ops::Deref for FurnaceRecipe {
    type Target = Recipe;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FurnaceRecipe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblerRecipe {
    inner: Recipe,
}

impl core::ops::Deref for AssemblerRecipe {
    type Target = Recipe;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AssemblerRecipe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChemicalRecipe {
    inner: Recipe,
}

impl core::ops::Deref for ChemicalRecipe {
    type Target = Recipe;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ChemicalRecipe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefineryRecipe {
    inner: Recipe,
}

impl core::ops::Deref for RefineryRecipe {
    type Target = Recipe;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RefineryRecipe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub const SMELTING_RECIPES: &[FurnaceRecipe] = &[
    FurnaceRecipe {
        inner: Recipe {
            output: Item::IronPlate,
            inputs: &[(Item::IronOre, 1)],
            output_amount: 1,
        },
    },
    FurnaceRecipe {
        inner: Recipe {
            output: Item::CopperPlate,
            inputs: &[(Item::CopperOre, 1)],
            output_amount: 1,
        },
    },
    FurnaceRecipe {
        inner: Recipe {
            output: Item::SteelPlate,
            inputs: &[(Item::IronPlate, 5)],
            output_amount: 1,
        },
    },
];

pub const ASSEMBLING_MACHINE_RECIPES: &[AssemblerRecipe] = &[
    AssemblerRecipe {
        inner: Recipe {
            output: Item::IronGearWheel,
            inputs: &[(Item::IronPlate, 2)],
            output_amount: 1,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::CopperCable,
            inputs: &[(Item::CopperPlate, 1)],
            output_amount: 2,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::ElectronicCircuit,
            inputs: &[(Item::IronPlate, 1), (Item::CopperCable, 3)],
            output_amount: 1,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::AdvancedCircuit,
            inputs: &[
                (Item::ElectronicCircuit, 2),
                (Item::CopperCable, 4),
                (Item::PlasticBar, 2),
            ],
            output_amount: 1,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::ProcessingUnit,
            inputs: &[
                (Item::ElectronicCircuit, 20),
                (Item::AdvancedCircuit, 2),
                (Item::SulfuricAcid, 5),
            ],
            output_amount: 1,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::EngineUnit,
            inputs: &[
                (Item::SteelPlate, 2),
                (Item::IronGearWheel, 1),
                (Item::Pipe, 2),
            ],
            output_amount: 1,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::ElectricEngineUnit,
            inputs: &[
                (Item::EngineUnit, 1),
                (Item::ElectronicCircuit, 2),
                (Item::Lubricant, 5),
            ],
            output_amount: 1,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::Battery,
            inputs: &[
                (Item::IronPlate, 1),
                (Item::CopperPlate, 1),
                (Item::SulfuricAcid, 20),
            ],
            output_amount: 1,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::TransportBelt,
            inputs: &[(Item::IronPlate, 1), (Item::IronGearWheel, 1)],
            output_amount: 2,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::Inserter,
            inputs: &[
                (Item::IronPlate, 1),
                (Item::IronGearWheel, 1),
                (Item::IronStick, 1),
                (Item::ElectronicCircuit, 1),
            ],
            output_amount: 1,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::FlyingRobotFrame,
            inputs: &[
                (Item::ElectricEngineUnit, 1),
                (Item::AdvancedCircuit, 3),
                (Item::SteelPlate, 1),
            ],
            output_amount: 1,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::AutomationSciencePack,
            inputs: &[(Item::IronGearWheel, 1), (Item::CopperPlate, 1)],
            output_amount: 1,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::LogisticSciencePack,
            inputs: &[(Item::TransportBelt, 1), (Item::Inserter, 1)],
            output_amount: 1,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::ChemicalSciencePack,
            inputs: &[
                (Item::AdvancedCircuit, 3),
                (Item::EngineUnit, 2),
                (Item::Sulfur, 1),
            ],
            output_amount: 2,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::ProductionSciencePack,
            inputs: &[
                (Item::Battery, 1),
                (Item::SteelPlate, 1),
                (Item::ElectricEngineUnit, 1),
            ],
            output_amount: 2,
        },
    },
    AssemblerRecipe {
        inner: Recipe {
            output: Item::UtilitySciencePack,
            inputs: &[
                (Item::ProcessingUnit, 1),
                (Item::Battery, 1),
                (Item::FlyingRobotFrame, 1),
            ],
            output_amount: 3,
        },
    },
];

pub const CHEMICAL_PLANT_RECIPES: &[ChemicalRecipe] = &[
    ChemicalRecipe {
        inner: Recipe {
            output: Item::PlasticBar,
            inputs: &[(Item::PetroleumGas, 20), (Item::Coal, 1)],
            output_amount: 2,
        },
    },
    ChemicalRecipe {
        inner: Recipe {
            output: Item::Sulfur,
            inputs: &[(Item::PetroleumGas, 30), (Item::Water, 30)],
            output_amount: 2,
        },
    },
    ChemicalRecipe {
        inner: Recipe {
            output: Item::SulfuricAcid,
            inputs: &[(Item::Sulfur, 5), (Item::Water, 100)],
            output_amount: 50,
        },
    },
];

pub const OIL_REFINERY_RECIPES: &[RefineryRecipe] = &[RefineryRecipe {
    inner: Recipe {
        output: Item::PetroleumGas,
        inputs: &[(Item::CrudeOil, 5), (Item::Water, 100)],
        output_amount: 50,
    },
}];
