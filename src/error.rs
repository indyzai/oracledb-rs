// Error types

use thiserror::Error;

/// Result type for Oracle operations
pub type Result<T> = std::result::Result<T, Error>;

/// Oracle database errors
#[derive(Error, Debug)]
pub enum Error {
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Connection is closed
    #[error("Connection is closed")]
    ConnectionClosed,

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// SQL execution error
    #[error("SQL execution error: {0}")]
    SqlExecution(String),

    /// Invalid SQL statement
    #[error("Invalid SQL: {0}")]
    InvalidSql(String),

    /// Type mismatch error
    #[error("Type mismatch: {0}")]
    TypeMismatch(String),

    /// Column not found
    #[error("Column not found: {0}")]
    ColumnNotFound(String),

    /// Invalid bind parameter
    #[error("Invalid bind parameter: {0}")]
    InvalidBindParameter(String),

    /// Pool error
    #[error("Connection pool error: {0}")]
    Pool(String),

    /// Pool timeout
    #[error("Connection pool timeout - no connections available")]
    PoolTimeout,

    /// Pool is closed
    #[error("Connection pool is closed")]
    PoolClosed,

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Unsupported feature
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Network I/O error
    #[error("Network I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Encoding/decoding error
    #[error("Encoding error: {0}")]
    Encoding(String),

    /// Timeout error
    #[error("Operation timeout")]
    Timeout,

    /// Oracle-specific error with code
    #[error("Oracle error ORA-{code:05}: {message}")]
    Oracle {
        /// Oracle error code
        code: i32,
        /// Error message
        message: String,
    },

    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// LOB operation error
    #[error("LOB operation error: {0}")]
    Lob(String),

    /// Invalid data
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// Feature not implemented yet
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Other errors
    #[error("Error: {0}")]
    Other(String),
}

impl Error {
    /// Create an Oracle error with code and message
    pub fn oracle(code: i32, message: impl Into<String>) -> Self {
        Self::Oracle {
            code,
            message: message.into(),
        }
    }

    /// Check if error is a connection error
    pub fn is_connection_error(&self) -> bool {
        matches!(
            self,
            Error::Connection(_) | Error::ConnectionClosed | Error::Io(_)
        )
    }

    /// Check if error is a pool error
    pub fn is_pool_error(&self) -> bool {
        matches!(
            self,
            Error::Pool(_) | Error::PoolTimeout | Error::PoolClosed
        )
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Timeout => true,
            Error::PoolTimeout => true,
            Error::Io(_) => true,
            Error::Oracle { code, .. } => {
                // Some Oracle errors are retryable
                matches!(
                    code,
                    // Connection errors
                    17002 | 17008 | 17410 |
                    // Session errors
                    1012 | 1013 | 1089 |
                    // Resource busy
                    54
                )
            }
            _ => false,
        }
    }

    /// Get error code if this is an Oracle error
    pub fn oracle_code(&self) -> Option<i32> {
        match self {
            Error::Oracle { code, .. } => Some(*code),
            _ => None,
        }
    }
}

/// Common Oracle error codes
pub mod codes {
    /// Unique constraint violated
    pub const UNIQUE_CONSTRAINT: i32 = 1;

    /// Invalid username/password
    pub const INVALID_USERNAME_PASSWORD: i32 = 1017;

    /// Not logged on
    pub const NOT_LOGGED_ON: i32 = 1012;

    /// No data found
    pub const NO_DATA_FOUND: i32 = 1403;

    /// Too many rows
    pub const TOO_MANY_ROWS: i32 = 1422;

    /// Deadlock detected
    pub const DEADLOCK: i32 = 60;

    /// Resource busy
    pub const RESOURCE_BUSY: i32 = 54;

    /// Timeout occurred
    pub const TIMEOUT: i32 = 1013;

    /// Connection timeout
    pub const CONNECTION_TIMEOUT: i32 = 17002;

    /// End of file on communication channel
    pub const EOF_COMMUNICATION: i32 = 3113;

    /// TNS: could not resolve service name
    pub const TNS_NO_SERVICE: i32 = 12154;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = Error::oracle(1017, "invalid username/password");
        assert_eq!(err.oracle_code(), Some(1017));
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_retryable_errors() {
        assert!(Error::Timeout.is_retryable());
        assert!(Error::PoolTimeout.is_retryable());
        assert!(Error::oracle(54, "resource busy").is_retryable());
        assert!(!Error::oracle(1, "unique constraint").is_retryable());
    }

    #[test]
    fn test_error_display() {
        let err = Error::oracle(1017, "invalid username/password");
        let msg = format!("{}", err);
        assert!(msg.contains("ORA-01017"));
    }
}
