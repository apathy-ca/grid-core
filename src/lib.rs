//! GRID Core - High-performance Rust components for AI governance
//!
//! This crate provides the foundational Rust components used by GRID-protocol
//! compatible governance platforms like SARK and YORI.
//!
//! # Components
//!
//! - **grid-opa**: Embedded OPA policy evaluation engine (4-10x faster than HTTP OPA)
//! - **grid-cache**: Thread-safe LRU+TTL cache (10-50x faster than Redis)
//!
//! # Usage
//!
//! ## Rust
//!
//! ```rust
//! use grid_core::{OPAEngine, LRUTTLCache};
//!
//! // Policy evaluation
//! let mut engine = OPAEngine::new().unwrap();
//! engine.load_policy("authz", "package authz\nallow = true".to_string()).unwrap();
//!
//! // Caching
//! let cache = LRUTTLCache::new(10_000, 3600);
//! cache.set("key".to_string(), "value".to_string(), None).unwrap();
//! ```
//!
//! ## Python
//!
//! Build with `maturin develop` and then:
//!
//! ```python
//! from grid_core import RustOPAEngine, RustCache
//!
//! engine = RustOPAEngine()
//! engine.load_policy("authz", "package authz\nallow = true")
//! result = engine.evaluate("data.authz.allow", {})
//!
//! cache = RustCache(max_size=10000, ttl_secs=3600)
//! cache.set("key", "value")
//! ```

#[cfg(feature = "python")]
use pyo3::prelude::*;

// Re-export from sub-crates
pub use grid_cache::{CacheError, LRUTTLCache};
pub use grid_opa::{OPAEngine, OPAError, Value};

#[cfg(feature = "python")]
use grid_cache::python::RustCache;
#[cfg(feature = "python")]
use grid_opa::python::RustOPAEngine;

/// Python module combining all GRID core components
#[cfg(feature = "python")]
#[pymodule]
fn grid_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // OPA components
    m.add_class::<RustOPAEngine>()?;

    // Cache components
    m.add_class::<RustCache>()?;

    // Version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
