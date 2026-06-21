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
}
