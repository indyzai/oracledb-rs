// Batch insert example

use oracledb_rs::{Connection, ConnectionConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = ConnectionConfig::new("localhost:1521/XEPDB1", "hr", "hr_password");

    let mut conn = Connection::connect(config).await?;

    // Prepare batch data
    let batch_data = vec![
        vec![&"John" as &dyn oracledb_rs::types::ToSql, &"Doe", &50000.0],
        vec![&"Jane", &"Smith", &60000.0],
        vec![&"Bob", &"Johnson", &55000.0],
    ];

    // Execute batch insert
    let sql = "INSERT INTO employees (first_name, last_name, salary) VALUES (:1, :2, :3)";
    let results = conn.execute_many(sql, &batch_data).await?;

    println!("Inserted {} rows", results.len());

    conn.commit().await?;
    conn.close().await?;
    Ok(())
}
