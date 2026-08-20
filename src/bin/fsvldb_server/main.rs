use fsvldb_rs::telemetry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = telemetry::init_telemetry()?;
    tracing::info!("Creating Http server");
    Ok(())
}
