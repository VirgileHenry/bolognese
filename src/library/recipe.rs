#[derive(Debug)]
pub struct Recipe {
    building: String,
    outputs: Vec<(String, usize)>,
    inputs: Vec<(String, usize)>,
    time: f32,
}

impl Recipe {
    pub fn new(
        building: String,
        outputs: Vec<(String, usize)>,
        inputs: Vec<(String, usize)>,
        time: f32,
    ) -> Recipe {
        Recipe {
            building,
            outputs,
            inputs,
            time,
        }
    }

    pub fn building(&self) -> &str {
        &self.building
    }

    pub fn outputs(&self) -> &[(String, usize)] {
        &self.outputs
    }

    pub fn inputs(&self) -> &[(String, usize)] {
        &self.inputs
    }

    pub fn time(&self) -> f32 {
        self.time
    }
}
