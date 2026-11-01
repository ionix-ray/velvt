use crate::error::ServerError;
use axum::{
    body::Body,
    extract::State,
    http::{HeaderValue, Request},
    response::{IntoResponse, Response},
};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AppState {
    pub static_root: PathBuf,
    pub index_html: PathBuf,
}

fn set_header(response: &mut Response, name: &'static str, value: &'static str) {
    response
        .headers_mut()
        .insert(name, HeaderValue::from_static(value));
}

pub async fn health_check() -> impl IntoResponse {
    let mut response = Response::new(Body::from(r#"{"status":"ok","service":"velvet-server"}"#));
    set_header(&mut response, "content-type", "application/json");
    response
}

#[derive(Debug)]
enum ServeResult {
    File {
        content: Vec<u8>,
        content_type: &'static str,
        cache_control: &'static str,
        encoding: Option<&'static str>,
    },
    Index {
        content: String,
    },
}

fn serve_blocking(
    static_root: &Path,
    index_html: &Path,
    path_str: &str,
) -> Result<ServeResult, ServerError> {
    if path_str.is_empty() {
        let content = std::fs::read_to_string(index_html)
            .map_err(|e| ServerError::AssetRead(format!("index.html: {e}")))?;
        return Ok(ServeResult::Index { content });
    }

    let requested = static_root.join(path_str);
    if !requested.exists() || !requested.is_file() {
        let content = std::fs::read_to_string(index_html)
            .map_err(|e| ServerError::AssetRead(format!("index.html: {e}")))?;
        return Ok(ServeResult::Index { content });
    }

    if !is_within_root(&requested, static_root) {
        return Err(ServerError::InvalidPath);
    }

    let mime = mime_type(&requested);
    let cache_control = if is_immutable_asset(path_str) {
        "public, max-age=31536000, immutable"
    } else {
        "public, max-age=3600"
    };

    let mut gz_path = requested.clone();
    if let Some(name) = gz_path.file_name().and_then(|n| n.to_str()) {
        let mut new_name = name.to_string();
        new_name.push_str(".gz");
        gz_path.set_file_name(new_name);
    }

    if gz_path.is_file() {
        if !is_within_root(&gz_path, static_root) {
            let content = std::fs::read(&requested)
                .map_err(|e| ServerError::AssetRead(format!("{path_str}: {e}")))?;
            return Ok(ServeResult::File {
                content,
                content_type: mime,
                cache_control,
                encoding: None,
            });
        }
        let content = std::fs::read(&gz_path)
            .map_err(|e| ServerError::AssetRead(format!("{path_str}.gz: {e}")))?;
        return Ok(ServeResult::File {
            content,
            content_type: mime,
            cache_control,
            encoding: Some("gzip"),
        });
    }

    let content = std::fs::read(&requested)
        .map_err(|e| ServerError::AssetRead(format!("{path_str}: {e}")))?;
    Ok(ServeResult::File {
        content,
        content_type: mime,
        cache_control,
        encoding: None,
    })
}

pub async fn serve_request(
    State(state): State<AppState>,
    req: Request<Body>,
) -> Result<Response, ServerError> {
    let path_str = req.uri().path().trim_start_matches('/').to_string();
    tracing::debug!(path = %path_str, "incoming request");

    let static_root = state.static_root.clone();
    let index_html = state.index_html.clone();
    let path_str_clone = path_str;

    let result = tokio::task::spawn_blocking(move || {
        serve_blocking(&static_root, &index_html, &path_str_clone)
    })
    .await
    .map_err(|e| ServerError::AssetRead(format!("spawn_blocking: {e}")))??;

    match result {
        ServeResult::File {
            content,
            content_type,
            cache_control,
            encoding,
        } => {
            let mut response = Response::new(Body::from(content));
            set_header(&mut response, "content-type", content_type);
            set_header(&mut response, "cache-control", cache_control);
            if let Some(enc) = encoding {
                set_header(&mut response, "content-encoding", enc);
                set_header(&mut response, "vary", "Accept-Encoding");
            }
            Ok(response)
        }
        ServeResult::Index { content } => {
            let mut response = Response::new(Body::from(content));
            set_header(&mut response, "content-type", "text/html; charset=utf-8");
            set_header(&mut response, "cache-control", "no-cache");
            Ok(response)
        }
    }
}

fn is_within_root(candidate: &std::path::Path, root: &std::path::Path) -> bool {
    let (Ok(canonical_candidate), Ok(canonical_root)) =
        (candidate.canonicalize(), root.canonicalize())
    else {
        return false;
    };
    canonical_candidate.starts_with(canonical_root)
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
    if !path.contains("dxh") {
        return false;
    }
    matches!(
        path.rsplit('.').next().unwrap_or(""),
        "js" | "wasm" | "css" | "woff2" | "woff" | "ttf"
    )
}
