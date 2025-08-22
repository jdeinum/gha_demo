use crate::{
    app::AppState,
    routes::v1::cats::{
        get::{get_all_cats, get_cat},
        post::create_cat,
    },
};
use axum::{Router, routing::get};

pub fn get_v1_router() -> Router<AppState> {
    Router::new()
        .route("/cats", get(get_all_cats).post(create_cat))
        .route("/cats/{cool_cat_club_id}", get(get_cat))
}
