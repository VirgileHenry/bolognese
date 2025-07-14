mod building;
mod default;
mod item;
mod parser;
mod recipe;

pub use building::Building;
pub use building::BuildingPropertyValue;

#[derive(Debug)]
pub struct Library {
    data_path: String,
    items: Vec<item::Item>,
    recipes: Vec<recipe::Recipe>,
    buildings: Vec<building::Building>,
}

#[derive(Debug)]
pub enum LibraryIntegrityError {
    UnknownItem { item: String, field: String },
}

impl std::fmt::Display for LibraryIntegrityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibraryIntegrityError::UnknownItem { item, field } => {
                write!(f, "Unknown item for field {field}: {item}")
            }
        }
    }
}

impl Library {
    // pub fn new(data_path: &str) -> std::io::Result<Self> {
    //     let (items, liquids) = item::load_items(&format!("{data_path}/items.txt"))?;
    //
    //     Ok(Self {
    //         data_path: data_path.to_string(),
    //         items,
    //         liquids,
    //     })
    // }

    pub fn ensure_integrity(&self) -> Result<(), LibraryIntegrityError> {
        for recipe in self.recipes.iter() {
            for (i, (input, _)) in recipe.inputs().iter().enumerate() {
                if !self.items.iter().any(|item| item.name() == input) {
                    return Err(LibraryIntegrityError::UnknownItem {
                        item: input.to_string(),
                        field: format!("Recipe input [{i}]"),
                    });
                }
            }
            for (i, (output, _)) in recipe.outputs().iter().enumerate() {
                if !self.items.iter().any(|item| item.name() == output) {
                    return Err(LibraryIntegrityError::UnknownItem {
                        item: output.to_string(),
                        field: format!("Recipe output [{i}]"),
                    });
                }
            }
            if !self
                .buildings
                .iter()
                .any(|item| item.name() == recipe.building())
            {
                return Err(LibraryIntegrityError::UnknownItem {
                    item: recipe.building().to_string(),
                    field: format!("Recipe building"),
                });
            }
        }

        Ok(())
    }

    pub fn info(&self) -> String {
        let mut info = format!("Library loaded from {}", self.data_path);
        info.push_str(&format!("\nItems: {}", self.items.len()));
        info.push_str(&format!("\nRecipes: {}", self.recipes.len()));
        info.push_str(&format!("\nBuildings: {}", self.buildings.len()));

        info
    }

    pub fn buildings(&self) -> impl Iterator<Item = &building::Building> {
        self.buildings.iter()
    }
}
