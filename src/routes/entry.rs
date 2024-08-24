use axum::{extract::State, routing::post, Json, Router};

use super::{error::Error, AppJson};
use crate::{
    AppState,
    service::create_entry::{self, CreateEntryRequest, CreateEntryResponse}
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_entry))
}

pub async fn create_entry(
    State(state): State<AppState>, 
    Json(request): Json<CreateEntryRequest>
)-> Result<AppJson<CreateEntryResponse>, Error> {
    let res = create_entry::execute(state.db_conn_pool.clone(), request).await?;
    Ok(AppJson(res))
}
