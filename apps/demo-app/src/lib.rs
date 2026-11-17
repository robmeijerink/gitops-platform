//! Demo service: health probes, request metrics and a `/metrics` endpoint.

use std::{sync::Arc, time::Instant};

use axum::{
    Router,
    extract::{MatchedPath, Request, State},
    http::{StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
};
use prometheus::{
    Encoder, HistogramOpts, HistogramVec, IntCounterVec, Opts, Registry, TextEncoder,
};

type Error = Box<dyn std::error::Error + Send + Sync>;

/// Request metrics, exported on `/metrics`.
pub struct Metrics {
    registry: Registry,
    requests: IntCounterVec,
    duration: HistogramVec,
}

impl Metrics {
    /// Creates the registry and registers all collectors.
    ///
    /// # Errors
    /// Returns an error if a collector cannot be created or registered.
    pub fn new() -> Result<Self, prometheus::Error> {
        let registry = Registry::new();
        let requests = IntCounterVec::new(
            Opts::new("http_requests_total", "Total HTTP requests"),
            &["method", "path", "status"],
        )?;
        let duration = HistogramVec::new(
            HistogramOpts::new("http_request_duration_seconds", "HTTP request latency"),
            &["method", "path"],
        )?;
        registry.register(Box::new(requests.clone()))?;
        registry.register(Box::new(duration.clone()))?;
        Ok(Self {
            registry,
            requests,
            duration,
        })
    }
}

/// Builds the application router.
pub fn router(metrics: Arc<Metrics>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/healthz", get(ok))
        .route("/readyz", get(ok))
        // 500 on purpose, to trigger the error-rate alert
        .route("/error", get(fail))
        .route("/metrics", get(export))
        .layer(middleware::from_fn_with_state(metrics.clone(), record))
        .with_state(metrics)
}

async fn index() -> &'static str {
    "Hello from gitops-platform\n"
}

async fn ok() -> &'static str {
    "ok\n"
}

async fn fail() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn export(State(metrics): State<Arc<Metrics>>) -> Response {
    let mut buffer = Vec::new();
    match TextEncoder::new().encode(&metrics.registry.gather(), &mut buffer) {
        Ok(()) => (
            [(header::CONTENT_TYPE, "text/plain; version=0.0.4")],
            buffer,
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn record(State(metrics): State<Arc<Metrics>>, request: Request, next: Next) -> Response {
    let method = request.method().to_string();
    // Matched route (not the raw URL) keeps label cardinality bounded.
    let path = request
        .extensions()
        .get::<MatchedPath>()
        .map_or_else(|| "unmatched".to_owned(), |p| p.as_str().to_owned());
    let start = Instant::now();

    let response = next.run(request).await;

    let status = response.status().as_u16().to_string();
    metrics
        .requests
        .with_label_values(&[&method, &path, &status])
        .inc();
    metrics
        .duration
        .with_label_values(&[&method, &path])
        .observe(start.elapsed().as_secs_f64());
    response
}

/// Runs the server until SIGTERM or Ctrl-C.
///
/// # Errors
/// Returns an error if metrics setup, binding the port or serving fails.
pub async fn run() -> Result<(), Error> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    // 0.0.0.0: the server runs in a container.
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    let app = router(Arc::new(Metrics::new()?));
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await?;
    Ok(())
}

async fn shutdown() {
    use tokio::signal::unix::{SignalKind, signal};
    let Ok(mut term) = signal(SignalKind::terminate()) else {
        return;
    };
    tokio::select! {
        _ = term.recv() => {}
        _ = tokio::signal::ctrl_c() => {}
    }
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    use super::*;

    async fn get_path(app: &Router, path: &str) -> Response {
        app.clone()
            .oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn health_probes_return_ok() {
        let app = router(Arc::new(Metrics::new().unwrap()));
        assert_eq!(get_path(&app, "/healthz").await.status(), StatusCode::OK);
        assert_eq!(get_path(&app, "/readyz").await.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn metrics_count_requests_by_status() {
        let app = router(Arc::new(Metrics::new().unwrap()));
        get_path(&app, "/").await;
        get_path(&app, "/error").await;

        let body = axum::body::to_bytes(get_path(&app, "/metrics").await.into_body(), usize::MAX)
            .await
            .unwrap();
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(text.contains(r#"http_requests_total{method="GET",path="/",status="200"} 1"#));
        assert!(text.contains(r#"status="500""#));
    }
}
