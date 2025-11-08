use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct TableSpec {
    pub table: String,
    pub columns: BTreeMap<String, String>,
}
