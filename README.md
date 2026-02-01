# GRID Core

**High-performance Rust components for AI governance**

GRID Core provides foundational Rust components used by GRID-protocol compatible governance platforms like [SARK](https://github.com/apathy-ca/sark) and [YORI](https://github.com/apathy-ca/yori).

## Components

| Crate | Purpose | Performance |
|-------|---------|-------------|
| `grid-opa` | Embedded OPA policy evaluation | 4-10x faster than HTTP OPA |
| `grid-cache` | Thread-safe LRU+TTL cache | 10-50x faster than Redis |

## Quick Start

### Rust

```toml
# Cargo.toml
[dependencies]
grid-core = { git = "https://github.com/apathy-ca/grid-core" }

# Or individual crates:
grid-opa = { git = "https://github.com/apathy-ca/grid-core" }
grid-cache = { git = "https://github.com/apathy-ca/grid-core" }
```

```rust
use grid_core::{OPAEngine, LRUTTLCache};

fn main() {
    // Policy evaluation
    let mut engine = OPAEngine::new().unwrap();
    engine.load_policy("authz".to_string(), r#"
        package authz
        allow { input.user == "admin" }
    "#.to_string()).unwrap();

    // Caching
    let cache = LRUTTLCache::new(10_000, 3600);
    cache.set("user:alice".to_string(), "admin".to_string(), None).unwrap();
    assert_eq!(cache.get("user:alice"), Some("admin".to_string()));
}
```

### Python

```bash
# Build the extension
pip install maturin
maturin develop

# Or build a wheel
maturin build --release
```

```python
from grid_core import RustOPAEngine, RustCache

# Policy evaluation
engine = RustOPAEngine()
engine.load_policy("authz", """
    package authz
    allow { input.user == "admin" }
""")
result = engine.evaluate("data.authz.allow", {"user": "admin"})
print(result)  # True

# Caching
cache = RustCache(max_size=10000, ttl_secs=3600)
cache.set("user:alice", "admin")
print(cache.get("user:alice"))  # admin
```

## Features

### grid-opa

- Full Rego support via [Regorus](https://github.com/microsoft/regorus)
- Policy compilation and caching
- Thread-safe evaluation
- Comprehensive error handling
- Python bindings via PyO3

### grid-cache

- Lock-free concurrent access via [DashMap](https://docs.rs/dashmap)
- LRU eviction when at capacity
- TTL-based automatic expiration
- Periodic cleanup of expired entries
- Python bindings via PyO3

## Performance

Benchmarks comparing GRID Core to alternatives:

| Operation | GRID Core | Alternative | Speedup |
|-----------|-----------|-------------|---------|
| Policy evaluation | 0.5-2ms | 5-20ms (HTTP OPA) | 4-10x |
| Cache get | <0.01ms | 0.5-1ms (Redis) | 50-100x |
| Cache set | <0.01ms | 0.5-1ms (Redis) | 50-100x |

## Building

### Prerequisites

- Rust 1.75+
- Python 3.9+ (for Python bindings)
- maturin (for Python builds)

### Commands

```bash
# Check compilation
cargo check

# Run tests
cargo test

# Build release
cargo build --release

# Build Python wheel
maturin build --release

# Development mode (editable install)
maturin develop
```

## Cross-Compilation

GRID Core supports cross-compilation for various targets:

```bash
# Install cross
cargo install cross

# FreeBSD (for OPNsense)
cross build --release --target x86_64-unknown-freebsd

# Linux ARM64 (for Raspberry Pi 4)
cross build --release --target aarch64-unknown-linux-gnu

# Linux musl (for Alpine)
cross build --release --target x86_64-unknown-linux-musl
```

See [SARK's cross-compilation guide](https://github.com/apathy-ca/sark/blob/main/docs/CROSS_COMPILATION.md) for detailed instructions.

## Integration

### With SARK

SARK uses GRID Core for high-performance policy evaluation and caching in enterprise deployments.

```toml
# SARK's Cargo.toml
[dependencies]
grid-opa = { git = "https://github.com/apathy-ca/grid-core" }
grid-cache = { git = "https://github.com/apathy-ca/grid-core" }
```

### With YORI

YORI uses GRID Core for home network LLM governance on resource-constrained routers.

```toml
# YORI's Cargo.toml
[dependencies]
grid-opa = { git = "https://github.com/apathy-ca/grid-core" }
grid-cache = { git = "https://github.com/apathy-ca/grid-core" }
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Related Projects

- **[SARK](https://github.com/apathy-ca/sark)** - Enterprise AI governance platform
- **[YORI](https://github.com/apathy-ca/yori)** - Home network LLM gateway
- **[GRID Protocol](https://github.com/apathy-ca/sark/blob/main/docs/specifications/GRID_PROTOCOL_SPECIFICATION_v0.1.md)** - Universal governance protocol specification

## Contributing

Contributions welcome! Please see the contributing guidelines in each downstream project (SARK, YORI).
