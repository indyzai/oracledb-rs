// Connection pool example

use oracledb_rs::{ConnectionConfig, Pool, PoolConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Configure connection
    let conn_config = ConnectionConfig::new("localhost:1521/XEPDB1", "hr", "hr_password");

    // Configure pool
    let pool_config = PoolConfig::new().min(2).max(10).increment(2);

    // Create pool
    let pool = Pool::new(conn_config, pool_config).await?;
    println!("Connection pool created!");

    // Use connection from pool
    let conn = pool.get_connection().await?;

    let result = conn
        .execute(
            "SELECT COUNT(*) FROM employees WHERE salary > :1",
            &[&50000.0],
        )
        .await?;

    if let Some(row) = result.rows().first() {
        let count: i64 = row.get_typed(0)?;
        println!("Employees with salary > 50000: {}", count);
    }

    // Connection automatically returned to pool when dropped
    drop(conn);

    // Get pool statistics
    let stats = pool.get_stats().await;
    println!("Pool stats: {:?}", stats);

    Ok(())
}
