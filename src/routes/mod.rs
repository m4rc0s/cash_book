pub(crate) mod entry;
pub(crate) mod error;


use crate::AppState;
use axum::{
    extract::FromRequest, http::StatusCode, response::IntoResponse, routing::get, Json, Router,
};
use error::Error;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Error))]
pub(crate) struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    Json<T>: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        Json(self.0).into_response()
    }
}


pub(crate) fn routes(state: AppState) -> Router {
    Router::new()
        .route("/health-check", get(health_check))
        .nest("/entry", entry::routes())
        .with_state(state)
}

pub(crate) async fn health_check() -> StatusCode {
    StatusCode::OK
}
