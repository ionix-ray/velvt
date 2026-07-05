use crate::error::ServerError;
use axum::{
    body::Body,
    extract::State,
    http::{HeaderValue, Request},
    response::{IntoResponse, Response},
};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct AppState {
    pub static_root: PathBuf,
    pub index_html: PathBuf,
}

/// Insert a header built from a trusted, hardcoded `&'static str` pair.
/// Infallible for the literals used in this module — kept as a helper so
/// call sites never reach for `Result::unwrap`.
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

/// Serve any request — file if it exists, else index.html for SPA.
pub async fn serve_request(
    State(state): State<AppState>,
    req: Request<Body>,
) -> Result<Response, ServerError> {
    let path_str = req.uri().path().trim_start_matches('/');
    tracing::debug!(path = %path_str, "incoming request");

    let requested = state.static_root.join(path_str);
    let requested_exists_as_file =
        !path_str.is_empty() && requested.exists() && requested.is_file();

    if requested_exists_as_file && !is_within_root(&requested, &state.static_root) {
        return Err(ServerError::InvalidPath);
    }

    if requested_exists_as_file {
        let content = fs::read(&requested)
            .await
            .map_err(|e| ServerError::AssetRead(format!("{path_str}: {e}")))?;

        let mime = mime_type(&requested);
        let cache_control = if is_immutable_asset(path_str) {
            "public, max-age=31536000, immutable"
        } else {
            "public, max-age=3600"
        };

        let mut response = Response::new(Body::from(content));
        set_header(&mut response, "content-type", mime);
        set_header(&mut response, "cache-control", cache_control);
        return Ok(response);
    }

    let content = fs::read_to_string(&state.index_html)
        .await
        .map_err(|e| ServerError::AssetRead(format!("index.html: {e}")))?;

    let mut response = Response::new(Body::from(content));
    set_header(&mut response, "content-type", "text/html; charset=utf-8");
    set_header(&mut response, "cache-control", "no-cache");
    Ok(response)
}

/// Lexical `starts_with` on an unresolved path is not a traversal guard —
/// `..` components and symlinks reach real files the OS resolves normally.
/// Canonicalize both sides and compare the resolved paths instead.
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
    path.contains("dxh") && (path.ends_with(".js") || path.ends_with(".wasm"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn temp_dir(name: &str) -> std::io::Result<std::path::PathBuf> {
        let dir = std::env::temp_dir().join(format!(
            "velvet-handlers-test-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    #[test]
    fn is_within_root_accepts_file_directly_under_root() -> TestResult {
        let root = temp_dir("within")?;
        let file = root.join("index.html");
        std::fs::write(&file, "ok")?;

        assert!(is_within_root(&file, &root));

        let _ = std::fs::remove_dir_all(&root);
        Ok(())
    }

    #[test]
    fn is_within_root_rejects_literal_dotdot_escape() -> TestResult {
        let root = temp_dir("escape-root")?;
        let outside = temp_dir("escape-outside")?.join("secret.txt");
        std::fs::write(&outside, "secret")?;

        let Some(outside_dir) = outside.parent() else {
            return Err("outside file has no parent dir".into());
        };
        let Some(outside_dir_name) = outside_dir.file_name() else {
            return Err("outside parent dir has no name".into());
        };

        // Lexically joins to `root/../escape-outside-<pid>/secret.txt`,
        // which resolves to a real file outside `root` once canonicalized.
        let escaping = root.join("..").join(outside_dir_name).join("secret.txt");

        assert!(!is_within_root(&escaping, &root));

        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(outside_dir);
        Ok(())
    }

    #[test]
    fn is_within_root_rejects_nonexistent_candidate() -> TestResult {
        let root = temp_dir("missing-candidate")?;
        let missing = root.join("does-not-exist.html");

        assert!(!is_within_root(&missing, &root));

        let _ = std::fs::remove_dir_all(&root);
        Ok(())
    }

    #[test]
    fn mime_type_covers_every_known_extension() {
        assert_eq!(mime_type(Path::new("a.html")), "text/html; charset=utf-8");
        assert_eq!(
            mime_type(Path::new("a.js")),
            "application/javascript; charset=utf-8"
        );
        assert_eq!(
            mime_type(Path::new("a.mjs")),
            "application/javascript; charset=utf-8"
        );
        assert_eq!(mime_type(Path::new("a.wasm")), "application/wasm");
        assert_eq!(mime_type(Path::new("a.css")), "text/css; charset=utf-8");
        assert_eq!(mime_type(Path::new("a.png")), "image/png");
        assert_eq!(mime_type(Path::new("a.jpg")), "image/jpeg");
        assert_eq!(mime_type(Path::new("a.jpeg")), "image/jpeg");
        assert_eq!(mime_type(Path::new("a.svg")), "image/svg+xml");
        assert_eq!(mime_type(Path::new("a.ico")), "image/x-icon");
        assert_eq!(mime_type(Path::new("a.woff2")), "font/woff2");
        assert_eq!(mime_type(Path::new("a.woff")), "font/woff");
        assert_eq!(mime_type(Path::new("a.ttf")), "font/ttf");
        assert_eq!(mime_type(Path::new("a.json")), "application/json");
        assert_eq!(
            mime_type(Path::new("a.unknown")),
            "application/octet-stream"
        );
        assert_eq!(mime_type(Path::new("noext")), "application/octet-stream");
    }

    #[test]
    fn is_immutable_asset_requires_hash_marker_and_extension() {
        assert!(is_immutable_asset("assets/app-dxh1234.wasm"));
        assert!(is_immutable_asset("assets/app-dxh1234.js"));
        assert!(!is_immutable_asset("assets/app.wasm"));
        assert!(!is_immutable_asset("assets/app-dxh1234.css"));
        assert!(!is_immutable_asset("index.html"));
    }
}
