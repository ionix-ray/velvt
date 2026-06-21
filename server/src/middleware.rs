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

/// Site uses none of these browser features — deny all of them outright.
const PERMISSIONS_POLICY_VALUE: &str = concat!(
    "accelerometer=(), camera=(), geolocation=(), gyroscope=(), ",
    "magnetometer=(), microphone=(), payment=(), usb=()"
);

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

    headers.insert(
        HeaderName::from_static("permissions-policy"),
        axum::http::HeaderValue::from_static(PERMISSIONS_POLICY_VALUE),
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
        assert!(headers.get("pragma").is_some_and(|v| v == "no-cache"));
        assert!(headers.get("expires").is_some_and(|v| v == "0"));
        assert!(
            headers
                .get("x-content-type-options")
                .is_some_and(|v| v == "nosniff")
        );
        Ok(())
    }

    #[tokio::test]
    async fn every_response_denies_unused_browser_permissions() -> TestResult {
        let app = router_with_content_type(None);
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty())?)
            .await?;

        let policy = response
            .headers()
            .get("permissions-policy")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_string();

        for feature in [
            "accelerometer",
            "camera",
            "geolocation",
            "gyroscope",
            "magnetometer",
            "microphone",
            "payment",
            "usb",
        ] {
            assert!(
                policy.contains(&format!("{feature}=()")),
                "permissions-policy missing {feature}=(): {policy}"
            );
        }
        Ok(())
    }

    thread_local! {
        // `tracing`'s per-callsite `Interest` cache is a *process-wide* cache,
        // not per-thread. Swapping subscribers via `set_default` per test races
        // other test threads doing the same, intermittently dropping events.
        // A single global subscriber (installed once) sidesteps the race; each
        // test reads only the events its own thread produced.
        static CAPTURE_BUFFER: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
    }

    #[derive(Clone, Copy, Default)]
    struct ThreadLocalWriter;

    impl std::io::Write for ThreadLocalWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            CAPTURE_BUFFER.with(|b| b.borrow_mut().write(buf))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl tracing_subscriber::fmt::MakeWriter<'_> for ThreadLocalWriter {
        type Writer = Self;
        fn make_writer(&self) -> Self::Writer {
            *self
        }
    }

    fn install_global_test_subscriber_once() {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            let subscriber = tracing_subscriber::fmt()
                .with_writer(ThreadLocalWriter)
                .with_ansi(false)
                .finish();
            // Best-effort: a prior global default (set by another test binary
            // sharing this process) is not something we need to fail over.
            let _ = tracing::subscriber::set_global_default(subscriber);
        });
    }

    #[tokio::test]
    async fn log_request_emits_structured_event_with_fields() -> TestResult {
        install_global_test_subscriber_once();
        // Worker threads are reused across tests; start from a clean buffer.
        CAPTURE_BUFFER.with(|b| b.borrow_mut().clear());

        let app = router_with_content_type(Some("text/plain"));
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty())?)
            .await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let log = CAPTURE_BUFFER.with(|b| String::from_utf8_lossy(&b.borrow()).into_owned());
        assert!(log.contains("method=GET"), "log was: {log}");
        assert!(log.contains("path=/"), "log was: {log}");
        assert!(log.contains("status=200"), "log was: {log}");
        assert!(log.contains("latency_ms="), "log was: {log}");
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
