use crate::api::AppState;
use crate::key;
use crate::language::Language;

use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::{extract::State, http::StatusCode};
use chrono::{SubsecRound, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateBinPayload {
    name: String,
    description: String,
    filename: String,
    content: Vec<u8>,
    language: Language,
}

pub async fn create_bin(State(state): State<AppState>, Json(bin_data) :Json<CreateBinPayload>) -> Response {
    let mut data_manager = state.data_manager.lock().await;

    match data_manager.create_bin(
        key::generate_key(),
        bin_data.name,
        bin_data.description,
        bin_data.filename,
        bin_data.content,
        bin_data.language,
        Utc::now().naive_utc().round_subsecs(3),
    ).await {
        Ok(new_bin) => (StatusCode::OK, Json(new_bin)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
