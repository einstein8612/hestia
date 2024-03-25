use crate::api::AppState;

use axum::{
    extract::State, http::StatusCode, response::{IntoResponse, Response}, Json
};

#[axum::debug_handler]
pub async fn get_statistics(
    State(state): State<AppState>,
) -> Response {
    let data_manager = state.data_manager.lock().await;

    match data_manager.fetch_statistics().await {
        Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
