//! Tower middleware — security headers, logging, and CORS.

use axum::{
    extract::Request,
    http::header::{
        HeaderName, CONTENT_SECURITY_POLICY, EXPIRES, PRAGMA,
        REFERRER_POLICY, STRICT_TRANSPORT_SECURITY, X_CONTENT_TYPE_OPTIONS,
        X_FRAME_OPTIONS,
    },
    middleware::Next,
    response::Response,
};

/// Security headers applied to every outgoing response.
const SECURITY_HEADERS: &[(HeaderName, &str)] = &[
    (X_CONTENT_TYPE_OPTIONS, "nosniff"),
    (X_FRAME_OPTIONS, "SAMEORIGIN"),
    (STRICT_TRANSPORT_SECURITY, "max-age=15552000; includeSubDomains"),
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
