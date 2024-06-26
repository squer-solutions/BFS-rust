use axum::response::IntoResponse;
use axum::http::StatusCode;
use tracing::error;
use crate::data::data_errors::DataError;

impl IntoResponse for DataError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            DataError::NotFound => StatusCode::NOT_FOUND.into_response(),
            DataError::Duplicate => StatusCode::CONFLICT.into_response(),
            DataError::InternalServerError(_) => {
                error!("Internal server error, {:?}", self);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
        }
    }
}