use anyhow::Context;
use axum::{Router, extract::FromRef, routing};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use minijinja::Environment;
use tokio::net;

use crate::inbound::http::{handlers::index, templates::AppEngine};

mod error;
mod handlers;
mod templates;

/// Configuration for the HTTP server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: u16,
    pub host: &'a str,
}

/// The application's HTTP server. The underlying HTTP package is opaque to module consumers.
pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateHttpServerError {
    #[error("Failed to listen to {0}")]
    FailedToListen(String),
}

#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}

impl HttpServer {
    /// Returns a new HTTP server bound to the port specified in `config`.
    pub async fn new(
        // blog_service: impl BlogService,
        config: HttpServerConfig<'_>,
    ) -> Result<Self, CreateHttpServerError> {
        // Construct dependencies to inject into handlers.
        // let state = AppState {
        //     author_service: Arc::new(blog_service),
        // };
        let mut jinja = minijinja::Environment::new();
        jinja.set_loader(minijinja::path_loader("templates"));

        let state = AppState {
            engine: AppEngine::from(jinja),
        };

        let router = app(state);
        //     .nest("/api", api_routes())
        //     .layer(trace_layer)
        //     .with_state(state);

        let listener = net::TcpListener::bind(format!("{}:{}", config.host, config.port))
            .await
            .map_err(|_| {
                CreateHttpServerError::FailedToListen(format!("{}:{}", config.host, config.port))
            })?;

        Ok(Self { router, listener })
    }

    pub fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }

    /// Runs the HTTP server.
    pub async fn run(self) -> anyhow::Result<()> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }
}

fn app(state: AppState) -> Router {
    Router::new()
        .route("/index.html", routing::get(index::index_html))
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .with_state(state)
}
