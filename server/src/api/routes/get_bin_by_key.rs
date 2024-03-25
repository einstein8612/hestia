use crate::api::AppState;

use axum::{
    extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, Json
};

#[axum::debug_handler]
pub async fn get_bin_by_key(
    State(state): State<AppState>,
    Path(bin_key): Path<String>,
) -> Response {
    let data_manager = state.data_manager.lock().await;

    match data_manager.find_bin_by_key(&bin_key).await {
        Ok(bin) => (StatusCode::OK, Json(bin)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response()
    }
}
