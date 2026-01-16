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

    // Initialize NLP engine
    info!("Inicializando motor de IA (Candle + BERT)...");
    let nlp_engine = obs::nlp::BertInference::new().await;

    match nlp_engine {
        Ok(engine) => {
             if let Some(first_contract) = contracts.first() {
                if let Some(obj) = &first_contract.objeto_del_contrato {
                    info!("Generando embedding para: {:.50}...", obj);
                    match engine.embed(obj) {
                        Ok(emb) => info!("Embedding generado: shape {:?}", emb.shape()),
                        Err(e) => warn!("Error al generar embedding: {}", e),
                    }
                }
             }
        },
        Err(e) => warn!("Falló la carga del modelo NLP: {}. Continuando sin IA.", e),
    }

    // Output: Generate daily_report.json
    let output_path = "../frontend/public/daily_report.json";
    info!("Guardando reporte en: {}", output_path);

    let json_data = serde_json::to_string_pretty(&contracts)?;
    tokio::fs::write(output_path, json_data).await?;

    info!("Pipeline de ingestión completado.");
    Ok(())
}
