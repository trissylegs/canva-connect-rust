//! Error types for the Canva Connect API client

use std::fmt;
use thiserror::Error;

/// Result type alias for this crate
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for the Canva Connect API client
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// API error response from Canva
    #[error("API error: {code} - {message}")]
    Api {
        /// Error code from the API
        code: ApiErrorCode,
        /// Error message from the API
        message: String,
    },

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimit,

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic error with message
    #[error("{0}")]
    Generic(String),

    /// Invalid header value
    #[error("Invalid header value: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    /// HTTP client build error
    #[error("Failed to build HTTP client: {0}")]
    ClientBuild(reqwest::Error),
}

/// API error codes returned by the Canva Connect API
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiErrorCode {
    /// Invalid request
    InvalidRequest,
    /// Unauthorized
    Unauthorized,
    /// Forbidden
    Forbidden,
    /// Not found
    NotFound,
    /// Method not allowed
    MethodNotAllowed,
    /// Conflict
    Conflict,
    /// Unprocessable entity
    UnprocessableEntity,
    /// Too many requests
    TooManyRequests,
    /// Internal server error
    InternalServerError,
    /// Service unavailable
    ServiceUnavailable,
    /// Unknown error code
    Unknown(String),
}

impl fmt::Display for ApiErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiErrorCode::InvalidRequest => write!(f, "INVALID_REQUEST"),
            ApiErrorCode::Unauthorized => write!(f, "UNAUTHORIZED"),
            ApiErrorCode::Forbidden => write!(f, "FORBIDDEN"),
            ApiErrorCode::NotFound => write!(f, "NOT_FOUND"),
            ApiErrorCode::MethodNotAllowed => write!(f, "METHOD_NOT_ALLOWED"),
            ApiErrorCode::Conflict => write!(f, "CONFLICT"),
            ApiErrorCode::UnprocessableEntity => write!(f, "UNPROCESSABLE_ENTITY"),
            ApiErrorCode::TooManyRequests => write!(f, "TOO_MANY_REQUESTS"),
            ApiErrorCode::InternalServerError => write!(f, "INTERNAL_SERVER_ERROR"),
            ApiErrorCode::ServiceUnavailable => write!(f, "SERVICE_UNAVAILABLE"),
            ApiErrorCode::Unknown(code) => write!(f, "{code}"),
        }
    }
}

impl From<String> for ApiErrorCode {
    fn from(code: String) -> Self {
        match code.as_str() {
            "INVALID_REQUEST" => ApiErrorCode::InvalidRequest,
            "UNAUTHORIZED" => ApiErrorCode::Unauthorized,
            "FORBIDDEN" => ApiErrorCode::Forbidden,
            "NOT_FOUND" => ApiErrorCode::NotFound,
            "METHOD_NOT_ALLOWED" => ApiErrorCode::MethodNotAllowed,
            "CONFLICT" => ApiErrorCode::Conflict,
            "UNPROCESSABLE_ENTITY" => ApiErrorCode::UnprocessableEntity,
            "TOO_MANY_REQUESTS" => ApiErrorCode::TooManyRequests,
            "INTERNAL_SERVER_ERROR" => ApiErrorCode::InternalServerError,
            "SERVICE_UNAVAILABLE" => ApiErrorCode::ServiceUnavailable,
            _ => ApiErrorCode::Unknown(code),
        }
    }
}

/// Error response from the API
#[derive(Debug, serde::Deserialize)]
pub struct ApiError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
}

impl From<ApiError> for Error {
    fn from(api_error: ApiError) -> Self {
        Error::Api {
            code: ApiErrorCode::from(api_error.code),
            message: api_error.message,
        }
    }
}
