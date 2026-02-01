# rosetta-utc

[![CI](https://github.com/earth-metabolome-initiative/rosetta-utc/actions/workflows/ci.yml/badge.svg)](https://github.com/earth-metabolome-initiative/rosetta-utc/actions/workflows/ci.yml)
[![Security Audit](https://github.com/earth-metabolome-initiative/rosetta-utc/workflows/Security%20Audit/badge.svg)](https://github.com/earth-metabolome-initiative/rosetta-utc/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Codecov](https://codecov.io/gh/earth-metabolome-initiative/rosetta-utc/branch/main/graph/badge.svg)](https://codecov.io/gh/earth-metabolome-initiative/rosetta-utc)
[![Crates.io](https://img.shields.io/crates/v/rosetta-utc.svg)](https://crates.io/crates/rosetta-utc)
[![Docs.rs](https://docs.rs/rosetta-utc/badge.svg)](https://docs.rs/rosetta-utc)

A wrapper implementation of `DateTime<Utc>` providing binary `diesel` bindings for SQLite and PostgreSQL.

## Why it exists

This crate bridges the gap, providing a unified `TimestampUTC` type that:

* In **PostgreSQL**, maps to `TIMESTAMPTZ` (Timestamp with time zone), with [`Timestamptz`](https://docs.diesel.rs/2.0.x/diesel/sql_types/struct.Timestamptz.html) sql type.
* In **SQLite**, maps to `TEXT`, storing ISO8639 strings, with [`TimestamptzSqlite`](https://docs.rs/diesel/latest/diesel/sql_types/struct.TimestamptzSqlite.html)

The two SQL types are remapped to the same Rust type, `TimestampUTC`, which internally uses `chrono::DateTime<Utc>`.

This ensures consistent UTC handling across both databases, preventing common timezone-related bugs in distributed applications.

## Features

This crate provides a `TimestampUTC` wrapper type that implements various traits based on enabled features:

* **[`diesel`](https://crates.io/crates/diesel)**: Enables Diesel integration.
  * **`postgres`**: Enables `TimestampUTC` support for [PostgreSQL](https://www.postgresql.org/docs/current/datatype-datetime.html).
  * **`sqlite`**: Enables `TimestampUTC` support for [SQLite](https://www.sqlite.org/datatype3.html).
* **[`serde`](https://crates.io/crates/serde)**: Enables serialization and deserialization via [Serde](https://serde.rs/).
* **`wasm`**: Enables support for `TimestampUTC::now()` on `wasm32-unknown-unknown` targets by enabling `chrono/wasmbind`.

## Usage

Add this to your `Cargo.toml`. Select the features matching your database requirements.

```toml
[dependencies]
rosetta-utc = { version = "0.1", features = ["diesel", "postgres", "serde"] }
```

### Example

```rust
use rosetta_utc::TimestampUTC;
use core::str::FromStr;

// Get current UTC time
let now = TimestampUTC::now();

// Parse from string (RFC 3339)
let parsed = TimestampUTC::from_str("2023-10-27T10:00:00+00:00").unwrap();

// Access underlying chrono::DateTime<Utc> methods via Deref
println!("Timestamp: {}", now.timestamp());
```
