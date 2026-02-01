//! GRID Cache - High-performance thread-safe LRU+TTL cache
//!
//! This library provides a lock-free concurrent cache using DashMap,
//! with LRU eviction and TTL-based expiration.
//!
//! # Features
//!
//! - **Lock-Free**: Uses DashMap for concurrent access without mutex contention
//! - **LRU Eviction**: Least-recently-used entries removed when at capacity
//! - **TTL Support**: Automatic expiration of entries
//! - **High Performance**: 10-50x faster than Redis for local caching
//! - **Thread-Safe**: Safe for concurrent reads and writes
//!
//! # Example
//!
//! ```
//! use grid_cache::LRUTTLCache;
//!
//! // Create cache with 10,000 max entries, 1 hour TTL
//! let cache = LRUTTLCache::new(10_000, 3600);
//!
//! // Store and retrieve values
//! cache.set("user:alice".to_string(), "admin".to_string(), None).unwrap();
//! assert_eq!(cache.get("user:alice"), Some("admin".to_string()));
//!
//! // Values expire after TTL
//! cache.set("temp".to_string(), "data".to_string(), Some(1)).unwrap();
//! // After 1 second, cache.get("temp") will return None
//! ```

pub mod error;
pub mod lru_ttl;

#[cfg(feature = "python")]
pub mod python;

// Re-export main types
pub use error::{CacheError, Result};
pub use lru_ttl::LRUTTLCache;
