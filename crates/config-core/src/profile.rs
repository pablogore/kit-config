pub struct ConfigurationProfile {
    name: String,
    active: bool,
}

impl ConfigurationProfile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            active: false,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
