use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use failure::Fail;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
    pub message: String,
    pub code: u16,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.errors.join(", "))
    }
}
// ================================================================================

#[derive(Debug, Fail, Serialize)]
pub enum ServiceError {
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    #[fail(display = "Not Found")]
    NotFound { message: String },

    #[fail(display = "Bad Request")]
    BadRequest { message: String },
}

impl ServiceError {
    fn name(&self) -> String {
        match self {
            Self::BadRequest { message } => message.to_string(),
            Self::InternalServerError => "Internal Server Error".to_string(),
            Self::NotFound { message } => message.to_string(),
        }
    }
}


impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        let code = self.status_code();
        let error_resp = ErrorResponse {
            code: code.as_u16(),
            errors: vec![self.name()],
            message: self.to_string(),
        };
        HttpResponse::build(code).json(error_resp)
    }
}
