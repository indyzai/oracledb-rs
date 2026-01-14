// Additional result utilities

use crate::types::Value;
use serde::{Deserialize, Serialize};

/// Query result formatting options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultFormat {
    /// Results as array of arrays
    Array,
    /// Results as array of objects
    Object,
}

/// Metadata for query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMetadata {
    /// Statement executed
    pub statement: String,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Number of rows affected/returned
    pub row_count: usize,
    /// Whether more rows are available
    pub more_rows: bool,
}

/// Extended result information
#[derive(Debug, Clone)]
pub struct ExtendedResult {
    /// The actual data values
    pub rows: Vec<Vec<Value>>,
    /// Column names
    pub column_names: Vec<String>,
    /// Metadata about the query
    pub metadata: QueryMetadata,
}

impl ExtendedResult {
    /// Create a new extended result
    pub fn new(rows: Vec<Vec<Value>>, column_names: Vec<String>, metadata: QueryMetadata) -> Self {
        Self {
            rows,
            column_names,
            metadata,
        }
    }

    /// Get number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get column names
    pub fn columns(&self) -> &[String] {
        &self.column_names
    }

    /// Convert to JSON-friendly format
    pub fn to_objects(&self) -> Vec<serde_json::Value> {
        self.rows
            .iter()
            .map(|row| {
                let mut obj = serde_json::Map::new();
                for (i, value) in row.iter().enumerate() {
                    let col_name = &self.column_names[i];
                    obj.insert(col_name.clone(), value_to_json(value));
                }
                serde_json::Value::Object(obj)
            })
            .collect()
    }
}

/// Convert Oracle Value to JSON value
fn value_to_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::Value::Null,
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Integer(i) => serde_json::Value::Number((*i).into()),
        Value::Float(f) => serde_json::Number::from_f64(*f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        Value::Boolean(b) => serde_json::Value::Bool(*b),
        Value::Date(d) => serde_json::Value::String(d.to_string()),
        Value::Timestamp(ts) => serde_json::Value::String(ts.to_string()),
        Value::TimestampTz(ts) => serde_json::Value::String(ts.to_rfc3339()),
        Value::Bytes(b) => {
            // Base64 encode binary data
            serde_json::Value::String(base64_encode(b))
        }
        Value::Clob(s) => serde_json::Value::String(s.clone()),
        Value::Blob(b) => serde_json::Value::String(base64_encode(b)),
        Value::Json(j) => j.clone(),
        Value::Array(arr) => {
            let json_arr: Vec<_> = arr.iter().map(value_to_json).collect();
            serde_json::Value::Array(json_arr)
        }
        Value::Object(obj) => {
            let json_obj: serde_json::Map<_, _> = obj
                .iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect();
            serde_json::Value::Object(json_obj)
        }
    }
}

/// Simple base64 encoding helper
fn base64_encode(data: &[u8]) -> String {
    use std::fmt::Write;
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        buf[..chunk.len()].copy_from_slice(chunk);

        let b64_chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        let _ = write!(
            &mut result,
            "{}{}{}{}",
            b64_chars[(buf[0] >> 2) as usize] as char,
            b64_chars[(((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize] as char,
            if chunk.len() > 1 {
                b64_chars[(((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize] as char
            } else {
                '='
            },
            if chunk.len() > 2 {
                b64_chars[(buf[2] & 0x3f) as usize] as char
            } else {
                '='
            }
        );
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_to_json() {
        let val = Value::String("test".to_string());
        let json = value_to_json(&val);
        assert_eq!(json, serde_json::Value::String("test".to_string()));

        let val = Value::Integer(42);
        let json = value_to_json(&val);
        assert_eq!(json, serde_json::Value::Number(42.into()));

        let val = Value::Null;
        let json = value_to_json(&val);
        assert_eq!(json, serde_json::Value::Null);
    }

    #[test]
    fn test_extended_result() {
        let rows = vec![
            vec![Value::Integer(1), Value::String("Alice".to_string())],
            vec![Value::Integer(2), Value::String("Bob".to_string())],
        ];
        let columns = vec!["id".to_string(), "name".to_string()];
        let metadata = QueryMetadata {
            statement: "SELECT * FROM users".to_string(),
            execution_time_ms: 10,
            row_count: 2,
            more_rows: false,
        };

        let result = ExtendedResult::new(rows, columns, metadata);
        assert_eq!(result.row_count(), 2);
        assert_eq!(result.columns().len(), 2);

        let objects = result.to_objects();
        assert_eq!(objects.len(), 2);
    }
}
