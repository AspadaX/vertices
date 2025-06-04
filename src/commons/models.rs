use std::collections::HashMap;

use crate::{commons::enums::SupportedVectorStore, text_chunking::Chunk};

/// Contains a collection's information
pub struct CollectionInformation {
    pub name: String,
    pub size: usize,
    pub entry_amount: usize,
    pub vector_store: SupportedVectorStore,
}

/// Represent a vector search result
pub struct VectorSearchResult {
    pub similarity_score: f32,
    pub vectory_store_entries: VectorStoreEntry,
}

pub struct OperationStatus {
    pub target_id: String,
    pub has_succeeded: bool,
}

pub struct VectorStoreEntry {
    pub document_id: String,
    pub document_title: String,
    pub metadata: HashMap<String, String>,
    pub chunk: Chunk,
}

pub struct VectorizationResult {
    pub vector: Vec<f32>
}