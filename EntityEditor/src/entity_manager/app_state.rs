use std::collections::BTreeMap;
use anyhow::Result;
use super::schema::TableSchema;
use super::dyn_entity::{DynEntity, DynRow};
use super::storage::{load_table, save_table};


pub struct DataSets {
pub info_schema: TableSchema,
pub status_schema: TableSchema,
pub attack_schema: TableSchema,
pub info: BTreeMap<String, DynRow>,
pub status: BTreeMap<String, DynRow>,
pub attack: BTreeMap<String, DynRow>,
}


impl DataSets {
pub fn load(info_path:&str, status_path:&str, attack_path:&str, status_key_hint:&str) -> Result<Self> {
let (info_schema, info) = load_table(info_path, "CharacterUnique")?;
let (status_schema, status) = load_table(status_path, status_key_hint)?;
let (attack_schema, attack) = load_table(attack_path, "CharacterUnique")?;
Ok(Self{ info_schema, status_schema, attack_schema, info, status, attack })
}


pub fn merged(&self) -> Vec<DynEntity> {
let mut keys: BTreeMap<String, ()> = BTreeMap::new();
for k in self.info.keys() { keys.insert(k.clone(),()); }
for k in self.status.keys() { keys.insert(k.clone(),()); }
for k in self.attack.keys() { keys.insert(k.clone(),()); }


keys.into_iter().map(|(k,_)| DynEntity{
unique: k.clone(),
info: self.info.get(&k).cloned(),
status: self.status.get(&k).cloned(),
attack: self.attack.get(&k).cloned(),
}).collect()
}


pub fn save_all(&self, info_path:&str, status_path:&str, attack_path:&str) -> Result<()> {
use super::dyn_entity::DynRow;
let info_map = self.info.clone();
let status_map = self.status.clone();
let attack_map = self.attack.clone();


save_table(info_path, &self.info_schema.key_column, &info_map)?;
save_table(status_path, &self.status_schema.key_column, &status_map)?;
save_table(attack_path, &self.attack_schema.key_column, &attack_map)?;
Ok(())
}
}