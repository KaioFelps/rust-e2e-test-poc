use actix_web::{http::StatusCode, ResponseError};
use inertia_rust::InertiaError;

pub type AppResult<T, E = AppError> = Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("An unexpected error occurred.")]
    InternalError(#[from] anyhow::Error),

    #[error("Some input(s) has not been correctly fulfilled.")]
    ValidationError(
        #[from]
        #[source]
        validator::ValidationErrors,
    ),
}

impl From<InertiaError> for AppError {
    fn from(value: InertiaError) -> Self {
        Self::InternalError(value.into())
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match &self {
            AppError::InternalError(err) => {
                log::error!("{err}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::ValidationError(_) => StatusCode::BAD_GATEWAY,
        }
    }
}
