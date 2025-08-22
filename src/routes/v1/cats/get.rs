use crate::{
    app::AppState,
    error::{Error, Result},
    types::v1::types::Cat,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

pub async fn get_all_cats(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Cat>>)> {
    // fetch all cats from the database
    let cats = sqlx::query_as::<_, Cat>("SELECT * FROM cats")
        .fetch_all(&app_state.db)
        .await?;

    Ok((StatusCode::OK, Json(cats)))
}

pub async fn get_cat(
    State(app_state): State<AppState>,
    Path(cool_cat_club_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Cat>)> {
    // fetch all cats from the database
    let cat = sqlx::query_as::<_, Cat>("SELECT * FROM cats WHERE cool_cat_club_id = $1")
        .bind(cool_cat_club_id)
        .fetch_optional(&app_state.db)
        .await?
        .ok_or(Error::NotFoundError)?;

    Ok((StatusCode::OK, Json(cat)))
}
