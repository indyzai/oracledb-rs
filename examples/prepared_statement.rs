// Prepared statement example

use oracledb_rs::{Connection, ConnectionConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = ConnectionConfig::new("localhost:1521/XEPDB1", "hr", "hr_password");

    let conn = Connection::connect(config).await?;

    // Prepare statement once
    let stmt = conn
        .prepare("SELECT * FROM employees WHERE department_id = :1 AND salary > :2")
        .await?;

    // Execute multiple times with different parameters
    for dept_id in [10, 20, 30] {
        let result = stmt.execute(&[&dept_id, &40000.0]).await?;
        println!("Department {}: {} employees", dept_id, result.len());
    }

    conn.close().await?;
    Ok(())
}
