use sea_query::{ColumnDef, Alias, Table, TableCreateStatement, TableDropStatement};

use super::super::structs;

pub fn build_create(spec: &structs::TableSpec) -> TableCreateStatement {
    let mut table = Table::create();
    table.table(Alias::new(&spec.table)).if_not_exists();

    for (name, kind) in &spec.columns {
        let mut col = ColumnDef::new(Alias::new(name));
        match kind {
            structs::ColumnKind::Id => { col.integer().auto_increment().primary_key(); }
            structs::ColumnKind::Number => { col.integer(); }
            structs::ColumnKind::String => { col.string(); }
            structs::ColumnKind::DateTime => { col.timestamp(); }
            structs::ColumnKind::Boolean => { col.boolean(); }
        };
        table.col(&mut col);
    }

    table.to_owned()
}

pub fn build_drop(spec: &structs::TableSpec) -> TableDropStatement {
    Table::drop()
        .table(Alias::new(&spec.table))
        .to_owned()
}
