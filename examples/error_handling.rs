// Error handling example

use oracledb_rs::{Connection, ConnectionConfig, Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = ConnectionConfig::new("localhost:1521/XEPDB1", "hr", "wrong_password");

    match Connection::connect(config).await {
        Ok(_) => println!("Connected successfully"),
        Err(e) => {
            // Format error message before moving e
            let error_msg = format!("{}", e);
            let is_retryable = e.is_retryable();

            match e {
                Error::AuthenticationFailed(msg) => {
                    eprintln!("Authentication failed: {}", msg);
                }
                Error::Connection(msg) => {
                    eprintln!("Connection error: {}", msg);
                }
                _ => {
                    eprintln!("Other error: {}", error_msg);
                }
            }

            // Check if error is retryable
            if is_retryable {
                eprintln!("This error is retryable");
            }
        }
    }

    Ok(())
}
