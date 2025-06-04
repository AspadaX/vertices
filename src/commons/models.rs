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
    pub chunk: Chunk,
}

pub struct VectorStoreDeletionResult {}

pub struct VectorStoreEntryUpdateResult {}

pub struct VectorStoreEntry {}

pub struct VectorizationResult {}