use diesel::result::Error;
use tracing::error;

use crate::data::data_errors::DataError;

impl From<diesel::result::Error> for DataError {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound => DataError::NotFound,
            Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => DataError::Duplicate,
            _ => {
                error!("Database error: {:?}", value);
                DataError::InternalServerError(value.into())
            }
        }
    }
}

impl From<r2d2::Error> for DataError {
    fn from(err: r2d2::Error) -> Self {
        DataError::InternalServerError(err.into())
    }
}
