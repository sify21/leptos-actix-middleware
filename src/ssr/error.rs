use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("{:?}", .0)]
    Blocking(#[from] actix_web::error::BlockingError),
    #[error("IO Err：{:?}", .0)]
    IO(#[from] std::io::Error),
    #[error("Invalid params：{:?}", .0)]
    Json(#[from] serde_json::Error),
    #[error("{}", .0)]
    String(String),
    #[error("{}", .0)]
    Str(&'static str),
    #[error("Unauthenticated")]
    Unauthenticated,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("No Content")]
    Nocontent,
}
impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(json!({"err": self.to_string()}))
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Json(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::FORBIDDEN,
            Self::Unauthenticated => StatusCode::UNAUTHORIZED,
            Self::Nocontent => StatusCode::NO_CONTENT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
