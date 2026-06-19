pub mod config;
pub mod error;
pub mod handlers;
pub mod middleware;

use axum::Router;
use std::net::SocketAddr;

pub fn app(config: config::Config) -> Router {
    let state = handlers::AppState {
        static_root: std::path::PathBuf::from(&config.static_root),
        index_html: std::path::PathBuf::from(&config.static_root).join("index.html"),
    };

    Router::new()
        .route("/health", axum::routing::get(handlers::health_check))
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
