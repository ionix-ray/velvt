//! Tower middleware — security headers, logging, and CORS.

use axum::body::Body;
use axum::http::StatusCode;
use axum::{
    extract::Request,
    http::header::{
        CONTENT_SECURITY_POLICY, EXPIRES, HeaderName, PRAGMA, REFERRER_POLICY,
        STRICT_TRANSPORT_SECURITY, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS,
    },
    middleware::Next,
    response::Response,
};
use std::time::Duration;

const PERMISSIONS_POLICY_VALUE: &str = concat!(
    "accelerometer=(), camera=(), geolocation=(), gyroscope=(), ",
    "magnetometer=(), microphone=(), payment=(), usb=()"
);

const SECURITY_HEADERS: &[(HeaderName, &str)] = &[
    (X_CONTENT_TYPE_OPTIONS, "nosniff"),
    (X_FRAME_OPTIONS, "SAMEORIGIN"),
    (
        STRICT_TRANSPORT_SECURITY,
        "max-age=15552000; includeSubDomains",
    ),
    (REFERRER_POLICY, "strict-origin-when-cross-origin"),
];

const CSP_VALUE: &str = concat!(
    "default-src 'self'; ",
    "script-src 'self' 'wasm-unsafe-eval'; ",
    "style-src 'self' 'unsafe-inline'; ",
    "img-src 'self' data:; ",
    "font-src 'self'; ",
    "connect-src 'self'; ",
    "frame-ancestors 'none'; ",
    "form-action 'self'; ",
    "base-uri 'self'; ",
    "object-src 'none'"
);

pub async fn security_headers(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    for (name, value) in SECURITY_HEADERS {
        headers.insert(name.clone(), axum::http::HeaderValue::from_static(value));
    }

    headers.insert(
        CONTENT_SECURITY_POLICY,
        axum::http::HeaderValue::from_static(CSP_VALUE),
    );

    headers.insert(
        HeaderName::from_static("permissions-policy"),
        axum::http::HeaderValue::from_static(PERMISSIONS_POLICY_VALUE),
    );

    headers.insert(
        HeaderName::from_static("cross-origin-opener-policy"),
        axum::http::HeaderValue::from_static("same-origin"),
    );

    headers.insert(
        HeaderName::from_static("x-dns-prefetch-control"),
        axum::http::HeaderValue::from_static("off"),
    );

    if let Some(content_type) = headers.get("content-type") {
        let ct = content_type.to_str().unwrap_or("");
        if ct.starts_with("text/html") {
            headers.insert(PRAGMA, axum::http::HeaderValue::from_static("no-cache"));
            headers.insert(EXPIRES, axum::http::HeaderValue::from_static("0"));
        }
    }

    response
}

pub async fn log_request(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();

    let response = next.run(request).await;

    let latency = start.elapsed();
    let status = response.status();

    tracing::info!(
        method = %method,
        path = %uri.path(),
        status = status.as_u16(),
        latency_ms = latency.as_millis(),
        "request"
    );

    response
}

pub async fn handle_timeout(request: Request, next: Next) -> Response {
    next.run(request).await
}

#[derive(Clone)]
pub struct TimeoutLayerConfig {
    pub timeout: Duration,
}

pub fn timeout_middleware(
    timeout: Duration,
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
+ Clone {
    move |req: Request, next: Next| {
        let timeout = timeout;
        Box::pin(async move {
            match tokio::time::timeout(timeout, next.run(req)).await {
                Ok(response) => response,
                Err(_) => {
                    let mut response = Response::new(Body::from("Request Timeout"));
                    *response.status_mut() = StatusCode::REQUEST_TIMEOUT;
                    response.headers_mut().insert(
                        "content-type",
                        axum::http::HeaderValue::from_static("text/plain"),
                    );
                    response
                }
            }
        })
    }
}
