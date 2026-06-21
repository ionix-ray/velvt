//! Tower middleware — security headers, logging, and CORS.

use axum::{
    extract::Request,
    http::header::{
        CONTENT_SECURITY_POLICY, EXPIRES, HeaderName, PRAGMA, REFERRER_POLICY,
        STRICT_TRANSPORT_SECURITY, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS,
    },
    middleware::Next,
    response::Response,
};

/// Security headers applied to every outgoing response.
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use std::error::Error;
    use tower::ServiceExt;

    type TestResult = Result<(), Box<dyn Error>>;

    fn router_with_content_type(content_type: Option<&'static str>) -> axum::Router {
        axum::Router::new()
            .route(
                "/",
                axum::routing::get(move || async move {
                    let mut response = Response::new(Body::from("ok"));
                    if let Some(ct) = content_type {
                        response
                            .headers_mut()
                            .insert("content-type", axum::http::HeaderValue::from_static(ct));
                    }
                    response
                }),
            )
            .layer(axum::middleware::from_fn(security_headers))
            .layer(axum::middleware::from_fn(log_request))
    }

    #[tokio::test]
    async fn html_responses_get_pragma_no_cache() -> TestResult {
        let app = router_with_content_type(Some("text/html; charset=utf-8"));
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty())?)
            .await?;

        let headers = response.headers();
        let Some(pragma) = headers.get("pragma").cloned() else {
            return Err("missing pragma header".into());
        };
        let Some(expires) = headers.get("expires").cloned() else {
            return Err("missing expires header".into());
        };
        let Some(nosniff) = headers.get("x-content-type-options").cloned() else {
            return Err("missing x-content-type-options header".into());
        };
        assert_eq!(pragma, "no-cache");
        assert_eq!(expires, "0");
        assert_eq!(nosniff, "nosniff");
        Ok(())
    }

    #[tokio::test]
    async fn non_html_responses_have_no_pragma() -> TestResult {
        let app = router_with_content_type(Some("application/json"));
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty())?)
            .await?;

        assert!(response.headers().get("pragma").is_none());
        Ok(())
    }

    #[tokio::test]
    async fn missing_content_type_is_handled_without_panicking() -> TestResult {
        let app = router_with_content_type(None);
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty())?)
            .await?;

        assert!(response.headers().get("pragma").is_none());
        assert!(response.headers().contains_key("content-security-policy"));
        Ok(())
    }
}
