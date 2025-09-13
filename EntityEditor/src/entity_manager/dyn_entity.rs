use std::collections::HashMap;
use super::schema::{TableSchema, DataType, ColumnDef};


/// A single table row, preserving arbitrary columns from CSV.
#[derive(Debug, Clone)]
pub struct DynRow {
pub key: String, // string key for uniformity
pub cells: HashMap<String, String>, // header(label)->raw string
}


impl DynRow {
pub fn get(&self, header: &str) -> Option<&str> {
self.cells.get(header).map(|s| s.as_str())
}
pub fn set(&mut self, header: &str, val: String) {
self.cells.insert(header.to_string(), val);
}
}


/// Three-source merged entity keyed by `unique`.
#[derive(Debug, Default, Clone)]
pub struct DynEntity {
pub unique: String,
pub info: Option<DynRow>,
pub status: Option<DynRow>,
pub attack: Option<DynRow>,
}


impl DynEntity {
pub fn ensure_unique(&mut self) { if self.unique.is_empty() {
if let Some(r) = self.info.as_ref() { self.unique = r.key.clone(); }
else if let Some(r) = self.status.as_ref() { self.unique = r.key.clone(); }
else if let Some(r) = self.attack.as_ref() { self.unique = r.key.clone(); }
}}
}