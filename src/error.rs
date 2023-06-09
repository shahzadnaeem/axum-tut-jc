use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    DbError(String),

    LoginFailed,

    AuthNoCookie,
    AuthTokenInvalid,
    AuthNoContextFound,

    DeleteTicketNotFound { id: u64 },
    // NOTE: If a new Error is added, then it will cause a compiler error...
    // NewErrorShouldTriggerCompilerError,
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::DbError(value.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // Create an initial Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Now insert the Error
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn to_client_error(&self) -> StatusAndClientError {
        match self {
            Self::DbError(_msg) => (StatusCode::BAD_REQUEST, ClientError::InternalError),

            Self::DeleteTicketNotFound { .. } => {
                (StatusCode::NOT_FOUND, ClientError::InvalidParams)
            }

            Self::AuthNoCookie | Self::AuthTokenInvalid | Self::AuthNoContextFound => {
                (StatusCode::FORBIDDEN, ClientError::NoAuth)
            }

            Self::LoginFailed => (StatusCode::FORBIDDEN, ClientError::NoAuth),
            // NOTE: Not needed as Rust will spot any missing patterns
            // _ => (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     ClientError::InternalError,
            // ),
        }
    }
}

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum ClientError {
    LoginFailed,
    NoAuth,
    InvalidParams,
    InternalError,
}

type StatusAndClientError = (StatusCode, ClientError);
