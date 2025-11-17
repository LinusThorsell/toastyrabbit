use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct TableSpec {
    pub table: String,
    pub columns: BTreeMap<String, ColumnKind>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ColumnKind {
    Id,
    Number,
    DateTime,
    String,
    Boolean
}
