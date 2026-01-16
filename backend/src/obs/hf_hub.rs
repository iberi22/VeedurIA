use anyhow::{Context, Result};
use hf_hub::api::tokio::ApiBuilder;
use reqwest::Client;
use std::path::PathBuf;
use tracing::info;

pub struct HFDataLake {
    repo_id: String,
    token: String,
    client: Client,
}

impl HFDataLake {
    pub fn new(repo_id: &str, token: &str) -> Self {
        Self {
            repo_id: repo_id.to_string(),
            token: token.to_string(),
            client: Client::new(),
        }
    }

    /// Download a file from the repository
    pub async fn download_file(&self, filename: &str) -> Result<PathBuf> {
        info!("Downloading {} from {}...", filename, self.repo_id);

        let api = ApiBuilder::new()
            .with_token(Some(self.token.clone()))
            .build()
            .context("Failed to build HF API")?;

        let repo = api.dataset(self.repo_id.clone());

        let path = repo.get(filename)
            .await
            .context(format!("Failed to download {} from HF Hub", filename))?;

        Ok(path)
    }

    /// Upload a file to the repository using direct HTTP API
    pub async fn upload_file(&self, local_path: PathBuf, target_name: &str) -> Result<()> {
        info!("Uploading {} to {} as {}...", local_path.display(), self.repo_id, target_name);

        let content = tokio::fs::read(&local_path)
            .await
            .context("Failed to read local file for upload")?;

        let url = format!(
            "https://huggingface.co/api/datasets/{}/upload/main/{}",
            self.repo_id, target_name
        );

        let response = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .body(content)
            .send()
            .await
            .context("Failed to send upload request to HF Hub")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("HF Hub upload error: {} - {}", status, body);
        }

        info!("Upload successful: {}", target_name);
        Ok(())
    }
}
