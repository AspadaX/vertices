pub mod qdrant;

pub struct VectorSearchResult {}

pub struct VectorStoreDeletionResult {}

pub struct VectorStoreEntryUpdateResult {}

pub struct VectorStoreEntry {}

pub trait VectorStoreCRUD {
    fn read(&self, query: &str) -> VectorSearchResult;
    
    fn delete(&mut self, id: &str) -> VectorStoreDeletionResult;
    
    fn create(&mut self, entry: VectorStoreEntry) -> VectorStoreEntryUpdateResult;
    
    fn update(&mut self, entry: VectorStoreEntry) -> VectorStoreEntryUpdateResult;
}
