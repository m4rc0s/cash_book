use super::error::Error;
use sqlx::{types::{time::Date, BigDecimal}, PgConnection};
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct EntryEntity {
    pub(crate) id: Uuid,
    pub(crate) amount: BigDecimal,
    pub(crate) kind: String,
    pub(crate) description: String,
    pub(crate) movement_date: Date
}

pub(crate) async fn insert(db_conn: &mut PgConnection, new_entry:  EntryEntity) -> Result<Uuid, Error> {
    println!("Running in {:?} mode", new_entry);

    let row = sqlx::query!(
        r#"
        INSERT INTO entry (id, kind, amount, movement_date, description) 
        VALUES($1, $2, $3, $4, $5) RETURNING id
        "#,
        new_entry.id,
        new_entry.kind,
        new_entry.amount,
        new_entry.movement_date,
        new_entry.description
    )
    .fetch_one(&mut *db_conn)
    .await?;

    Ok(row.id)
}
