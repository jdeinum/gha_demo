use crate::app::AppState;
use crate::error::Result;
use crate::types::v1::types::Cat;
use axum::{Json, extract::State, http::StatusCode};

pub async fn create_cat(
    State(app_state): State<AppState>,
    Json(cat): Json<Cat>,
) -> Result<(StatusCode, Json<Cat>)> {
    cat.write_to_db(&app_state.db).await?;

    // a little wasteful we reserialize, but ok for this
    Ok((StatusCode::CREATED, Json(cat)))
}
