pub mod config;
pub mod error;
pub mod handlers;
pub mod middleware;

#[cfg(test)]
pub mod test_support;

use axum::Router;
use std::future::Future;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::signal;

pub fn app(config: config::Config) -> Router {
    let state = handlers::AppState {
        static_root: std::path::PathBuf::from(&config.static_root),
        index_html: std::path::PathBuf::from(&config.static_root).join("index.html"),
    };

    let _timeout_duration = Duration::from_secs(config.request_timeout_secs);
    let max_concurrent = config.max_concurrent_requests;

    Router::new()
        .route("/health", axum::routing::get(handlers::health_check))
        .fallback(axum::routing::get(handlers::serve_request))
        .layer(axum::middleware::from_fn(middleware::security_headers))
        .layer(axum::middleware::from_fn(middleware::log_request))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(
            config.max_body_bytes,
        ))
        .layer(tower::limit::ConcurrencyLimitLayer::new(max_concurrent))
        .layer(axum::middleware::from_fn(move |req, next| {
            let timeout = _timeout_duration;
            middleware::timeout_middleware(timeout)(req, next)
        }))
        .with_state(state)
}

async fn shutdown_signal() {
    shutdown_signal_with(ctrl_c(), terminate_signal()).await
}

async fn ctrl_c() {
    let _ = signal::ctrl_c().await;
}

#[cfg(unix)]
async fn terminate_signal() {
    if let Ok(mut sig) = signal::unix::signal(signal::unix::SignalKind::terminate()) {
        let _ = sig.recv().await;
    }
}

#[cfg(not(unix))]
async fn terminate_signal() {
    std::future::pending::<()>().await;
}

async fn shutdown_signal_with<F1, F2>(ctrl_c: F1, terminate: F2)
where
    F1: Future<Output = ()>,
    F2: Future<Output = ()>,
{
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received");
}

pub async fn serve(router: Router, addr: &str) -> Result<(), crate::error::ServerError> {
    serve_with_shutdown(router, addr, shutdown_signal()).await
}

pub async fn serve_with_shutdown<F>(
    router: Router,
    addr: &str,
    shutdown: F,
) -> Result<(), crate::error::ServerError>
where
    F: Future<Output = ()> + Send + 'static,
{
    let socket_addr: SocketAddr = addr
        .parse()
        .map_err(|e| crate::error::ServerError::AddrParse(format!("{e}")))?;

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .map_err(crate::error::ServerError::Bind)?;

    tracing::info!("listening on http://{}", addr);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown)
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
