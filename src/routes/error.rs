use super::AppJson;
use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Repository(#[from] crate::repository::error::Error),

    #[error(transparent)]
    Domain(#[from] crate::service::error::Error),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            e => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}