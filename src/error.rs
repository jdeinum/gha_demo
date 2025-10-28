use axum::http::StatusCode;
use axum::{body::Body, http::Response, response::IntoResponse};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("Unexpected Error")]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Database Error")]
    DbError(#[from] sqlx::Error),
    #[error("Cat Not Found")]
    NotFoundError,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match &self {
            Error::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFoundError => StatusCode::NOT_FOUND,
        }
    }

    pub fn body(&self) -> Body {
        match &self {
            Error::UnexpectedError(_) => Body::empty(),
            Error::DbError(_) => Body::empty(),
            Error::NotFoundError => Body::empty(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let body = self.body();
        let status_code = self.status_code();
        error!("Error: {status_code} : {body:?}");
        Response::builder().status(status_code).body(body).unwrap()
    }
}
