use super::Quantic;
use crate::factorio;

pub type FurnaceRecipe = Vec<factorio::FurnaceRecipe>;

impl Quantic for FurnaceRecipe {
    type Collapsed = factorio::FurnaceRecipe;

    fn all(_: crate::common::Position, _: crate::common::Size) -> Self {
        factorio::SMELTING_RECIPES.iter().cloned().collect()
    }

    fn entropy(&self) -> f32 {
        self.len() as f32
    }

    fn collapse(&self) -> Result<Self::Collapsed, super::Error> {
        self.iter()
            .next()
            .cloned()
            .ok_or(super::Error::EmptyCantCollapse)
    }

    fn can_collapse_to(&self, potential_collapse: &Self::Collapsed) -> bool {
        self.iter().any(|item| item == potential_collapse)
    }
}

pub type AssemblerRecipe = Vec<factorio::AssemblerRecipe>;

impl Quantic for AssemblerRecipe {
    type Collapsed = factorio::AssemblerRecipe;

    fn all(_: crate::common::Position, _: crate::common::Size) -> Self {
        factorio::ASSEMBLING_MACHINE_RECIPES
            .iter()
            .cloned()
            .collect()
    }

    fn entropy(&self) -> f32 {
        self.len() as f32
    }

    fn collapse(&self) -> Result<Self::Collapsed, super::Error> {
        self.iter()
            .next()
            .cloned()
            .ok_or(super::Error::EmptyCantCollapse)
    }

    fn can_collapse_to(&self, potential_collapse: &Self::Collapsed) -> bool {
        self.iter().any(|item| item == potential_collapse)
    }
}

pub type ChemicalRecipe = Vec<factorio::ChemicalRecipe>;

impl Quantic for ChemicalRecipe {
    type Collapsed = factorio::ChemicalRecipe;

    fn all(_: crate::common::Position, _: crate::common::Size) -> Self {
        factorio::CHEMICAL_PLANT_RECIPES.iter().cloned().collect()
    }

    fn entropy(&self) -> f32 {
        self.len() as f32
    }

    fn collapse(&self) -> Result<Self::Collapsed, super::Error> {
        self.iter()
            .next()
            .cloned()
            .ok_or(super::Error::EmptyCantCollapse)
    }

    fn can_collapse_to(&self, potential_collapse: &Self::Collapsed) -> bool {
        self.iter().any(|item| item == potential_collapse)
    }
}

pub type RefineryRecipe = Vec<factorio::RefineryRecipe>;

impl Quantic for RefineryRecipe {
    type Collapsed = factorio::RefineryRecipe;

    fn all(_: crate::common::Position, _: crate::common::Size) -> Self {
        factorio::OIL_REFINERY_RECIPES.iter().cloned().collect()
    }

    fn entropy(&self) -> f32 {
        self.len() as f32
    }

    fn collapse(&self) -> Result<Self::Collapsed, super::Error> {
        self.into_iter()
            .next()
            .cloned()
            .ok_or(super::Error::EmptyCantCollapse)
    }

    fn can_collapse_to(&self, potential_collapse: &Self::Collapsed) -> bool {
        self.iter().any(|item| item == potential_collapse)
    }
}
