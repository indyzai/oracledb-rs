#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs, rust_2018_idioms)]

//! # oracledb-rs
//!
//! High-performance Oracle Database driver for Rust with async/await support.
//!
//! This library provides both thin (pure Rust) and thick (Oracle Client library)
//! connection modes, inspired by node-oracledb.
//!
//! ## Features
//!
//! - **Async/Await Support**: First-class async support with tokio
//! - **Connection Pooling**: Built-in connection pool with configurable sizing
//! - **Prepared Statements**: Efficient statement caching and reuse
//! - **Type Safety**: Strong typing for Oracle data types
//! - **Transactions**: Full transaction support with savepoints
//! - **LOBs**: CLOB, BLOB, and NCLOB support
//! - **PL/SQL**: Execute stored procedures and functions
//! - **Batching**: Batch DML operations for performance
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use oracledb_rs::{Connection, ConnectionConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ConnectionConfig::new(
//!         "localhost:1521/XEPDB1",
//!         "myuser",
//!         "mypassword"
//!     );
//!     
//!     let conn = Connection::connect(config).await?;
//!     
//!     let rows = conn.execute("SELECT * FROM employees WHERE dept_id = :1", &[&10]).await?;
//!     
//!     for row in rows {
//!         println!("{:?}", row);
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod auth;
pub mod connection;
pub mod error;
pub mod pool;
pub mod protocol;
pub mod result;
pub mod statement;
pub mod types;

pub use connection::{Connection, ConnectionConfig, ConnectionMode};
pub use error::{Error, Result};
pub use pool::{Pool, PoolConfig};
pub use statement::{ResultSet, Row, Statement};
pub use types::{OracleType, Value};

/// Oracle database connection modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Thin mode - pure Rust implementation (default)
    Thin,
    /// Thick mode - uses Oracle Client libraries
    Thick,
}

/// Constants for Oracle Database versions
pub mod constants {
    /// Minimum supported Oracle Database version (12.1) for thin mode
    pub const MIN_DB_VERSION_THIN: (u8, u8) = (12, 1);

    /// Minimum supported Oracle Database version (9.2) for thick mode
    pub const MIN_DB_VERSION_THICK: (u8, u8) = (9, 2);

    /// Default port for Oracle TNS listener
    pub const DEFAULT_PORT: u16 = 1521;

    /// Default fetch array size
    pub const DEFAULT_FETCH_ARRAY_SIZE: usize = 100;

    /// Default statement cache size
    pub const DEFAULT_STMT_CACHE_SIZE: usize = 30;
}

/// Privilege modes for connections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Privilege {
    /// Normal connection
    Normal,
    /// SYSDBA privilege
    SysDba,
    /// SYSOPER privilege
    SysOper,
    /// SYSASM privilege (for ASM)
    SysAsm,
    /// SYSBACKUP privilege
    SysBackup,
    /// SYSDG privilege
    SysDg,
    /// SYSKM privilege
    SysKm,
}

/// Execute options for statements
#[derive(Debug, Clone)]
pub struct ExecuteOptions {
    /// Auto-commit after execution
    pub auto_commit: bool,
    /// Array size for fetch operations
    pub fetch_array_size: usize,
    /// Maximum number of rows to fetch (0 = unlimited)
    pub max_rows: usize,
    /// Result set format
    pub out_format: OutFormat,
}

impl Default for ExecuteOptions {
    fn default() -> Self {
        Self {
            auto_commit: false,
            fetch_array_size: constants::DEFAULT_FETCH_ARRAY_SIZE,
            max_rows: 0,
            out_format: OutFormat::Object,
        }
    }
}

/// Output format for query results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutFormat {
    /// Results as objects (hashmaps)
    Object,
    /// Results as arrays
    Array,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(constants::DEFAULT_PORT, 1521);
        assert_eq!(constants::MIN_DB_VERSION_THIN, (12, 1));
    }
}
