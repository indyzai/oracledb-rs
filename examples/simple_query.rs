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
