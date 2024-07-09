use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("not found")]
    NotFound,
    #[error("entity already exists")]
    Duplicate,
    #[error("internal server error")]
    InternalServerError(anyhow::Error),
}

impl From<anyhow::Error> for DataError {
    fn from(err: anyhow::Error) -> Self {
        DataError::InternalServerError(err)
    }
}
