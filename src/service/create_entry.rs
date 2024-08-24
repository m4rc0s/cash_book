use crate::repository::entry::{self as entry_repo, EntryEntity};

use super::error::Error;
use bigdecimal::{BigDecimal, FromPrimitive};
use serde::{Deserialize, Serialize};
use sqlx::{types::time::Date, PgPool};
use time::macros::format_description;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateEntryRequest {
    pub(crate) kind: String,
    pub(crate) amount: f32,
    pub(crate) date: String,
    pub(crate) description: String
}

#[derive(Serialize)]
pub struct CreateEntryResponse {
    id: Uuid
}


pub async fn execute(db_conn_pool: PgPool, request: CreateEntryRequest) -> Result<CreateEntryResponse, Error>{
    let mut conn = db_conn_pool.acquire().await?.detach();
    let new_entry: EntryEntity = request.try_into()?;
    let id = entry_repo::insert(&mut conn, new_entry).await?;
    Ok(CreateEntryResponse { id })
}


impl TryFrom<CreateEntryRequest> for EntryEntity {
    type Error = Error;
    fn try_from(value: CreateEntryRequest) -> Result<Self, Self::Error> {
        let a = format_description!("[year]-[month]-[day]");
        Ok(Self {
            id: Uuid::new_v4(),
            kind: value.kind,
            amount: BigDecimal::from_f32(value.amount).unwrap().round(2),
            movement_date: Date::parse(&value.date, &a).unwrap(),
            description: value.description
        })
    }
}