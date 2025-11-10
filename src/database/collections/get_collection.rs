use axum::{
    extract::{Path, State},
    Json,
};
use http::StatusCode;
use serde_json::{Map, Value};
use sqlx::{Column, Row};

use crate::AppState;
use crate::database::collections::utils::check_table::table_exists;

pub async fn get_collection(
    State(state): State<AppState>,
    Path(table): Path<String>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    {
        let is_cached = {
            let cache = state.table_cache.read().unwrap();
            cache.contains(&table)
        };

        if is_cached {
            return fetch_table_as_json(&state, &table).await;
        }
    }

    match table_exists(&state.db, &table).await {
        Ok(true) => {
            let mut cache = state.table_cache.write().unwrap();
            cache.insert(table.clone());
        }
        Ok(false) => return Err(StatusCode::NOT_FOUND),
        Err(code) => return Err(code),
    }

    fetch_table_as_json(&state, &table).await
}

async fn fetch_table_as_json(
    state: &AppState,
    table: &str,
) -> Result<Json<Vec<Value>>, StatusCode> {
    let query = format!(r#"SELECT * FROM "{}""#, table);

    let rows_res = sqlx::query(&query).fetch_all(&state.db).await;

    let rows = match rows_res {
        Ok(r) => r,
        Err(e) => {
            // postgres uses "42P01" for undefined table
            let msg = e.to_string();
            let looks_like_missing_table =
                msg.contains("42P01") ||
                msg.contains("relation") && msg.contains("does not exist");

            if looks_like_missing_table {
                let mut cache = state.table_cache.write().unwrap();
                cache.remove(table);
                return Err(StatusCode::NOT_FOUND);
            }

            tracing::error!("failed to fetch rows from {table}. Error: {e} Message: {msg}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let mut obj = Map::new();
        for col in row.columns() {
            let name = col.name();
            let val = row
                .try_get::<String, _>(name)
                .map(Value::String)
                .or_else(|_| row.try_get::<i64, _>(name).map(|i| Value::Number(i.into())))
                .unwrap_or(Value::Null);
            obj.insert(name.to_string(), val);
        }
        out.push(Value::Object(obj));
    }

    Ok(Json(out))
}
