use sqlx::{PgPool, FromRow};
use std::collections::{BTreeMap, HashSet};
use crate::database::structs::{ColumnKind, TableSpec};

#[derive(Debug, FromRow)]
struct TableRow {
    table_name: String,
}

#[derive(Debug, FromRow)]
struct ColumnRow {
    column_name: String,
    data_type: String,
    is_identity: Option<String>,
}

#[derive(Debug, FromRow)]
struct PrimaryKeyRow {
    column_name: String,
}

pub async fn load_table_specs(pool: &PgPool) -> Result<Vec<TableSpec>, sqlx::Error> {
    let tables: Vec<TableRow> = sqlx::query_as::<_, TableRow>(
        r#"
        SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = 'public'
          AND table_type = 'BASE TABLE'
          AND table_name NOT LIKE 'refinery_%'
        "#
    )
    .fetch_all(pool)
    .await?;


    let mut specs = Vec::new();

    for t in tables {
        let pk_rows: Vec<PrimaryKeyRow> = sqlx::query_as::<_, PrimaryKeyRow>(
                r#"
                SELECT kcu.column_name
                FROM information_schema.table_constraints tc
                JOIN information_schema.key_column_usage kcu
                  ON tc.constraint_name = kcu.constraint_name
                  AND tc.table_schema = kcu.table_schema
                WHERE tc.table_schema = 'public'
                  AND tc.table_name = $1
                  AND tc.constraint_type = 'PRIMARY KEY'
                "#
            )
            .bind(&t.table_name)
            .fetch_all(pool)
            .await?;


        let pk_set: HashSet<String> = pk_rows.into_iter().map(|r| r.column_name).collect();

        let col_rows: Vec<ColumnRow> = sqlx::query_as::<_, ColumnRow>(
            r#"
            SELECT
                column_name,
                data_type,
                is_identity
            FROM information_schema.columns
            WHERE table_schema = 'public'
              AND table_name = $1
            ORDER BY ordinal_position
            "#
        )
            .bind(&t.table_name)
            .fetch_all(pool)
            .await?;

        let mut cols = BTreeMap::new();
        for col in col_rows {
            let kind = map_pg_type_to_kind(
                &col.data_type,
                pk_set.contains(&col.column_name),
                col.is_identity.as_deref() == Some("YES"),
            );
            cols.insert(col.column_name, kind);
        }

        specs.push(TableSpec {
            table: t.table_name,
            columns: cols,
        });
    }

    Ok(specs)
}

fn map_pg_type_to_kind(
    data_type: &str,
    is_primary_key: bool,
    is_identity: bool,
) -> ColumnKind {
    if is_primary_key && is_identity && (data_type == "integer" || data_type == "bigint") {
        return ColumnKind::Id;
    }

    match data_type {
        "integer" | "bigint" | "smallint" => ColumnKind::Number,
        "timestamp without time zone" | "timestamp with time zone" => ColumnKind::DateTime,
        "character varying" | "text" | "uuid" => ColumnKind::String,
        "boolean" => ColumnKind::Boolean,
        // Default to string for now.
        _ => {
            tracing::warn!("Unknown data type: {}, defaulting to string", data_type);
            ColumnKind::String
        }
    }
}

