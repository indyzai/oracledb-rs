// LOB handling example

use oracledb_rs::{types::Value, Connection, ConnectionConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = ConnectionConfig::new("localhost:1521/XEPDB1", "hr", "hr_password");

    let mut conn = Connection::connect(config).await?;

    // Insert CLOB data
    let large_text = "This is a large text document...".repeat(1000);
    conn.execute_dml(
        "INSERT INTO documents (id, content) VALUES (:1, :2)",
        &[&1, &large_text.as_str()],
    )
    .await?;

    // Read CLOB data
    let result = conn
        .execute("SELECT content FROM documents WHERE id = :1", &[&1])
        .await?;

    if let Some(row) = result.rows().first() {
        if let Some(content) = row.get(0) {
            match content {
                Value::Clob(text) => {
                    println!("CLOB length: {} characters", text.len());
                }
                _ => println!("Unexpected type"),
            }
        }
    }

    conn.commit().await?;
    conn.close().await?;
    Ok(())
}
