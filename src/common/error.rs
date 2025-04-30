use actix_web::{
    body::BoxBody,
    http::{
        header::{self, TryIntoHeaderValue},
        StatusCode,
    },
    HttpResponse, ResponseError,
};
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
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut res = HttpResponse::new(self.status_code());

        match self {
            Self::ValidationError(validation_errors) => {
                insert_content_type_header(&mut res, header::ContentType::json());
                res = res.set_body(BoxBody::new(
                    serde_json::to_string(&validation_errors.field_errors()).unwrap(),
                ));
            }
            Self::InternalError(_) => {
                insert_content_type_header(&mut res, header::ContentType::plaintext());
                res = res.set_body(BoxBody::new(self.to_string()));
            }
        }

        res
    }
}

fn insert_content_type_header(res: &mut HttpResponse, content_type: header::ContentType) {
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        content_type.0.try_into_value().unwrap(),
    );
}
