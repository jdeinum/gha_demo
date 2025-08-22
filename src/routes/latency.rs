use crate::{app::AppState, error::Result};
use axum::{extract::State, http::StatusCode};
use rand::Rng;
use std::time::Duration;

const UPPER_RANGE: u16 = 1000;

pub async fn latency(State(mut app_state): State<AppState>) -> Result<StatusCode> {
    // (simulate work)
    let millis_to_work = app_state.rng.random_range(0..UPPER_RANGE);
    tokio::time::sleep(Duration::from_millis(millis_to_work.into())).await;

    Ok(StatusCode::OK)
}
