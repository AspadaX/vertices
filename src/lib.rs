//! Vertices - A Rust library for building vector-based content management systems
//! 
//! This library provides tools and utilities for creating and managing
//! vector-based content management systems.

//! Plans
//! 
//! 1. VectorStoreConnector trait for impl different vector stores
//!     1. Qdrant impl
//!     2. In-Memory impl
//!     3. Provide methods: CRUD
//! 2. Blog trait for impl blog create, read, update and delete methods
//!     1. default impl 
//!     2. Blog handles Category, and Category handles Post
mod vector_stores;
mod text_chunking;
mod document;
mod commons;

/// A placeholder function for the vertices library
pub fn hello() -> String {
    "Hello from vertices library!".to_string()
}
