use axum::{Json, extract::State};
use http::StatusCode;

use crate::AppState;
use crate::database::structs;
use crate::database::utils::introspect_table::load_table_specs;

pub async fn get_tables(
    State(state): State<AppState>,
) -> Result<Json<Vec<structs::TableSpec>>, StatusCode> {
    match load_table_specs(&state.db).await {
        Ok(specs) => Ok(Json(specs)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
