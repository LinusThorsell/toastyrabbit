use super::{structs, utils::create_migration_file};
use sea_query::PostgresQueryBuilder;
use axum::Json;
use tokio::{fs::File, io::AsyncWriteExt};
use chrono::Utc;

pub async fn create(Json(payload): Json<structs::TableSpec>) -> () {
    let migration_string = create_migration_file::build_create(&payload).to_string(PostgresQueryBuilder).to_string();

    println!("Creating migration file: {}", migration_string);

    let _ = File::create(format!("migrations/U{}__create_table_{}.sql", Utc::now().timestamp(), payload.table)).await.expect("Could not create migration file.").write_all(migration_string.as_bytes()).await.expect("Could not write to migration file.");
}
