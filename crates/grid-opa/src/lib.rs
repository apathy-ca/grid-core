//! GRID OPA - High-performance embedded OPA policy evaluation engine
//!
//! This library provides a Rust-based OPA policy evaluation engine using the
//! Regorus library, with optional Python bindings via PyO3 for seamless integration.
//!
//! # Features
//!
//! - **High Performance**: 4-10x faster than HTTP-based OPA
//! - **Low Latency**: <5ms p95 latency for policy evaluation
//! - **Embedded**: No network overhead, policies run in-process
//! - **Thread-Safe**: Concurrent evaluations supported
//! - **Python Integration**: Optional PyO3 bindings
//!
//! # Example (Rust)
//!
//! ```
//! use grid_opa::engine::OPAEngine;
//! use regorus::Value;
//! use std::collections::BTreeMap;
//! use std::sync::Arc;
//!
//! let mut engine = OPAEngine::new().unwrap();
//!
//! let policy = r#"
//!     package authz
//!     allow {
//!         input.user == "admin"
//!     }
//! "#;
//!
//! engine.load_policy("authz", policy).unwrap();
//!
//! let mut input_map = BTreeMap::new();
//! input_map.insert(Value::String("user".into()), Value::String("admin".into()));
//! let input = Value::Object(Arc::new(input_map));
//! let result = engine.evaluate("data.authz.allow", input).unwrap();
//! assert_eq!(result, Value::Bool(true));
//! ```

pub mod engine;
pub mod error;

#[cfg(feature = "python")]
pub mod python;

// Re-export main types for convenience
pub use engine::OPAEngine;
pub use error::{OPAError, Result};
pub use regorus::Value;
