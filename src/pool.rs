// Connection pooling

use crate::{Connection, ConnectionConfig, Error, Result};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Minimum number of connections in the pool
    pub pool_min: usize,
    /// Maximum number of connections in the pool
    pub pool_max: usize,
    /// Increment for pool expansion
    pub pool_increment: usize,
    /// Timeout for acquiring a connection (seconds)
    pub pool_timeout: u64,
    /// Connection idle timeout (seconds, 0 = no timeout)
    pub pool_idle_timeout: u64,
    /// Maximum lifetime of a connection (seconds, 0 = no limit)
    pub pool_max_lifetime: u64,
    /// Enable connection validation on checkout
    pub pool_ping_interval: u64,
    /// Queue timeout when pool is full (seconds)
    pub queue_timeout: u64,
    /// Maximum queue size (0 = unlimited)
    pub queue_max: usize,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            pool_min: 2,
            pool_max: 10,
            pool_increment: 1,
            pool_timeout: 60,
            pool_idle_timeout: 60,
            pool_max_lifetime: 3600,
            pool_ping_interval: 60,
            queue_timeout: 60,
            queue_max: 500,
        }
    }
}

impl PoolConfig {
    /// Create a new pool configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set minimum pool size
    pub fn min(mut self, min: usize) -> Self {
        self.pool_min = min;
        self
    }

    /// Set maximum pool size
    pub fn max(mut self, max: usize) -> Self {
        self.pool_max = max;
        self
    }

    /// Set pool increment
    pub fn increment(mut self, increment: usize) -> Self {
        self.pool_increment = increment;
        self
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.pool_min > self.pool_max {
            return Err(Error::InvalidConfiguration(
                "pool_min cannot be greater than pool_max".into(),
            ));
        }
        if self.pool_increment == 0 {
            return Err(Error::InvalidConfiguration(
                "pool_increment must be greater than 0".into(),
            ));
        }
        Ok(())
    }
}

/// Connection pool
pub struct Pool {
    config: ConnectionConfig,
    pool_config: PoolConfig,
    semaphore: Arc<Semaphore>,
    stats: Arc<tokio::sync::Mutex<PoolStats>>,
}

/// Pool statistics
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// Total connections created
    pub connections_created: u64,
    /// Total connections closed
    pub connections_closed: u64,
    /// Current number of connections in use
    pub connections_in_use: usize,
    /// Current number of idle connections
    pub connections_idle: usize,
    /// Total requests for connections
    pub connection_requests: u64,
    /// Total failed connection requests
    pub connection_timeouts: u64,
}

impl Pool {
    /// Create a new connection pool
    pub async fn new(config: ConnectionConfig, pool_config: PoolConfig) -> Result<Self> {
        pool_config.validate()?;

        let pool = Self {
            config,
            pool_config: pool_config.clone(),
            semaphore: Arc::new(Semaphore::new(pool_config.pool_max)),
            stats: Arc::new(tokio::sync::Mutex::new(PoolStats::default())),
        };

        // Initialize minimum connections
        pool.initialize_pool().await?;

        Ok(pool)
    }

    /// Initialize the pool with minimum connections
    async fn initialize_pool(&self) -> Result<()> {
        for _ in 0..self.pool_config.pool_min {
            // In a real implementation, we'd create and store connections
            // This is a simplified version
        }
        Ok(())
    }

    /// Get a connection from the pool
    pub async fn get_connection(&self) -> Result<PooledConnection> {
        let timeout = Duration::from_secs(self.pool_config.pool_timeout);

        // Update stats
        {
            let mut stats = self.stats.lock().await;
            stats.connection_requests += 1;
        }

        // Acquire semaphore permit
        let permit = tokio::time::timeout(timeout, self.semaphore.clone().acquire_owned())
            .await
            .map_err(|_| Error::PoolTimeout)?
            .map_err(|_| Error::PoolClosed)?;

        // Create or retrieve connection
        let conn = Connection::connect(self.config.clone()).await?;

        // Update stats
        {
            let mut stats = self.stats.lock().await;
            stats.connections_created += 1;
            stats.connections_in_use += 1;
        }

        Ok(PooledConnection {
            connection: Some(conn),
            pool: self.clone(),
            _permit: permit,
        })
    }

    /// Get pool statistics
    pub async fn get_stats(&self) -> PoolStats {
        self.stats.lock().await.clone()
    }

    /// Close the pool and all connections
    pub async fn close(&self) -> Result<()> {
        // In a real implementation, we'd close all connections
        Ok(())
    }

    /// Reconfigure the pool
    pub async fn reconfigure(&mut self, new_config: PoolConfig) -> Result<()> {
        new_config.validate()?;
        self.pool_config = new_config;
        Ok(())
    }
}

impl Clone for Pool {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            pool_config: self.pool_config.clone(),
            semaphore: self.semaphore.clone(),
            stats: self.stats.clone(),
        }
    }
}

/// A connection from the pool
pub struct PooledConnection {
    connection: Option<Connection>,
    #[allow(dead_code)]
    pool: Pool,
    _permit: tokio::sync::OwnedSemaphorePermit,
}

impl PooledConnection {
    /// Get a reference to the underlying connection
    pub fn connection(&self) -> &Connection {
        self.connection.as_ref().unwrap()
    }

    /// Get a mutable reference to the underlying connection
    pub fn connection_mut(&mut self) -> &mut Connection {
        self.connection.as_mut().unwrap()
    }
}

impl std::ops::Deref for PooledConnection {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        self.connection()
    }
}

impl std::ops::DerefMut for PooledConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.connection_mut()
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        // Return connection to pool
        // Update stats
        if let Some(_conn) = self.connection.take() {
            // In a real implementation, we'd return the connection to the pool
            // For now, the permit is automatically released
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_config_validation() {
        let config = PoolConfig::new().min(5).max(3);
        assert!(config.validate().is_err());

        let config = PoolConfig::new().min(2).max(10);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_pool_config_default() {
        let config = PoolConfig::default();
        assert_eq!(config.pool_min, 2);
        assert_eq!(config.pool_max, 10);
    }
}
