use anyhow::Result;

pub struct Chunk {}

/// Allow slicing a text into smaller chunks for vectorizations
pub trait Chunking {
    async fn split_into_chunks(&self, text: &str) -> Result<Vec<Chunk>>;
}
