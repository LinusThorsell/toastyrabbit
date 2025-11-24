use super::{structs, utils::create_migration_file};
use axum::{Json, extract::State};
use chrono::Utc;
use refinery::config::Config;
use sea_query::PostgresQueryBuilder;
use std::str::FromStr;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{AppState, database::utils};

pub async fn create(State(state): State<AppState>, Json(payload): Json<structs::TableSpec>) -> () {
    let migration_string = create_migration_file::build_create(&payload)
        .to_string(PostgresQueryBuilder)
        .to_string();

    println!("Creating migration file: {}", migration_string);

    let _ = File::create(format!(
        "migrations/U{}__create_table_{}.sql",
        Utc::now().timestamp(),
        payload.table
    ))
    .await
    .expect("Could not create migration file.")
    .write_all(migration_string.as_bytes())
    .await
    .expect("Could not write to migration file.");

    utils::migration_runner::runner_from_fs()
        .expect("could not load migrations")
        .run_async(&mut Config::from_str(&state.db_url).expect("Database URL is not set."))
        .await
        .expect("Failed to run migrations");
}
