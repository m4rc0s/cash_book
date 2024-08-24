use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),
}