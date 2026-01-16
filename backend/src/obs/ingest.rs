use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

const SECOP_CONTRATOS_ID: &str = "jbjy-vk9h";
const SECOP_PROCESOS_ID: &str = "p6dx-8zbt";
const SOCRATA_BASE_URL: &str = "https://www.datos.gov.co/resource";

#[derive(Debug, Serialize, Deserialize)]
pub struct ContratoSecop {
    #[serde(default)]
    pub id_contrato: Option<String>,
    #[serde(default)]
    pub nombre_entidad: Option<String>,
    #[serde(default)]
    pub nit_entidad: Option<String>,
    #[serde(default)]
    pub departamento: Option<String>,
    #[serde(default)]
    pub ciudad: Option<String>,
    #[serde(default)]
    pub objeto_del_contrato: Option<String>,
    #[serde(default)]
    pub tipo_de_contrato: Option<String>,
    #[serde(default)]
    pub modalidad_de_contratacion: Option<String>,
    #[serde(default)]
    pub valor_del_contrato: Option<String>,
    #[serde(default)]
    pub nombre_contratista: Option<String>,
    #[serde(default)]
    pub nit_contratista: Option<String>,
    #[serde(default)]
    pub fecha_de_firma: Option<String>,
    #[serde(default)]
    pub fecha_de_inicio_del_contrato: Option<String>,
    #[serde(default)]
    pub duracion: Option<String>,
}

pub struct SocrataClient {
    client: Client,
    app_token: Option<String>,
}

impl SocrataClient {
    pub fn new(app_token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            app_token,
        }
    }

    /// Fetch contracts from SECOP II with pagination
    pub async fn fetch_contratos(
        &self,
        limit: u32,
        offset: u32,
        since_date: Option<&str>,
    ) -> Result<Vec<ContratoSecop>> {
        let mut url = format!(
            "{}/{}.json?$limit={}&$offset={}&$order=fecha_de_firma DESC",
            SOCRATA_BASE_URL, SECOP_CONTRATOS_ID, limit, offset
        );

        if let Some(date) = since_date {
            url.push_str(&format!("&$where=fecha_de_firma > '{}'", date));
        }

        info!("Fetching SECOP II contracts: limit={}, offset={}", limit, offset);

        let mut request = self.client.get(&url);

        if let Some(token) = &self.app_token {
            request = request.header("X-App-Token", token);
        } else {
            warn!("No App Token provided. Rate limits may apply.");
        }

        let response = request
            .send()
            .await
            .context("Failed to send request to Socrata API")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Socrata API error: {} - {}", status, body);
        }

        let contratos: Vec<ContratoSecop> = response
            .json()
            .await
            .context("Failed to parse Socrata response")?;

        info!("Fetched {} contracts", contratos.len());
        Ok(contratos)
    }

    /// Fetch all contracts with automatic pagination
    pub async fn fetch_all_contratos(
        &self,
        since_date: Option<&str>,
        max_records: Option<u32>,
    ) -> Result<Vec<ContratoSecop>> {
        let mut all_contratos = Vec::new();
        let page_size = 1000;
        let mut offset = 0;

        loop {
            let batch = self.fetch_contratos(page_size, offset, since_date).await?;
            let batch_len = batch.len();
            all_contratos.extend(batch);

            if batch_len < page_size as usize {
                break; // No more records
            }

            offset += page_size;

            if let Some(max) = max_records {
                if all_contratos.len() >= max as usize {
                    all_contratos.truncate(max as usize);
                    break;
                }
            }

            // Respect rate limits - small delay between pages
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        info!("Total contracts fetched: {}", all_contratos.len());
        Ok(all_contratos)
    }
}

/// Main entry point for data ingestion
pub async fn run(token: &str) -> Result<()> {
    let app_token = if token == "ANONYMOUS" {
        None
    } else {
        Some(token.to_string())
    };

    let client = SocrataClient::new(app_token);

    // Fetch last 100 contracts for testing
    let contratos = client.fetch_contratos(100, 0, None).await?;

    info!("Sample contract: {:?}", contratos.first());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_contratos() {
        let client = SocrataClient::new(None);
        let result = client.fetch_contratos(10, 0, None).await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}
