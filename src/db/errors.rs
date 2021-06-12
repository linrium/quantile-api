use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("pool not found")]
    PoolNotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
