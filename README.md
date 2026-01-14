# oracledb-rs

[![Crates.io](https://img.shields.io/crates/v/oracledb-rs.svg)](https://crates.io/crates/oracledb-rs)
[![Documentation](https://docs.rs/oracledb-rs/badge.svg)](https://docs.rs/oracledb-rs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

High-performance Oracle Database driver for Rust with async/await support, inspired by node-oracledb.

## Features

- **🚀 Async/Await Support**: First-class async support with Tokio runtime
- **📦 Connection Pooling**: Built-in connection pool with configurable sizing and health checks
- **🔒 Type Safety**: Strong typing for Oracle data types with compile-time guarantees
- **⚡ Performance**: Efficient statement caching, batch operations, and array fetches
- **🔌 Two Modes**:
  - **Thin Mode** (default): Pure Rust implementation, no Oracle Client required
  - **Thick Mode** (optional): Uses Oracle Client libraries for advanced features
- **🛡️ Comprehensive Error Handling**: Detailed error types with retry detection
- **🎯 Modern API**: Ergonomic, intuitive API design
- **📊 LOB Support**: CLOB, BLOB, and NCLOB handling
- **🔄 Transactions**: Full transaction support with commit/rollback
- **🎨 Result Formats**: Query results as arrays or objects

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
oracledb-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

For thick mode (optional Oracle Client library support):

```toml
[dependencies]
oracledb-rs = { version = "0.1", features = ["thick"] }
```

## Quick Start

```rust
use oracledb_rs::{Connection, ConnectionConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure connection
    let config = ConnectionConfig::new(
        "localhost:1521/XEPDB1",
        "myuser",
        "mypassword"
    );
    
    // Connect to database
    let conn = Connection::connect(config).await?;
    
    // Execute query
    let result = conn.execute(
        "SELECT employee_id, first_name, last_name FROM employees WHERE dept_id = :1",
        &[&10]
    ).await?;
    
    // Process results
    for row in result.rows() {
        let id: i64 = row.get_typed(0)?;
        let first_name: String = row.get_typed(1)?;
        let last_name: String = row.get_typed(2)?;
        
        println!("{}: {} {}", id, first_name, last_name);
    }
    
    // Close connection
    conn.close().await?;
    Ok(())
}
```

## Connection Pooling

```rust
use oracledb_rs::{Pool, PoolConfig, ConnectionConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_config = ConnectionConfig::new(
        "localhost:1521/XEPDB1",
        "myuser",
        "mypassword"
    );
    
    let pool_config = PoolConfig::new()
        .min(2)
        .max(10)
        .increment(2);
    
    let pool = Pool::new(conn_config, pool_config).await?;
    
    // Get connection from pool
    let conn = pool.get_connection().await?;
    
    // Use connection
    let result = conn.execute("SELECT * FROM users", &[]).await?;
    
    // Connection automatically returned to pool when dropped
    Ok(())
}
```

## Transactions

```rust
let mut conn = Connection::connect(config).await?;

// Execute DML
let affected = conn.execute_dml(
    "UPDATE employees SET salary = salary * 1.1 WHERE dept_id = :1",
    &[&10]
).await?;

// Commit or rollback
if affected > 0 {
    conn.commit().await?;
} else {
    conn.rollback().await?;
}
```

## Batch Operations

```rust
let batch_data = vec![
    vec![&"John" as &dyn oracledb_rs::types::ToSql, &"Doe", &50000.0],
    vec![&"Jane", &"Smith", &60000.0],
    vec![&"Bob", &"Johnson", &55000.0],
];

let sql = "INSERT INTO employees (first_name, last_name, salary) VALUES (:1, :2, :3)";
let results = conn.execute_many(sql, &batch_data).await?;

conn.commit().await?;
```

## Prepared Statements

```rust
let stmt = conn.prepare(
    "SELECT * FROM employees WHERE dept_id = :1 AND salary > :2"
).await?;

// Execute multiple times
for dept_id in [10, 20, 30] {
    let result = stmt.execute(&[&dept_id, &40000.0]).await?;
    println!("Department {}: {} employees", dept_id, result.len());
}
```

## Error Handling

```rust
use oracledb_rs::{Connection, Error};

match Connection::connect(config).await {
    Ok(conn) => {
        // Use connection
    }
    Err(e) => {
        match e {
            Error::AuthenticationFailed(msg) => {
                eprintln!("Auth failed: {}", msg);
            }
            Error::ConnectionClosed => {
                eprintln!("Connection was closed");
            }
            _ if e.is_retryable() => {
                eprintln!("Retryable error: {}", e);
                // Implement retry logic
            }
            _ => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
```

## Supported Data Types

| Oracle Type | Rust Type |
|-------------|-----------|
| VARCHAR2, NVARCHAR2 | `String` |
| CHAR, NCHAR | `String` |
| NUMBER | `i64`, `f64` |
| BINARY_FLOAT | `f32` |
| BINARY_DOUBLE | `f64` |
| DATE | `chrono::NaiveDate` |
| TIMESTAMP | `chrono::NaiveDateTime` |
| TIMESTAMP WITH TIME ZONE | `chrono::DateTime<Utc>` |
| RAW | `Vec<u8>` |
| CLOB, NCLOB | `String` |
| BLOB | `Vec<u8>` |
| JSON | `serde_json::Value` |
| BOOLEAN (PL/SQL) | `bool` |

## Configuration Options

### Connection Configuration

```rust
let config = ConnectionConfig::new(
    "localhost:1521/XEPDB1",
    "username",
    "password"
)
.mode(ConnectionMode::Thin)
.privilege(Privilege::SysDba)
.timeout(60);
```

### Pool Configuration

```rust
let pool_config = PoolConfig::new()
    .min(2)              // Minimum connections
    .max(10)             // Maximum connections
    .increment(1)        // Growth increment
    .timeout(60)         // Connection acquisition timeout
    .idle_timeout(60)    // Idle connection timeout
    .max_lifetime(3600); // Max connection lifetime
```

## Thin vs Thick Mode

### Thin Mode (Default)
- Pure Rust implementation
- No Oracle Client installation required
- Supports Oracle Database 12.1+
- Direct connection to database
- Smaller binary size

### Thick Mode (Optional)
- Uses Oracle Client libraries (11.2+)
- Supports advanced features:
  - Advanced Queuing (AQ)
  - Continuous Query Notification (CQN)
  - Database Resident Connection Pooling (DRCP)
  - Some legacy database versions (9.2+)

Enable thick mode:
```rust
let config = ConnectionConfig::new(...)
    .mode(ConnectionMode::Thick);
```

## Minimum Requirements

- **Rust**: 1.70 or later
- **Database (Thin mode)**: Oracle Database 12.1 or later
- **Database (Thick mode)**: Oracle Database 9.2 or later
- **Oracle Client (Thick mode only)**: Oracle Client 11.2 or later

## Examples

See the [examples](examples/) directory for more:

- [simple_query.rs](examples/simple_query.rs) - Basic query execution
- [connection_pool.rs](examples/connection_pool.rs) - Connection pooling
- [transactions.rs](examples/transactions.rs) - Transaction handling
- [batch_insert.rs](examples/batch_insert.rs) - Batch operations
- [prepared_statement.rs](examples/prepared_statement.rs) - Prepared statements
- [lob_handling.rs](examples/lob_handling.rs) - LOB data handling
- [error_handling.rs](examples/error_handling.rs) - Error handling

## Performance Tips

1. **Use Connection Pooling**: Reuse connections instead of creating new ones
2. **Batch Operations**: Use `execute_many()` for bulk inserts/updates
3. **Statement Caching**: Prepared statements are automatically cached
4. **Array Fetches**: Adjust `fetch_array_size` for large result sets
5. **Transaction Management**: Group related operations in transactions

## Roadmap

- [x] Basic connection and query execution
- [x] Connection pooling
- [x] Prepared statements
- [x] Batch operations
- [x] Transaction support
- [ ] REF CURSOR support
- [ ] PL/SQL stored procedures
- [ ] Advanced Queuing (AQ)
- [ ] Continuous Query Notification (CQN)
- [ ] SODA (Simple Oracle Document Access)
- [ ] Full TNS connection string parsing
- [ ] Oracle Client library integration (thick mode)
- [ ] Async streaming for large result sets

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

This library is inspired by:
- [node-oracledb](https://github.com/oracle/node-oracledb) - Oracle's official Node.js driver
- [rust-oracle](https://github.com/kubo/rust-oracle) - Existing Rust Oracle driver

## Resources

- [Documentation](https://docs.rs/oracledb-rs)
- [Oracle Database Documentation](https://docs.oracle.com/en/database/)
- [Examples](examples/)
- [Issue Tracker](https://github.com/yourusername/oracledb-rs/issues)