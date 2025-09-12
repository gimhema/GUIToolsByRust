use super::entity::*;
use std::error::Error;

pub struct EntityDataHanlder;

impl EntityDataHanlder {
    pub fn new() -> Self { Self }

    pub fn load_all(
        &mut self,
        container: &mut CharacterEntityContainer,
        status_key_override: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(key) = status_key_override {
            container.set_status_key_column_override(key);
        }
        container.load_data()
    }
}
