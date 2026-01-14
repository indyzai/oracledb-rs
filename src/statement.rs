// SQL statement execution

use crate::protocol::Protocol;
use crate::types::{ColumnInfo, FromSql, ToSql, Value};
use crate::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Prepared statement
pub struct Statement {
    sql: String,
    protocol: Arc<Mutex<Protocol>>,
    metadata: Option<Vec<ColumnInfo>>,
}

impl Statement {
    /// Create a new statement
    pub fn new(sql: impl Into<String>, protocol: Arc<Mutex<Protocol>>) -> Self {
        Self {
            sql: sql.into(),
            protocol,
            metadata: None,
        }
    }

    /// Execute the statement and return results
    pub async fn execute(&self, params: &[&dyn ToSql]) -> Result<ResultSet> {
        let mut protocol = self.protocol.lock().await;

        // Convert parameters to Values
        let values: Vec<Value> = params.iter().map(|p| p.to_sql()).collect();

        // Execute statement through protocol
        let (rows, metadata) = protocol.execute(&self.sql, &values).await?;

        Ok(ResultSet {
            rows,
            metadata,
            current_row: 0,
        })
    }

    /// Execute DML and return affected rows
    pub async fn execute_dml(&self, params: &[&dyn ToSql]) -> Result<u64> {
        let mut protocol = self.protocol.lock().await;

        let values: Vec<Value> = params.iter().map(|p| p.to_sql()).collect();
        protocol.execute_dml(&self.sql, &values).await
    }

    /// Execute many statements with batch binding
    pub async fn execute_many(&self, batch_params: &[Vec<&dyn ToSql>]) -> Result<Vec<u64>> {
        let mut results = Vec::with_capacity(batch_params.len());

        for params in batch_params {
            let count = self.execute_dml(params.as_slice()).await?;
            results.push(count);
        }

        Ok(results)
    }

    /// Get statement metadata
    pub async fn get_metadata(&mut self) -> Result<&[ColumnInfo]> {
        if self.metadata.is_none() {
            let mut protocol = self.protocol.lock().await;
            let metadata = protocol.get_metadata(&self.sql).await?;
            self.metadata = Some(metadata);
        }

        Ok(self.metadata.as_ref().unwrap())
    }
}

/// Result set from query execution
pub struct ResultSet {
    rows: Vec<Row>,
    metadata: Vec<ColumnInfo>,
    current_row: usize,
}

impl ResultSet {
    /// Get number of rows in result set
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Check if result set is empty
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Get column metadata
    pub fn metadata(&self) -> &[ColumnInfo] {
        &self.metadata
    }

    /// Get all rows
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    /// Convert to vector of rows
    pub fn into_rows(self) -> Vec<Row> {
        self.rows
    }

    /// Fetch next row
    pub fn next(&mut self) -> Option<&Row> {
        if self.current_row < self.rows.len() {
            let row = &self.rows[self.current_row];
            self.current_row += 1;
            Some(row)
        } else {
            None
        }
    }

    /// Convert to typed results
    pub fn as_typed<T: FromRow>(&self) -> Result<Vec<T>> {
        self.rows.iter().map(|row| T::from_row(row)).collect()
    }
}

impl Iterator for ResultSet {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row < self.rows.len() {
            let row = self.rows[self.current_row].clone();
            self.current_row += 1;
            Some(row)
        } else {
            None
        }
    }
}

/// Row from query result
#[derive(Debug, Clone)]
pub struct Row {
    /// Column values (indexed)
    values: Vec<Value>,
    /// Column names mapped to indices
    columns: HashMap<String, usize>,
}

impl Row {
    /// Create a new row
    pub fn new(values: Vec<Value>, column_names: Vec<String>) -> Self {
        let columns = column_names
            .into_iter()
            .enumerate()
            .map(|(i, name)| (name, i))
            .collect();

        Self { values, columns }
    }

    /// Get value by index
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.values.get(index)
    }

    /// Get value by column name
    pub fn get_by_name(&self, name: &str) -> Option<&Value> {
        self.columns.get(name).and_then(|&i| self.values.get(i))
    }

    /// Get typed value by index
    pub fn get_typed<T: FromSql>(&self, index: usize) -> Result<T> {
        let value = self
            .get(index)
            .ok_or(Error::ColumnNotFound(index.to_string()))?;
        T::from_sql(value)
    }

    /// Get typed value by column name
    pub fn get_typed_by_name<T: FromSql>(&self, name: &str) -> Result<T> {
        let value = self
            .get_by_name(name)
            .ok_or(Error::ColumnNotFound(name.to_string()))?;
        T::from_sql(value)
    }

    /// Get all values
    pub fn values(&self) -> &[Value] {
        &self.values
    }

    /// Get number of columns
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Check if row is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Convert row to HashMap
    pub fn to_map(&self) -> HashMap<String, Value> {
        self.columns
            .iter()
            .map(|(name, &i)| (name.clone(), self.values[i].clone()))
            .collect()
    }
}

/// Trait for converting from a Row
pub trait FromRow: Sized {
    /// Convert from row
    fn from_row(row: &Row) -> Result<Self>;
}

// Example implementation for tuple
impl<T1: FromSql> FromRow for (T1,) {
    fn from_row(row: &Row) -> Result<Self> {
        Ok((row.get_typed(0)?,))
    }
}

impl<T1: FromSql, T2: FromSql> FromRow for (T1, T2) {
    fn from_row(row: &Row) -> Result<Self> {
        Ok((row.get_typed(0)?, row.get_typed(1)?))
    }
}

impl<T1: FromSql, T2: FromSql, T3: FromSql> FromRow for (T1, T2, T3) {
    fn from_row(row: &Row) -> Result<Self> {
        Ok((row.get_typed(0)?, row.get_typed(1)?, row.get_typed(2)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_creation() {
        let values = vec![Value::Integer(1), Value::String("test".to_string())];
        let columns = vec!["id".to_string(), "name".to_string()];

        let row = Row::new(values, columns);

        assert_eq!(row.len(), 2);
        assert!(matches!(row.get(0), Some(Value::Integer(1))));
        assert!(matches!(row.get_by_name("name"), Some(Value::String(_))));
    }

    #[test]
    fn test_row_typed_access() {
        let values = vec![Value::Integer(42)];
        let columns = vec!["count".to_string()];
        let row = Row::new(values, columns);

        let count: i64 = row.get_typed(0).unwrap();
        assert_eq!(count, 42);

        let count: i64 = row.get_typed_by_name("count").unwrap();
        assert_eq!(count, 42);
    }
}
