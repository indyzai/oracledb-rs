// Connection management

use crate::auth::Authenticator;
use crate::protocol::Protocol;
use crate::statement::{ResultSet, Statement};
use crate::{Error, Privilege, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Connection configuration
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// Connection string (host:port/service_name or EZ Connect)
    pub connection_string: String,
    /// Username
    pub user: String,
    /// Password
    pub password: String,
    /// Connection mode (Thin or Thick)
    pub mode: ConnectionMode,
    /// Privilege level
    pub privilege: Privilege,
    /// Connection timeout in seconds
    pub connect_timeout: u32,
    /// Statement cache size
    pub stmt_cache_size: usize,
    /// Enable connection health checks
    pub enable_ping: bool,
}

impl ConnectionConfig {
    /// Create a new connection configuration
    pub fn new(
        connection_string: impl Into<String>,
        user: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            connection_string: connection_string.into(),
            user: user.into(),
            password: password.into(),
            mode: ConnectionMode::Thin,
            privilege: Privilege::Normal,
            connect_timeout: 60,
            stmt_cache_size: crate::constants::DEFAULT_STMT_CACHE_SIZE,
            enable_ping: true,
        }
    }

    /// Set connection mode
    pub fn mode(mut self, mode: ConnectionMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set privilege level
    pub fn privilege(mut self, privilege: Privilege) -> Self {
        self.privilege = privilege;
        self
    }

    /// Set connection timeout
    pub fn timeout(mut self, seconds: u32) -> Self {
        self.connect_timeout = seconds;
        self
    }
}

/// Connection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionMode {
    /// Thin mode - pure Rust implementation
    Thin,
    /// Thick mode - uses Oracle Client libraries
    Thick,
}

/// Oracle Database connection
pub struct Connection {
    config: ConnectionConfig,
    protocol: Arc<Mutex<Protocol>>,
    is_open: bool,
    transaction_active: bool,
}

impl Connection {
    /// Establish a connection to Oracle Database
    pub async fn connect(config: ConnectionConfig) -> Result<Self> {
        match config.mode {
            ConnectionMode::Thin => Self::connect_thin(config).await,
            ConnectionMode::Thick => Self::connect_thick(config).await,
        }
    }

    /// Connect using thin mode (pure Rust)
    async fn connect_thin(config: ConnectionConfig) -> Result<Self> {
        let protocol = Protocol::new(&config).await?;

        let mut conn = Self {
            config,
            protocol: Arc::new(Mutex::new(protocol)),
            is_open: true,
            transaction_active: false,
        };

        conn.authenticate().await?;

        Ok(conn)
    }

    /// Connect using thick mode (Oracle Client libraries)
    #[cfg(feature = "thick")]
    async fn connect_thick(_config: ConnectionConfig) -> Result<Self> {
        // Implementation would use FFI to Oracle Client libraries
        Err(Error::UnsupportedFeature(
            "Thick mode not yet implemented".into(),
        ))
    }

    #[cfg(not(feature = "thick"))]
    async fn connect_thick(_config: ConnectionConfig) -> Result<Self> {
        Err(Error::UnsupportedFeature(
            "Thick mode requires 'thick' feature".into(),
        ))
    }

    /// Authenticate with the database
    async fn authenticate(&mut self) -> Result<()> {
        let mut protocol = self.protocol.lock().await;
        let auth = Authenticator::new(&self.config);
        auth.authenticate(&mut protocol).await
    }

    /// Execute a SQL statement
    pub async fn execute(
        &self,
        sql: &str,
        params: &[&dyn crate::types::ToSql],
    ) -> Result<ResultSet> {
        self.check_open()?;

        let stmt = Statement::new(sql, self.protocol.clone());
        stmt.execute(params).await
    }

    /// Execute a query and return results
    pub async fn query(&self, sql: &str, params: &[&dyn crate::types::ToSql]) -> Result<ResultSet> {
        self.execute(sql, params).await
    }

    /// Execute a DML statement (INSERT, UPDATE, DELETE)
    pub async fn execute_dml(&self, sql: &str, params: &[&dyn crate::types::ToSql]) -> Result<u64> {
        self.check_open()?;

        let stmt = Statement::new(sql, self.protocol.clone());
        stmt.execute_dml(params).await
    }

    /// Execute many statements with batch binding
    pub async fn execute_many(
        &self,
        sql: &str,
        batch_params: &[Vec<&dyn crate::types::ToSql>],
    ) -> Result<Vec<u64>> {
        self.check_open()?;

        let stmt = Statement::new(sql, self.protocol.clone());
        stmt.execute_many(batch_params).await
    }

    /// Prepare a statement for later execution
    pub async fn prepare(&self, sql: &str) -> Result<Statement> {
        self.check_open()?;
        Ok(Statement::new(sql, self.protocol.clone()))
    }

    /// Commit the current transaction
    pub async fn commit(&mut self) -> Result<()> {
        self.check_open()?;

        let mut protocol = self.protocol.lock().await;
        protocol.commit().await?;
        self.transaction_active = false;
        Ok(())
    }

    /// Rollback the current transaction
    pub async fn rollback(&mut self) -> Result<()> {
        self.check_open()?;

        let mut protocol = self.protocol.lock().await;
        protocol.rollback().await?;
        self.transaction_active = false;
        Ok(())
    }

    /// Ping the database to check connection health
    pub async fn ping(&self) -> Result<()> {
        self.check_open()?;

        let mut protocol = self.protocol.lock().await;
        protocol.ping().await
    }

    /// Close the connection
    pub async fn close(mut self) -> Result<()> {
        if !self.is_open {
            return Ok(());
        }

        let mut protocol = self.protocol.lock().await;
        protocol.close().await?;
        self.is_open = false;
        Ok(())
    }

    /// Check if connection is open
    fn check_open(&self) -> Result<()> {
        if !self.is_open {
            return Err(Error::ConnectionClosed);
        }
        Ok(())
    }

    /// Get connection information
    pub fn info(&self) -> ConnectionInfo {
        ConnectionInfo {
            mode: self.config.mode,
            user: self.config.user.clone(),
            connection_string: self.config.connection_string.clone(),
            is_open: self.is_open,
            transaction_active: self.transaction_active,
        }
    }
}

/// Connection information
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    /// Connection mode (Thin or Thick)
    pub mode: ConnectionMode,
    /// Username
    pub user: String,
    /// Connection string
    pub connection_string: String,
    /// Whether the connection is currently open
    pub is_open: bool,
    /// Whether a transaction is currently active
    pub transaction_active: bool,
}

impl Drop for Connection {
    fn drop(&mut self) {
        if self.is_open {
            // In a real implementation, we'd properly close the connection
            // For now, just mark it as closed
            self.is_open = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_config() {
        let config = ConnectionConfig::new("localhost:1521/XEPDB1", "testuser", "testpass")
            .mode(ConnectionMode::Thin)
            .privilege(Privilege::SysDba)
            .timeout(30);

        assert_eq!(config.user, "testuser");
        assert_eq!(config.mode, ConnectionMode::Thin);
        assert_eq!(config.privilege, Privilege::SysDba);
        assert_eq!(config.connect_timeout, 30);
    }
}
