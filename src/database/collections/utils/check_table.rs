use sqlx::PgPool;
use http::StatusCode;

/// Check if a table exists in the current database (public schema) and is not a system/migration table.
/// 
/// Returns:
/// - `Ok(true)` if the table exists
/// - `Ok(false)` if it doesn't exist
/// - `Err(StatusCode::INTERNAL_SERVER_ERROR)` if the query failed
pub async fn table_exists(pool: &PgPool, table_name: &str) -> Result<bool, StatusCode> {
    let exists = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM information_schema.tables
        WHERE table_schema = 'public'
          AND table_type = 'BASE TABLE'
          AND table_name = $1
          AND table_name NOT LIKE 'refinery_%'
        "#
    )
    .bind(table_name)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("failed to check table existence: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(exists > 0)
}

