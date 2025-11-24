use axum::{
    Router,
    routing::{get, post},
};
use http::{Method, header};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tokio_postgres::NoTls;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use std::collections::HashSet;
use std::sync::{Arc, RwLock};


mod database;
mod development;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub db_url: String,
    pub table_cache: Arc<RwLock<HashSet<String>>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_url = "postgres://postgres:Password1@localhost:5433/project";

    let (mut client, connection) = tokio_postgres::connect(db_url, NoTls).await.expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {e}");
        }
    });

    database::utils::migration_runner::runner_from_fs()
        .expect("could not load migrations")
        .run_async(&mut client)
        .await
        .expect("Failed to run migrations");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("failed to create sqlx pool");

    let state = AppState {
        db: pool,
        db_url: db_url.to_string(),
        table_cache: Arc::new(RwLock::new(HashSet::new())),
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(Any);

    // TODO: hide /development routes when not in dev.
    let app = Router::new()
        .route("/", get(|| async { "ok" }))
        .route("/health", get(|| async { "ok" }))
        .route("/database/table", post(database::post_table::create))
        .route("/database/table", get(database::get_tables::get_tables))
        .route("/development/typedefs", get(development::get_typescript_types::get_typedefs))
        // Collection routes - more specific routes first
        .route("/collection/{table}/first", get(database::collections::get_first))
        .route("/collection/{table}/page", get(database::collections::get_page))
        .route("/collection/{table}/get", get(database::collections::get_by_id))
        .route("/collection/{table}", get(database::collections::get_all))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let ip = "0.0.0.0";
    let port = "3000";
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", ip, port))
        .await
        .unwrap();

    tracing::info!("Listening on {}:{}", ip, port);

    axum::serve(listener, app).await.unwrap();
}
