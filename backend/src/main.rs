use anyhow::Result;
use dotenvy::dotenv;
use tracing::{info, warn};
use std::env;

mod obs;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Iniciando Veeduría Ciudadana Backend v0.1.0");

    let socrata_token = env::var("SOCRATA_APP_TOKEN").unwrap_or_else(|_| {
        warn!("SOCRATA_APP_TOKEN no configurado. Usando modo anónimo (limitado).");
        "ANONYMOUS".to_string()
    });

    info!("Iniciando ingestión de datos desde SECOP II...");

    // Run data ingestion
    obs::ingest::run(&socrata_token).await?;

    info!("Pipeline de ingestión completado.");
    Ok(())
}
