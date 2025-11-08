use axum::{
    Router,
    routing::{get, post},
};
use http::{Method, header};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tokio_postgres::NoTls;

mod database;

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

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(|| async { "ok" }))
        .route("/health", get(|| async { "ok" }))
        .route("/database/table", post(database::post_table::create))
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
