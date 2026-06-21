//! Error types for velvet-server.

use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

/// Top-level server error.
#[derive(Debug)]
pub enum ServerError {
    AddrParse(String),
    Bind(std::io::Error),
    Serve(std::io::Error),
    AssetNotFound(String),
    AssetRead(String),
    InvalidPath,
    ConfigRead(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddrParse(msg) => write!(f, "address parse error: {msg}"),
            Self::Bind(e) => write!(f, "bind error: {e}"),
            Self::Serve(e) => write!(f, "serve error: {e}"),
            Self::AssetNotFound(p) => write!(f, "asset not found: {p}"),
            Self::AssetRead(p) => write!(f, "asset read error: {p}"),
            Self::InvalidPath => write!(f, "invalid path (possible directory traversal)"),
            Self::ConfigRead(p) => write!(f, "config read error: {p}"),
        }
    }
}

impl std::error::Error for ServerError {}

/// Convert a [`ServerError`] into an HTTP response.
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            Self::AssetNotFound(_) => (StatusCode::NOT_FOUND, Body::from("Not Found")),
            Self::InvalidPath => (StatusCode::BAD_REQUEST, Body::from("Bad Request")),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::from("Internal Server Error"),
            ),
        };

        let mut response = Response::new(body);
        *response.status_mut() = status;
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("text/plain"),
        );
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_messages() {
        assert_eq!(
            format!("{}", ServerError::AddrParse("bad".to_string())),
            "address parse error: bad"
        );
        assert_eq!(
            format!("{}", ServerError::AssetNotFound("/x".to_string())),
            "asset not found: /x"
        );
        assert_eq!(
            format!("{}", ServerError::AssetRead("/x".to_string())),
            "asset read error: /x"
        );
        assert_eq!(
            format!("{}", ServerError::InvalidPath),
            "invalid path (possible directory traversal)"
        );
        assert_eq!(
            format!("{}", ServerError::ConfigRead("/x".to_string())),
            "config read error: /x"
        );
    }

    #[test]
    fn into_response_maps_status_codes() -> Result<(), Box<dyn std::error::Error>> {
        let resp = ServerError::AssetNotFound("/x".to_string()).into_response();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let resp = ServerError::InvalidPath.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let resp = ServerError::AssetRead("/x".to_string()).into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let Some(content_type) = resp.headers().get("content-type").cloned() else {
            return Err("missing content-type header".into());
        };
        assert_eq!(content_type, "text/plain");
        Ok(())
    }
}
