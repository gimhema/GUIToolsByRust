use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataType { Int, Float, Text }


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDef {
pub key: String, // canonical key (case-insensitive match)
pub label: String, // display label (from header)
pub dtype: DataType, // inferred or overridden
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
pub name: String, // e.g., "character_info"
pub key_column: String, // e.g., "CharacterUnique"
pub columns: Vec<ColumnDef>, // includes key column too
}


impl TableSchema {
pub fn find(&self, key: &str) -> Option<&ColumnDef> {
self.columns.iter().find(|c| c.key.eq_ignore_ascii_case(key))
}
}