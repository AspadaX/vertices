use std::collections::HashMap;

use crate::text_chunking::Chunk;

pub struct Document {
    pub document_id: String,
    pub document_title: String,
    pub chunks: Vec<Chunk>,
    pub metadata: HashMap<String, String>,
}