// Transaction example

use oracledb_rs::{Connection, ConnectionConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = ConnectionConfig::new("localhost:1521/XEPDB1", "hr", "hr_password");

    let mut conn = Connection::connect(config).await?;

    // Start transaction (implicit)
    let affected = conn
        .execute_dml(
            "UPDATE employees SET salary = salary * 1.1 WHERE department_id = :1",
            &[&10],
        )
        .await?;

    println!("Updated {} rows", affected);

    // You can rollback if needed
    if affected > 100 {
        conn.rollback().await?;
        println!("Rolled back transaction");
    } else {
        conn.commit().await?;
        println!("Committed transaction");
    }

    conn.close().await?;
    Ok(())
}
