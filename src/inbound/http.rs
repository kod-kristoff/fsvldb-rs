use anyhow::Context;
use axum::Router;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use tokio::net;

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

impl HttpServer {
    /// Returns a new HTTP server bound to the port specified in `config`.
    pub async fn new(
        // blog_service: impl BlogService,
        config: HttpServerConfig<'_>,
    ) -> anyhow::Result<Self> {
        // Construct dependencies to inject into handlers.
        // let state = AppState {
        //     author_service: Arc::new(blog_service),
        // };

        let router = app();
        //     .nest("/api", api_routes())
        //     .layer(trace_layer)
        //     .with_state(state);

        let listener = net::TcpListener::bind(format!("{}:{}", config.host, config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

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

fn app() -> Router {
    Router::new()
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
}
