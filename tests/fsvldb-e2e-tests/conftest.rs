use exn::{Exn, ResultExt};
use fsvldb_rs::inbound::http::{HttpServer, HttpServerConfig};

#[derive(Debug, thiserror::Error)]
#[error("Test failed")]
pub struct TestError;

#[derive(Debug, thiserror::Error)]
#[error("Spawning TestApp failed")]
pub struct SpawnTestAppError;

pub struct TestApp {
    pub address: String,
}

impl TestApp {
    pub async fn spawn_app() -> Result<Self, Exn<SpawnTestAppError>> {
        let http_server_config = HttpServerConfig {
            port: 0,
            host: "127.0.0.1",
        };
        let http_server = HttpServer::new(http_server_config)
            .await
            .or_raise(|| SpawnTestAppError)?;

        let address = http_server
            .local_addr()
            .or_raise(|| SpawnTestAppError)?
            .to_string();

        tokio::spawn(async move { http_server.run().await });
        Ok(Self {
            address: format!("http://{address}"),
        })
    }

    pub fn index(&self) -> String {
        format!("{}/index.html", self.address)
    }
}
