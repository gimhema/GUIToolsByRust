use serde::Deserialize;
use super::raw_data::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub trait EntityBox {
    fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn save_data(&mut self);
}

#[derive(Debug, Clone)]
pub struct CharacterEntity {
    unique: u32,
    character_info : RawDataCharacterInfo,
    character_status_info: RawDataCharacterStatusInfo,
    character_attack_info: RawDataCharacterAttackInfo,
}

pub struct CharacterEntityContainer {
    entities: HashMap<u32, CharacterEntity>,
    characeter_info_data_path : String,
    character_status_info_data_path: String,
    character_attack_info_data_path: String,
}

impl CharacterEntityContainer {
    pub fn new() -> Self {
        CharacterEntityContainer {
            entities: HashMap::new(),
            characeter_info_data_path: String::new(),
            character_status_info_data_path: String::new(),
            character_attack_info_data_path: String::new(),
        }
    }

    pub fn add_entity(&mut self, entity: CharacterEntity) {
        self.entities.insert(entity.unique, entity);
    }

    pub fn get_entity(&self, unique: u32) -> Option<&CharacterEntity> {
        self.entities.get(&unique)
    }

    pub fn update_character_entity_status(&mut self, unique: u32, status_info: RawDataCharacterStatusInfo) {
        if let Some(entity) = self.entities.get_mut(&unique) {
            entity.character_status_info = status_info;
        }
    }

    pub fn update_character_entity_attack_info(&mut self, unique: u32, attack_info: RawDataCharacterAttackInfo) {
        if let Some(entity) = self.entities.get_mut(&unique) {
            entity.character_attack_info = attack_info;
        }
    }

    pub fn remove_entity(&mut self, unique: u32) {
        self.entities.remove(&unique);
    }
}

impl EntityBox for CharacterEntityContainer {
    fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        {
            let file = File::open(self.characeter_info_data_path.as_str())?;
            let mut rdr = csv::Reader::from_reader(file);

            for result in rdr.deserialize::<RawDataCharacterInfo>() {
                let ref record = result?;
                let entity = CharacterEntity {
                    unique: record.unique.clone(),
                    character_info: record.clone(),
                    character_status_info: RawDataCharacterStatusInfo::new_zero(),
                    character_attack_info: RawDataCharacterAttackInfo::new_zero()
                };
                self.add_entity(entity);
            }
        }

        {
            let file = File::open(self.character_status_info_data_path.as_str())?;
            let mut rdr = csv::Reader::from_reader(file);

            for result in rdr.deserialize::<RawDataCharacterStatusInfo>() {
                let status_info = result?;
                let unique = status_info.unique;
                let health = status_info.health;
                let mana = status_info.mana;
                let stamina = status_info.stamina;
                self.update_character_entity_status(unique, RawDataCharacterStatusInfo {
                    unique,
                    health,
                    mana,
                    stamina,
                });
            }
        }

        {
            let file = File::open(self.character_attack_info_data_path.as_str())?;
            let mut rdr = csv::Reader::from_reader(file);

            for result in rdr.deserialize::<RawDataCharacterAttackInfo>() {
                let attack_info = result?;
                let unique = attack_info.unique;
                let attack_power = attack_info.attack_power;
                let attack_speed = attack_info.attack_speed;
                self.update_character_entity_attack_info(unique, RawDataCharacterAttackInfo {
                    unique,
                    attack_power,
                    attack_speed,
                });
            }
        } 
        Ok(())
    }

    fn save_data(&mut self) {
        // Save data logic here
    }
}