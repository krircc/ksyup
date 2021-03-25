//! Custom error module

use ntex::http::ResponseBuilder;
use ntex::web::{HttpRequest, HttpResponse, error::{DefaultError, WebResponseError}};
use ntex::http::{header, StatusCode};
use derive_more::{Display, Error};
use serde::Serialize;

/// Represents the custom error message
#[derive(Serialize)]
pub struct AppErrorMessage {
    pub code: u16,
    pub message: String,
}

/// Defines available errors
#[derive(Display, Debug, Error)]
pub enum AppError {
    #[display(fmt = "{}", message)]
    InternalError { message: String },

    #[display(fmt = "{}", message)]
    BadRequest { message: String },

    #[display(fmt = "{}", message)]
    NotFound { message: String },

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl AppError {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound { message: m } => m.to_owned(),
            Self::BadRequest { message: m } => m.to_owned(),
            Self::InternalError { message: m } => m.to_owned(),
            Self::Unauthorized => "Unauthorized".to_owned(),
        }
    }
}

impl WebResponseError<DefaultError> for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .header(header::CONTENT_TYPE, "application/json")
            .json(&AppErrorMessage {
                code: self.status_code().as_u16(),
                message: self.to_string(),
            })
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            _ => Self::InternalError {
                message: "Database Error".to_owned(),
            },
        }
    }
}
