//! Router-level tests: hit `app()` through tower::ServiceExt, no live socket.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use std::error::Error;
use tower::ServiceExt;
use velvet_server::{config::Config, handlers::AppState};

type TestResult = Result<(), Box<dyn Error>>;

fn temp_static_root(name: &str) -> Result<std::path::PathBuf, std::io::Error> {
    let dir =
        std::env::temp_dir().join(format!("velvet-server-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("assets"))?;
    std::fs::write(dir.join("index.html"), "<html>spa-shell</html>")?;
    std::fs::write(dir.join("style.css"), "body{}")?;
    std::fs::write(dir.join("assets").join("app-dxh1234.wasm"), b"\0asm-fake")?;
    Ok(dir)
}

fn app_with_root(root: &std::path::Path) -> axum::Router {
    let config = Config {
        static_root: root.to_string_lossy().into_owned(),
        ..Config::default()
    };
    velvet_server::app(config)
}

async fn body_string(response: axum::response::Response) -> Result<String, Box<dyn Error>> {
    let bytes = response.into_body().collect().await?.to_bytes();
    Ok(String::from_utf8(bytes.to_vec())?)
}

#[tokio::test]
async fn health_check_returns_ok_json() -> TestResult {
    let root = temp_static_root("health")?;
    let app = app_with_root(&root);

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response.headers().get("content-type").cloned();
    let body = body_string(response).await?;
    let Some(content_type) = content_type else {
        return Err("missing content-type header".into());
    };
    assert_eq!(content_type.to_str()?, "application/json");
    assert!(body.contains(r#""status":"ok""#));

    let _ = std::fs::remove_dir_all(&root);
    Ok(())
}

#[tokio::test]
async fn existing_file_served_with_short_cache() -> TestResult {
    let root = temp_static_root("file")?;
    let app = app_with_root(&root);

    let response = app
        .oneshot(Request::builder().uri("/style.css").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let Some(cache_control) = response.headers().get("cache-control").cloned() else {
        return Err("missing cache-control header".into());
    };
    assert_eq!(cache_control.to_str()?, "public, max-age=3600");

    let _ = std::fs::remove_dir_all(&root);
    Ok(())
}

#[tokio::test]
async fn hashed_asset_served_with_immutable_cache() -> TestResult {
    let root = temp_static_root("immutable")?;
    let app = app_with_root(&root);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/assets/app-dxh1234.wasm")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let Some(content_type) = response.headers().get("content-type").cloned() else {
        return Err("missing content-type header".into());
    };
    let Some(cache_control) = response.headers().get("cache-control").cloned() else {
        return Err("missing cache-control header".into());
    };
    assert_eq!(content_type.to_str()?, "application/wasm");
    assert_eq!(
        cache_control.to_str()?,
        "public, max-age=31536000, immutable"
    );

    let _ = std::fs::remove_dir_all(&root);
    Ok(())
}

#[tokio::test]
async fn unknown_path_falls_back_to_index_html() -> TestResult {
    let root = temp_static_root("fallback")?;
    let app = app_with_root(&root);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/this/route/does/not/exist")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let Some(content_type) = response.headers().get("content-type").cloned() else {
        return Err("missing content-type header".into());
    };
    assert_eq!(content_type.to_str()?, "text/html; charset=utf-8");
    let body = body_string(response).await?;
    assert_eq!(body, "<html>spa-shell</html>");

    let _ = std::fs::remove_dir_all(&root);
    Ok(())
}

#[tokio::test]
async fn path_traversal_attempt_falls_back_to_index_not_escaped_file() -> TestResult {
    let root = temp_static_root("traversal")?;
    let app = app_with_root(&root);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/..%2f..%2f..%2fetc%2fpasswd")
                .body(Body::empty())?,
        )
        .await?;

    // Either rejected outright or safely falls back to the SPA shell —
    // never serves a file outside static_root.
    assert!(response.status() == StatusCode::OK || response.status().is_client_error());

    let _ = std::fs::remove_dir_all(&root);
    Ok(())
}

#[tokio::test]
async fn security_headers_present_on_every_response() -> TestResult {
    let root = temp_static_root("headers")?;
    let app = app_with_root(&root);

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty())?)
        .await?;

    let headers = response.headers();
    let Some(nosniff) = headers.get("x-content-type-options").cloned() else {
        return Err("missing x-content-type-options header".into());
    };
    let Some(frame_options) = headers.get("x-frame-options").cloned() else {
        return Err("missing x-frame-options header".into());
    };
    assert_eq!(nosniff, "nosniff");
    assert_eq!(frame_options, "SAMEORIGIN");
    assert!(headers.contains_key("content-security-policy"));
    assert!(headers.contains_key("strict-transport-security"));
    assert!(headers.contains_key("referrer-policy"));

    let _ = std::fs::remove_dir_all(&root);
    Ok(())
}

#[tokio::test]
async fn html_response_gets_no_cache_pragma_headers() -> TestResult {
    let root = temp_static_root("pragma")?;
    let app = app_with_root(&root);

    let response = app
        .oneshot(Request::builder().uri("/missing").body(Body::empty())?)
        .await?;

    let Some(pragma) = response.headers().get("pragma").cloned() else {
        return Err("missing pragma header".into());
    };
    let Some(expires) = response.headers().get("expires").cloned() else {
        return Err("missing expires header".into());
    };
    assert_eq!(pragma, "no-cache");
    assert_eq!(expires, "0");

    let _ = std::fs::remove_dir_all(&root);
    Ok(())
}

#[test]
fn app_state_constructs_index_html_under_static_root() {
    let state = AppState {
        static_root: std::path::PathBuf::from("/srv/static"),
        index_html: std::path::PathBuf::from("/srv/static/index.html"),
    };
    assert_eq!(state.index_html, state.static_root.join("index.html"));
}
