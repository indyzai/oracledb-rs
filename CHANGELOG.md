# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-14

### Added
- Initial release of oracledb-rs
- Basic connection management (thin mode)
- Async/await support with Tokio
- Connection pooling with configurable sizing
- Prepared statements with automatic caching
- Transaction support (commit/rollback)
- Type-safe parameter binding
- Result set handling with typed access
- Error handling with retryable error detection
- Support for common Oracle data types:
  - VARCHAR2, NVARCHAR2, CHAR, NCHAR
  - NUMBER, BINARY_FLOAT, BINARY_DOUBLE
  - DATE, TIMESTAMP, TIMESTAMP WITH TIME ZONE
  - RAW, CLOB, NCLOB, BLOB
  - JSON
  - BOOLEAN (PL/SQL)
- Batch DML operations
- Comprehensive examples
- Full documentation

### Planned
- Thick mode support (Oracle Client libraries)
- REF CURSOR support
- PL/SQL stored procedures
- Advanced Queuing (AQ)
- Continuous Query Notification (CQN)
- SODA (Simple Oracle Document Access)
- Full TNS connection string parsing
- Async streaming for large result sets

[Unreleased]: https://github.com/indyzai/oracledb-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/indyzai/oracledb-rs/releases/tag/v0.1.0
