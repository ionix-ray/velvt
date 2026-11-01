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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use std::fs;

    fn temp_test_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("velvet-handler-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(dir.join("index.html"), "<html>Index</html>").unwrap();
        fs::write(dir.join("style.css"), "body{}").unwrap();
        fs::write(dir.join("bundle.js"), "console.log(1);").unwrap();
        fs::write(dir.join("bundle.js.gz"), b"fake-gzip").unwrap();
        dir
    }

    #[test]
    fn mime_type_covers_all_supported_variants() {
        assert_eq!(
            mime_type(Path::new("test.html")),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            mime_type(Path::new("test.js")),
            "application/javascript; charset=utf-8"
        );
        assert_eq!(
            mime_type(Path::new("test.mjs")),
            "application/javascript; charset=utf-8"
        );
        assert_eq!(mime_type(Path::new("test.wasm")), "application/wasm");
        assert_eq!(mime_type(Path::new("test.css")), "text/css; charset=utf-8");
        assert_eq!(mime_type(Path::new("test.png")), "image/png");
        assert_eq!(mime_type(Path::new("test.jpg")), "image/jpeg");
        assert_eq!(mime_type(Path::new("test.jpeg")), "image/jpeg");
        assert_eq!(mime_type(Path::new("test.svg")), "image/svg+xml");
        assert_eq!(mime_type(Path::new("test.ico")), "image/x-icon");
        assert_eq!(mime_type(Path::new("test.woff2")), "font/woff2");
        assert_eq!(mime_type(Path::new("test.woff")), "font/woff");
        assert_eq!(mime_type(Path::new("test.ttf")), "font/ttf");
        assert_eq!(mime_type(Path::new("test.json")), "application/json");
        assert_eq!(mime_type(Path::new("test.bin")), "application/octet-stream");
        assert_eq!(mime_type(Path::new("test")), "application/octet-stream");
    }

    #[test]
    fn is_immutable_asset_checks_dxh_and_extension() {
        assert!(is_immutable_asset("app-dxh123.js"));
        assert!(is_immutable_asset("app-dxh123.wasm"));
        assert!(is_immutable_asset("style-dxh123.css"));
        assert!(is_immutable_asset("font-dxh123.woff2"));
        assert!(is_immutable_asset("font-dxh123.woff"));
        assert!(is_immutable_asset("font-dxh123.ttf"));
        assert!(!is_immutable_asset("app-dxh123.png"));
        assert!(!is_immutable_asset("app-normal.js"));
        assert!(!is_immutable_asset("dxh-no-extension"));
    }

    #[test]
    fn is_within_root_identifies_inside_and_outside_paths() {
        let dir = temp_test_dir("root-check");
        let valid_file = dir.join("index.html");
        assert!(is_within_root(&valid_file, &dir));

        let outside_dir = std::env::temp_dir();
        assert!(!is_within_root(&outside_dir, &dir));

        let non_existent = dir.join("does_not_exist.txt");
        assert!(!is_within_root(&non_existent, &dir));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn serve_blocking_handles_index_and_gzip_assets() {
        let dir = temp_test_dir("serve-blocking");
        let index_path = dir.join("index.html");

        // Empty path returns Index
        match serve_blocking(&dir, &index_path, "").unwrap() {
            ServeResult::Index { content } => assert_eq!(content, "<html>Index</html>"),
            other => panic!("expected Index, got {other:?}"),
        }

        // Non-existent file returns Index
        match serve_blocking(&dir, &index_path, "missing.html").unwrap() {
            ServeResult::Index { content } => assert_eq!(content, "<html>Index</html>"),
            other => panic!("expected Index, got {other:?}"),
        }

        // Regular file without gz
        match serve_blocking(&dir, &index_path, "style.css").unwrap() {
            ServeResult::File {
                content,
                content_type,
                cache_control,
                encoding,
            } => {
                assert_eq!(content, b"body{}");
                assert_eq!(content_type, "text/css; charset=utf-8");
                assert_eq!(cache_control, "public, max-age=3600");
                assert_eq!(encoding, None);
            }
            other => panic!("expected File, got {other:?}"),
        }

        // File with .gz available
        match serve_blocking(&dir, &index_path, "bundle.js").unwrap() {
            ServeResult::File {
                content,
                content_type,
                cache_control,
                encoding,
            } => {
                assert_eq!(content, b"fake-gzip");
                assert_eq!(content_type, "application/javascript; charset=utf-8");
                assert_eq!(cache_control, "public, max-age=3600");
                assert_eq!(encoding, Some("gzip"));
            }
            other => panic!("expected File with gzip, got {other:?}"),
        }

        let _ = fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn health_check_returns_valid_json() {
        let resp = health_check().await.into_response();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "application/json"
        );
        let bytes = resp.into_body().collect().await.unwrap().to_bytes();
        assert!(String::from_utf8_lossy(&bytes).contains("\"status\":\"ok\""));
    }

    #[tokio::test]
    async fn serve_request_serves_files_and_index() {
        let dir = temp_test_dir("serve-req");
        let state = AppState {
            static_root: dir.clone(),
            index_html: dir.join("index.html"),
        };

        // Request index
        let req_index = Request::builder().uri("/").body(Body::empty()).unwrap();
        let resp_index = serve_request(State(state.clone()), req_index)
            .await
            .unwrap();
        assert_eq!(resp_index.status(), StatusCode::OK);
        assert_eq!(
            resp_index.headers().get("content-type").unwrap(),
            "text/html; charset=utf-8"
        );

        // Request gzip file
        let req_gz = Request::builder()
            .uri("/bundle.js")
            .body(Body::empty())
            .unwrap();
        let resp_gz = serve_request(State(state.clone()), req_gz).await.unwrap();
        assert_eq!(resp_gz.status(), StatusCode::OK);
        assert_eq!(resp_gz.headers().get("content-encoding").unwrap(), "gzip");
        assert_eq!(resp_gz.headers().get("vary").unwrap(), "Accept-Encoding");

        let _ = fs::remove_dir_all(&dir);
    }
}
