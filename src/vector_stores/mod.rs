//! Converted relationships:
//! a vertices collection -> a vector store collection
//! a vertices entry -> `n` vector store entries (because a text will be splitted into smaller chunks for vectorizations. `n` stands for number of chunks.)

pub mod qdrant;

use anyhow::Result;

pub struct CollectionInformation {}

pub struct VectorSearchResult {}

pub struct VectorStoreDeletionResult {}

pub struct VectorStoreEntryUpdateResult {}

pub struct VectorStoreEntry {}

pub struct VectorizationResult {}

pub struct Chunk {}

pub enum Distance {
    Cosine,
    DotProduct,
}

/// Allow slicing a text into smaller chunks for vectorizations
pub trait Chunking {
    async fn split_into_chunks(&self, text: &str) -> Result<Vec<Chunk>>;
}

/// Provide vectorization implementations
pub trait Vectorizations {
    async fn vectorize(&self, text: &str) -> Result<VectorizationResult>;
}

/// Provide implementations to manage vertices collections
pub trait CollectionsManagements {
    async fn create_new_collection(&mut self, name: &str, size: usize, distance_metrics: Distance) -> Result<()>;
    
    async fn delete_collection(&mut self, name: &str) -> Result<()>;
    
    async fn read_collection_information(&self, name: &str) -> Result<CollectionInformation>;
}

/// Provide basic CRUD operations for entries in a vertices collection
pub trait VectorStoreCRUDOperations {
    async fn read_entries(&self, query: &str) -> Result<Vec<VectorSearchResult>>;

    async fn delete_entry(&mut self, id: &str) -> Result<VectorStoreDeletionResult>;

    async fn create_entry(&mut self, entry: VectorStoreEntry) -> Result<VectorStoreEntryUpdateResult>;

    async fn update_entry(&mut self, entry: VectorStoreEntry) -> Result<VectorStoreEntryUpdateResult>;
}
