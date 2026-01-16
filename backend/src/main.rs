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
    let contracts = obs::ingest::run(&socrata_token).await?;

    let hf_token = env::var("HF_TOKEN").unwrap_or_default();
    let hf_repo = "iberi22/veeduria-secop-ii";

    if hf_token.is_empty() {
        warn!("HF_TOKEN no configurado. Saltando sincronización con Data Lake.");
    } else {
        info!("Sincronizando {} registros con {}...", contracts.len(), hf_repo);
        let _lake = obs::hf_hub::HFDataLake::new(hf_repo, &hf_token);

        // TODO: Transform to Parquet using Polars and upload
    }

    info!("Pipeline de ingestión completado.");
    Ok(())
}
