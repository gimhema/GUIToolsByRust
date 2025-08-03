use serde::Deserialize;
use super::raw_data::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

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
                // let record = result?;
                // println!("{:?}", record);
            }
        }

        {
            let file = File::open(self.character_status_info_data_path.as_str())?;
            let mut rdr = csv::Reader::from_reader(file);

            for result in rdr.deserialize::<RawDataCharacterStatusInfo>() {
                // let record = result?;
                // println!("{:?}", record);
            }
        }

        {
            let file = File::open(self.character_attack_info_data_path.as_str())?;
            let mut rdr = csv::Reader::from_reader(file);

            for result in rdr.deserialize::<RawDataCharacterAttackInfo>() {
                // let record = result?;
                // println!("{:?}", record);
            }
        } 




        Ok(())
    }

    fn save_data(&mut self) {
        // Save data logic here
    }
}