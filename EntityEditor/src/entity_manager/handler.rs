use super::entity::*;
use super::raw_data::*;
use std::error::Error;
use std::fs::File;
use std::collections::HashMap;


pub enum RawDataKey {
    Base,
    End
}

pub struct EntityDataHanlder {
//    raw_datas: HashMap<RawDataKey, RawDataBox>,
}

impl EntityDataHanlder {
    pub fn new() -> Self {
        EntityDataHanlder {
        //    raw_datas: HashMap::new(),
        }
    }

    pub fn load_data(&mut self, path : &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        Ok(())
    }


}
