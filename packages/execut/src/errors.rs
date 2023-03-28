use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum Error {
    Internal,
    InvalidRequest,
    InvalidToken,
    MissingCredentials,
    Unauthorized,
    UnknownUser,
    DuplicateUser,
    UnknownBadge,
    WrongCredentials,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
            Self::InvalidRequest => (StatusCode::UNPROCESSABLE_ENTITY, "invalid request"),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "invalid token"),
            Self::MissingCredentials => (StatusCode::UNPROCESSABLE_ENTITY, "no credentials found"),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "not authorized"),
            Self::UnknownUser => (StatusCode::NOT_FOUND, "unknown user"),
            Self::DuplicateUser => (StatusCode::CONFLICT, "this user already exists"),
            Self::UnknownBadge => (StatusCode::NOT_FOUND, "unknown badge"),
            Self::WrongCredentials => (StatusCode::UNPROCESSABLE_ENTITY, "wrong credentials"),
        };

        let body = Json(json!({
            "detail": message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
