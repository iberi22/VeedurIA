//! Vector database integration using Qdrant.
//!
//! This module provides semantic search capabilities for contracts
//! by storing and querying embedding vectors.

use anyhow::Result;
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointStruct, SearchPointsBuilder,
    UpsertPointsBuilder, VectorParamsBuilder,
};
use tracing::info;
use candle_core::Tensor;
use uuid::Uuid;

use crate::obs::ingest::ContratoSecop;

const COLLECTION_NAME: &str = "contracts";
const VECTOR_SIZE: u64 = 384; // all-MiniLM-L6-v2 dimension

/// Wrapper for Qdrant vector database operations.
pub struct VectorDB {
    client: Qdrant,
}

impl VectorDB {
    /// Create a new VectorDB connection.
    ///
    /// # Arguments
    /// * `uri` - Qdrant server URI (e.g., "http://localhost:6334")
    pub async fn new(uri: &str) -> Result<Self> {
        let client = Qdrant::from_url(uri).build()?;

        let db = Self { client };
        db.ensure_collection().await?;

        Ok(db)
    }

    /// Ensure the contracts collection exists.
    async fn ensure_collection(&self) -> Result<()> {
        if !self.client.collection_exists(COLLECTION_NAME).await? {
            info!("Creating Qdrant collection '{}'", COLLECTION_NAME);

            self.client
                .create_collection(
                    CreateCollectionBuilder::new(COLLECTION_NAME)
                        .vectors_config(VectorParamsBuilder::new(VECTOR_SIZE, Distance::Cosine)),
                )
                .await?;

            info!("Collection '{}' created successfully", COLLECTION_NAME);
        }
        Ok(())
    }

    /// Upsert a contract with its embedding into the vector database.
    pub async fn upsert_contract(&self, contract: &ContratoSecop, embedding: &Tensor) -> Result<()> {
        // Convert Candle Tensor to Vec<f32>
        let embedding_vec: Vec<f32> = embedding.flatten_all()?.to_vec1()?;

        // Use id_contrato as unique identifier, fallback to empty string
        let contract_id = contract.id_contrato.as_deref().unwrap_or("");

        // Build payload as HashMap<String, Value> for Qdrant
        let mut payload: std::collections::HashMap<String, qdrant_client::qdrant::Value> = std::collections::HashMap::new();
        payload.insert("id".to_string(), contract_id.into());
        payload.insert("objeto".to_string(), contract.objeto_del_contrato.clone().unwrap_or_default().into());
        payload.insert("entidad".to_string(), contract.nombre_entidad.clone().unwrap_or_default().into());
        payload.insert("valor".to_string(), contract.valor_del_contrato.clone().unwrap_or_default().into());
        payload.insert("fecha".to_string(), contract.fecha_de_firma.clone().unwrap_or_default().into());
        payload.insert("ciudad".to_string(), contract.ciudad.clone().unwrap_or_default().into());
        payload.insert("departamento".to_string(), contract.departamento.clone().unwrap_or_default().into());

        // Generate deterministic UUID from contract ID for idempotency
        let point_id = Uuid::new_v5(&Uuid::NAMESPACE_URL, contract_id.as_bytes());

        let point = PointStruct::new(point_id.to_string(), embedding_vec, payload);

        self.client
            .upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, vec![point]))
            .await?;

        Ok(())
    }

    /// Search for similar contracts by query embedding.
    pub async fn search(&self, query_vector: &Tensor, limit: u64) -> Result<Vec<serde_json::Value>> {
        let embedding_vec: Vec<f32> = query_vector.flatten_all()?.to_vec1()?;

        let search_result = self
            .client
            .search_points(
                SearchPointsBuilder::new(COLLECTION_NAME, embedding_vec, limit)
                    .with_payload(true),
            )
            .await?;

        let results: Vec<serde_json::Value> = search_result
            .result
            .into_iter()
            .map(|scored_point| {
                serde_json::json!({
                    "score": scored_point.score,
                    "payload": scored_point.payload
                })
            })
            .collect();

        info!("Found {} similar contracts", results.len());
        Ok(results)
    }
}
