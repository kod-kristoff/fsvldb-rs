use fsvldb_rs::{
    inbound::http::{HttpServer, HttpServerConfig},
    telemetry,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = telemetry::init_telemetry()?;
    tracing::debug!("Creating Http server");

    let server_config = HttpServerConfig {
        port: 3004,
        host: "127.0.0.1",
    };

    let http_server = HttpServer::new(server_config).await?;

    let address = http_server.local_addr()?;
    tracing::warn!("listening on {}", address);
    http_server.run().await?;

    Ok(())
}
