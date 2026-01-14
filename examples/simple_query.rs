// Simple query example

use oracledb_rs::{Connection, ConnectionConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Configure connection
    let config = ConnectionConfig::new("localhost:1521/XEPDB1", "hr", "hr_password");

    // Connect to database
    let conn = Connection::connect(config).await?;
    println!("Connected to Oracle Database!");

    // Simple query
    let sql = "SELECT employee_id, first_name, last_name, salary 
               FROM employees 
               WHERE department_id = :1 
               ORDER BY employee_id";

    let result = conn.execute(sql, &[&10]).await?;

    // Print results
    println!("Found {} employees:", result.len());
    for row in result.rows() {
        let emp_id: i64 = row.get_typed(0)?;
        let first_name: String = row.get_typed(1)?;
        let last_name: String = row.get_typed(2)?;
        let salary: f64 = row.get_typed(3)?;

        println!(
            "ID: {}, Name: {} {}, Salary: ${:.2}",
            emp_id, first_name, last_name, salary
        );
    }

    // Close connection
    conn.close().await?;
    Ok(())
}

// examples/connection_pool.rs - Connection pool example

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
    let mut conn = pool.get_connection().await?;

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

// examples/transactions.rs - Transaction example

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

// examples/batch_insert.rs - Batch insert example

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

// examples/prepared_statement.rs - Prepared statement example

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

// examples/lob_handling.rs - LOB handling example

use oracledb_rs::{Connection, ConnectionConfig, Result, types::Value};

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

// examples/error_handling.rs - Error handling example

use oracledb_rs::{Connection, ConnectionConfig, Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = ConnectionConfig::new("localhost:1521/XEPDB1", "hr", "wrong_password");

    match Connection::connect(config).await {
        Ok(_) => println!("Connected successfully"),
        Err(e) => {
            match e {
                Error::AuthenticationFailed(msg) => {
                    eprintln!("Authentication failed: {}", msg);
                }
                Error::Connection(msg) => {
                    eprintln!("Connection error: {}", msg);
                }
                _ => {
                    eprintln!("Other error: {}", e);
                }
            }

            // Check if error is retryable
            if e.is_retryable() {
                eprintln!("This error is retryable");
            }
        }
    }

    Ok(())
}
