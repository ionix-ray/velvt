use crate::error::ServerError;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct AppState {
    pub static_root: PathBuf,
    pub index_html: PathBuf,
}

pub async fn health_check() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(r#"{"status":"ok","service":"velvet-server"}"#))
        .unwrap()
}

/// Serve any request — file if it exists, else index.html for SPA.
pub async fn serve_request(
    State(state): State<AppState>,
    req: Request<Body>,
) -> Result<Response, ServerError> {
    let path_str = req.uri().path().trim_start_matches('/');
    tracing::debug!(path = %path_str, "incoming request");

    let requested = state.static_root.join(path_str);

    let is_valid = !path_str.is_empty()
        && requested.starts_with(&state.static_root)
        && requested.exists()
        && requested.is_file();

    if is_valid {
        let content = fs::read(&requested)
            .await
            .map_err(|e| ServerError::AssetRead(format!("{}: {e}", path_str)))?;

        let mime = mime_type(&requested);
        let cache_control = if is_immutable_asset(path_str) {
            "public, max-age=31536000, immutable"
        } else {
            "public, max-age=3600"
        };

        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", mime)
            .header("cache-control", cache_control)
            .body(Body::from(content))
            .unwrap());
    }

    let content = fs::read_to_string(&state.index_html)
        .await
        .map_err(|e| ServerError::AssetRead(format!("index.html: {e}")))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html; charset=utf-8")
        .header("cache-control", "no-cache")
        .body(Body::from(content))
        .unwrap())
}

fn mime_type(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") | Some("mjs") => "application/javascript; charset=utf-8",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css; charset=utf-8",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        Some("ttf") => "font/ttf",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

fn is_immutable_asset(path: &str) -> bool {
    path.contains("dxh") && (path.ends_with(".js") || path.ends_with(".wasm"))
}
