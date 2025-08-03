use serde::Deserialize;
use super::raw_data::*;
use std::collections::HashMap;


pub trait EntityBox {
    fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn save_data(&mut self);
}

pub struct CharacterEntity {
    unique: u32,
    character_info : RawDataCharacterInfo,
    character_status_info: RawDataCharacterStatusInfo,
    character_attack_info: RawDataCharacterAttackInfo,
}

pub struct CharacterEntityContainer {
    entities: HashMap<u32, CharacterEntity>,
}

impl CharacterEntityContainer {
    pub fn new() -> Self {
        CharacterEntityContainer {
            entities: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: CharacterEntity) {
        self.entities.insert(entity.unique, entity);
    }

    pub fn get_entity(&self, unique: u32) -> Option<&CharacterEntity> {
        self.entities.get(&unique)
    }

    pub fn remove_entity(&mut self, unique: u32) {
        self.entities.remove(&unique);
    }
}

impl EntityBox for CharacterEntityContainer {
    fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Load data logic here
        Ok(())
    }

    fn save_data(&mut self) {
        // Save data logic here
    }
}