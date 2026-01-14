// Oracle data type mappings

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Oracle data types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OracleType {
    /// VARCHAR2
    Varchar2,
    /// NVARCHAR2
    NVarchar2,
    /// CHAR
    Char,
    /// NCHAR
    NChar,
    /// NUMBER
    Number,
    /// BINARY_FLOAT
    BinaryFloat,
    /// BINARY_DOUBLE
    BinaryDouble,
    /// DATE
    Date,
    /// TIMESTAMP
    Timestamp,
    /// TIMESTAMP WITH TIME ZONE
    TimestampTz,
    /// TIMESTAMP WITH LOCAL TIME ZONE
    TimestampLtz,
    /// INTERVAL YEAR TO MONTH
    IntervalYM,
    /// INTERVAL DAY TO SECOND
    IntervalDS,
    /// RAW
    Raw,
    /// LONG RAW
    LongRaw,
    /// ROWID
    Rowid,
    /// UROWID
    URowid,
    /// CLOB
    Clob,
    /// NCLOB
    NClob,
    /// BLOB
    Blob,
    /// BFILE
    BFile,
    /// JSON
    Json,
    /// XMLTYPE
    XmlType,
    /// Object type
    Object,
    /// REF CURSOR
    RefCursor,
    /// Boolean (PL/SQL only)
    Boolean,
}

/// Value wrapper for Oracle types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    /// NULL value
    Null,
    /// String value
    String(String),
    /// Integer value
    Integer(i64),
    /// Float value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// Date value
    Date(NaiveDate),
    /// Timestamp value
    Timestamp(NaiveDateTime),
    /// Timestamp with timezone
    TimestampTz(DateTime<Utc>),
    /// Binary data
    Bytes(Vec<u8>),
    /// CLOB data
    Clob(String),
    /// BLOB data
    Blob(Vec<u8>),
    /// JSON data
    Json(serde_json::Value),
    /// Array of values
    Array(Vec<Value>),
    /// Object (key-value pairs)
    Object(HashMap<String, Value>),
}

impl Value {
    /// Check if value is NULL
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Try to convert to string
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            Value::Clob(s) => Some(s),
            _ => None,
        }
    }

    /// Try to convert to integer
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Try to convert to float
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Try to convert to boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Try to convert to bytes
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Bytes(b) => Some(b),
            Value::Blob(b) => Some(b),
            _ => None,
        }
    }
}

/// Trait for types that can be converted to SQL values
pub trait ToSql: Send + Sync {
    /// Convert to Oracle value
    fn to_sql(&self) -> Value;
}

/// Trait for types that can be converted from SQL values
pub trait FromSql: Sized {
    /// Convert from Oracle value
    fn from_sql(value: &Value) -> Result<Self, crate::Error>;
}

// Implementations for basic types
impl ToSql for String {
    fn to_sql(&self) -> Value {
        Value::String(self.clone())
    }
}

impl ToSql for &str {
    fn to_sql(&self) -> Value {
        Value::String(self.to_string())
    }
}

impl ToSql for i32 {
    fn to_sql(&self) -> Value {
        Value::Integer(*self as i64)
    }
}

impl ToSql for i64 {
    fn to_sql(&self) -> Value {
        Value::Integer(*self)
    }
}

impl ToSql for f32 {
    fn to_sql(&self) -> Value {
        Value::Float(*self as f64)
    }
}

impl ToSql for f64 {
    fn to_sql(&self) -> Value {
        Value::Float(*self)
    }
}

impl ToSql for bool {
    fn to_sql(&self) -> Value {
        Value::Boolean(*self)
    }
}

impl ToSql for Vec<u8> {
    fn to_sql(&self) -> Value {
        Value::Bytes(self.clone())
    }
}

impl ToSql for NaiveDate {
    fn to_sql(&self) -> Value {
        Value::Date(*self)
    }
}

impl ToSql for NaiveDateTime {
    fn to_sql(&self) -> Value {
        Value::Timestamp(*self)
    }
}

impl ToSql for DateTime<Utc> {
    fn to_sql(&self) -> Value {
        Value::TimestampTz(*self)
    }
}

impl<T: ToSql> ToSql for Option<T> {
    fn to_sql(&self) -> Value {
        match self {
            Some(v) => v.to_sql(),
            None => Value::Null,
        }
    }
}

// FromSql implementations
impl FromSql for String {
    fn from_sql(value: &Value) -> Result<Self, crate::Error> {
        match value {
            Value::String(s) => Ok(s.clone()),
            Value::Clob(s) => Ok(s.clone()),
            _ => Err(crate::Error::TypeMismatch(format!(
                "Cannot convert {:?} to String",
                value
            ))),
        }
    }
}

impl FromSql for i64 {
    fn from_sql(value: &Value) -> Result<Self, crate::Error> {
        match value {
            Value::Integer(i) => Ok(*i),
            _ => Err(crate::Error::TypeMismatch(format!(
                "Cannot convert {:?} to i64",
                value
            ))),
        }
    }
}

impl FromSql for f64 {
    fn from_sql(value: &Value) -> Result<Self, crate::Error> {
        match value {
            Value::Float(f) => Ok(*f),
            Value::Integer(i) => Ok(*i as f64),
            _ => Err(crate::Error::TypeMismatch(format!(
                "Cannot convert {:?} to f64",
                value
            ))),
        }
    }
}

impl FromSql for bool {
    fn from_sql(value: &Value) -> Result<Self, crate::Error> {
        match value {
            Value::Boolean(b) => Ok(*b),
            _ => Err(crate::Error::TypeMismatch(format!(
                "Cannot convert {:?} to bool",
                value
            ))),
        }
    }
}

impl<T: FromSql> FromSql for Option<T> {
    fn from_sql(value: &Value) -> Result<Self, crate::Error> {
        match value {
            Value::Null => Ok(None),
            _ => Ok(Some(T::from_sql(value)?)),
        }
    }
}

/// Column metadata
#[derive(Debug, Clone)]
pub struct ColumnInfo {
    /// Column name
    pub name: String,
    /// Oracle data type
    pub oracle_type: OracleType,
    /// Column size
    pub size: usize,
    /// Precision (for numeric types)
    pub precision: Option<u8>,
    /// Scale (for numeric types)
    pub scale: Option<i8>,
    /// Nullable
    pub nullable: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_conversions() {
        let v = Value::String("test".to_string());
        assert_eq!(v.as_str(), Some("test"));
        assert!(v.as_i64().is_none());

        let v = Value::Integer(42);
        assert_eq!(v.as_i64(), Some(42));
        assert_eq!(v.as_f64(), Some(42.0));
    }

    #[test]
    fn test_to_sql() {
        let s = "hello";
        assert!(matches!(s.to_sql(), Value::String(_)));

        let i = 42i64;
        assert!(matches!(i.to_sql(), Value::Integer(42)));

        let b = true;
        assert!(matches!(b.to_sql(), Value::Boolean(true)));
    }
}
