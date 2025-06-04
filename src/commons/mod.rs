//! Converted relationships:
//! a vertices collection -> a vector store collection
//! a vertices entry -> `n` vector store entries (because a text will be splitted into smaller chunks for vectorizations. `n` stands for number of chunks.)
mod models;
mod enums;

use anyhow::Result;

use crate::{commons::{enums::Distance, models::{CollectionInformation, VectorSearchResult, OperationStatus, VectorStoreEntry, VectorizationResult}}, document::Document};

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

/// Provide basic CRUD operations for entries in a `vertices` collection. 
/// This allows `vertices` to perform higher level operations. 
pub trait VectorStoreCollectionOperations {
    async fn read_entries(&self, query: &str, document_id: Option<&str>) -> Result<Vec<VectorSearchResult>>;

    async fn delete_entry(&mut self, id: &str) -> Result<OperationStatus>;

    async fn create_entry(&mut self, entry: VectorStoreEntry) -> Result<OperationStatus>;

    async fn update_entry(&mut self, entry: VectorStoreEntry) -> Result<OperationStatus>;
}

/// An abstract layer upon the vector store collections operations
pub trait DocumentOperations {
    async fn get_document(&self, collection_name: &str, document_id: Option<&str>) -> Result<Vec<Document>>;
    
    async fn search_document_chunks(&self, collection_name: &str, document_id: Option<&str>) -> Result<Vec<VectorSearchResult>>;

    async fn delete_document(&mut self, collection_name: &str, document_id: &str) -> Result<OperationStatus>;

    async fn create_document(&mut self, collection_name: &str, document: Document) -> Result<OperationStatus>;

    async fn update_document(&mut self, collection_name: &str, document_id: &str, document: Document) -> Result<OperationStatus>;
}
