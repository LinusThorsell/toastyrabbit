use axum::{
    extract::{Path, Query, State},
    Json,
};
use http::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use sqlx::{Row, FromRow};

use crate::AppState;
use crate::database::collections::utils::check_table::table_exists;

/// Ensure table exists and is cached
async fn ensure_table_cached(state: &AppState, table: &str) -> Result<(), StatusCode> {
    let is_cached = {
        let cache = state.table_cache.read().unwrap();
        cache.contains(table)
    };

    if !is_cached {
        match table_exists(&state.db, table).await {
            Ok(true) => {
                let mut cache = state.table_cache.write().unwrap();
                cache.insert(table.to_string());
            }
            Ok(false) => return Err(StatusCode::NOT_FOUND),
            Err(code) => return Err(code),
        }
    }

    Ok(())
}

/// Handle database query errors consistently
fn handle_db_error(state: &AppState, table: &str, error: sqlx::Error) -> StatusCode {
    let msg = error.to_string();
    let looks_like_missing_table =
        msg.contains("42P01") ||
        msg.contains("relation") && msg.contains("does not exist");

    if looks_like_missing_table {
        let mut cache = state.table_cache.write().unwrap();
        cache.remove(table);
        return StatusCode::NOT_FOUND;
    }

    tracing::error!("failed to query table {table}. Error: {error} Message: {msg}");
    StatusCode::INTERNAL_SERVER_ERROR
}

/// Convert a single row to JSON Value
fn row_to_json(row: sqlx::postgres::PgRow) -> Value {
    row.try_get("row").unwrap_or(Value::Null)
}

/// Convert multiple rows to JSON Vec
fn rows_to_json(rows: Vec<sqlx::postgres::PgRow>) -> Vec<Value> {
    rows.into_iter().map(row_to_json).collect()
}

/// Execute a query and return a single optional row
async fn fetch_single_row(
    state: &AppState,
    query: &str,
    table: &str,
) -> Result<Option<Value>, StatusCode> {
    let row_opt = sqlx::query(query)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| handle_db_error(state, table, e))?;

    Ok(row_opt.map(row_to_json))
}

/// Execute a query and return multiple rows
async fn fetch_multiple_rows(
    state: &AppState,
    query: &str,
    table: &str,
) -> Result<Vec<Value>, StatusCode> {
    let rows = sqlx::query(query)
        .fetch_all(&state.db)
        .await
        .map_err(|e| handle_db_error(state, table, e))?;

    Ok(rows_to_json(rows))
}

pub async fn get_all(
    State(state): State<AppState>,
    Path(table): Path<String>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    ensure_table_cached(&state, &table).await?;

    let query = format!(r#"SELECT to_jsonb(t) AS row FROM "{}" t"#, table);
    let rows = fetch_multiple_rows(&state, &query, &table).await?;
    Ok(Json(rows))
}

#[derive(FromRow)]
struct PrimaryKeyColumn {
    column_name: String,
}

async fn get_primary_key_column(
    pool: &sqlx::PgPool,
    table: &str,
) -> Result<Option<String>, StatusCode> {
    let pk_rows: Vec<PrimaryKeyColumn> = sqlx::query_as::<_, PrimaryKeyColumn>(
        r#"
        SELECT kcu.column_name
        FROM information_schema.table_constraints tc
        JOIN information_schema.key_column_usage kcu
          ON tc.constraint_name = kcu.constraint_name
          AND tc.table_schema = kcu.table_schema
        WHERE tc.table_schema = 'public'
          AND tc.table_name = $1
          AND tc.constraint_type = 'PRIMARY KEY'
        ORDER BY kcu.ordinal_position
        LIMIT 1
        "#
    )
    .bind(table)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("failed to get primary key column: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(pk_rows.first().map(|r| r.column_name.clone()))
}

/// Fetch a single row by ID, trying integer first, then string
async fn fetch_row_by_id(
    state: &AppState,
    table: &str,
    pk_column: &str,
    id: &str,
) -> Result<Option<Value>, StatusCode> {
    let query = format!(r#"SELECT to_jsonb(t) AS row FROM "{}" t WHERE "{}" = $1 LIMIT 1"#, table, pk_column);

    // Try integer first
    if let Ok(id_int) = id.parse::<i64>() {
        let row_opt = sqlx::query(&query)
            .bind(id_int)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| handle_db_error(state, table, e))?;
        
        if let Some(row) = row_opt {
            return Ok(Some(row_to_json(row)));
        }
    }

    // Fall back to string
    let row_opt = sqlx::query(&query)
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| handle_db_error(state, table, e))?;

    Ok(row_opt.map(row_to_json))
}

#[derive(Deserialize)]
pub struct GetByIdParams {
    id: String,
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(table): Path<String>,
    Query(params): Query<GetByIdParams>,
) -> Result<Json<Option<Value>>, StatusCode> {
    ensure_table_cached(&state, &table).await?;

    let pk_column = get_primary_key_column(&state.db, &table)
        .await?
        .ok_or(StatusCode::BAD_REQUEST)?; // Table has no primary key

    let row = fetch_row_by_id(&state, &table, &pk_column, &params.id).await?;
    Ok(Json(row))
}

pub async fn get_first(
    State(state): State<AppState>,
    Path(table): Path<String>,
) -> Result<Json<Option<Value>>, StatusCode> {
    ensure_table_cached(&state, &table).await?;

    let query = format!(r#"SELECT to_jsonb(t) AS row FROM "{}" t LIMIT 1"#, table);
    let row = fetch_single_row(&state, &query, &table).await?;
    Ok(Json(row))
}

#[derive(Deserialize)]
pub struct PageParams {
    page: Option<u32>,
    per_page: Option<u32>,
}

pub async fn get_page(
    State(state): State<AppState>,
    Path(table): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    ensure_table_cached(&state, &table).await?;

    let page_index = params.page.unwrap_or(0);
    let items_per_page = params.per_page.unwrap_or(10);
    let offset = page_index * items_per_page;

    let query = format!(r#"SELECT to_jsonb(t) AS row FROM "{}" t LIMIT {} OFFSET {}"#, table, items_per_page, offset);
    let rows = fetch_multiple_rows(&state, &query, &table).await?;
    Ok(Json(rows))
}
