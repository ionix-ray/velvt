pub mod config;
pub mod error;
pub mod handlers;
pub mod middleware;

use axum::Router;
use std::net::SocketAddr;

pub fn app(config: config::Config) -> Router {
    let index_html = std::path::PathBuf::from(&config.static_root).join("index.html");
    let mut index_html_content = std::fs::read_to_string(&index_html).unwrap_or_default();

    // Dynamically inject hashed assets for preloading to avoid lazy loading pop-in
    let assets_dir = std::path::PathBuf::from(&config.static_root).join("assets");
    let mut theme_css = String::new();
    let mut brand_png = String::new();
    let mut favicon_png = String::new();

    if let Ok(entries) = std::fs::read_dir(&assets_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("theme-") && name.ends_with(".css") {
                theme_css = name;
            } else if name.starts_with("velvet-square-") && name.ends_with(".png") {
                brand_png = name;
            } else if name.starts_with("favicon-") && name.ends_with(".png") {
                favicon_png = name;
            }
        }
    }

    if !theme_css.is_empty() {
        let preload = format!(
            r#"<link rel="stylesheet" href="/assets/{}">
<link rel="preload" as="image" fetchpriority="high" href="/assets/{}">
<link rel="icon" type="image/png" href="/assets/{}">
<link rel="apple-touch-icon" href="/assets/{}">
</head>"#,
            theme_css, brand_png, favicon_png, favicon_png
        );
        index_html_content = index_html_content.replace("</head>", &preload);
    }

    let state = handlers::AppState {
        static_root: std::path::PathBuf::from(&config.static_root),
        index_html_content,
    };

    Router::new()
        .route("/health", axum::routing::get(handlers::health_check))
        .route("/api/inquiry", axum::routing::post(handlers::submit_inquiry))
        .fallback(axum::routing::get(handlers::serve_request))
        .layer(axum::middleware::from_fn(middleware::security_headers))
        .layer(axum::middleware::from_fn(middleware::log_request))
        .with_state(state)
}

pub async fn serve(router: Router, addr: &str) -> Result<(), crate::error::ServerError> {
    let socket_addr: SocketAddr = addr
        .parse()
        .map_err(|e| crate::error::ServerError::AddrParse(format!("{e}")))?;

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .map_err(crate::error::ServerError::Bind)?;

    tracing::info!("listening on http://{}", addr);

    axum::serve(listener, router)
        .await
        .map_err(crate::error::ServerError::Serve)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_builds_router_without_panicking() {
        let _router = app(config::Config::default());
    }

    #[tokio::test]
    async fn serve_rejects_unparsable_address() {
        let router = app(config::Config::default());
        let result = serve(router, "not-an-address").await;
        assert!(matches!(
            result,
            Err(crate::error::ServerError::AddrParse(_))
        ));
    }

    #[tokio::test]
    async fn serve_binds_and_starts_serving_on_an_ephemeral_port() {
        let router = app(config::Config::default());
        let handle = tokio::spawn(serve(router, "127.0.0.1:0"));

        // Bind + axum::serve entry have happened by the time the task is
        // running; abort before the (otherwise infinite) accept loop blocks.
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        handle.abort();
        let result = handle.await;
        assert!(result.is_err_and(|e| e.is_cancelled()));
    }
}
