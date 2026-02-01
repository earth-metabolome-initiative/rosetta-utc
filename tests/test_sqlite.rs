//! Tests for SQLite support in rosetta_utc
#![cfg(feature = "sqlite")]

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use rosetta_utc::TimestampUTC;

diesel::table! {
    /// The test table for SQLite integration tests.
    test_table (id) {
        /// The primary key column.
        id -> Integer,
        /// The UTC timestamp column.
        created_at -> rosetta_utc::diesel_impls::TimestampUTC,
    }
}

#[derive(Queryable, Insertable, Debug, PartialEq)]
#[diesel(table_name = test_table)]
struct TestEntity {
    id: i32,
    created_at: TimestampUTC,
}

#[test]
fn test_sqlite_roundtrip() {
    let mut conn = SqliteConnection::establish(":memory:").unwrap();

    diesel::sql_query("CREATE TABLE test_table (id INTEGER PRIMARY KEY, created_at TEXT NOT NULL)")
        .execute(&mut conn)
        .unwrap();

    let now = TimestampUTC::now();
    let entity = TestEntity {
        id: 1,
        created_at: now,
    };

    diesel::insert_into(test_table::table)
        .values(&entity)
        .execute(&mut conn)
        .unwrap();

    let result: TestEntity = test_table::table.find(1).first(&mut conn).unwrap();

    assert_eq!(result.id, entity.id);
    assert_eq!(result.created_at, entity.created_at);
}
