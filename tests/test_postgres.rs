//! Tests for PostgreSQL integration in rosetta_utc
#![cfg(feature = "postgres")]

use diesel::pg::PgConnection;
use diesel::prelude::*;
use rosetta_utc::TimestampUTC;
use std::env;

diesel::table! {
    /// The test table for PostgreSQL integration tests.
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
fn test_postgres_roundtrip() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url).expect("Error connecting to database");

    conn.test_transaction::<_, diesel::result::Error, _>(|conn| {
        diesel::sql_query("CREATE TEMPORARY TABLE test_table (id SERIAL PRIMARY KEY, created_at TIMESTAMPTZ NOT NULL)")
            .execute(conn)?;
        let now = TimestampUTC::now();
        // Postgres has microsecond precision, so we must truncate nanoseconds for equality check to succeed.
        let dt = now.as_ref();
        let truncated = chrono::DateTime::from_timestamp(
            dt.timestamp(),
            dt.timestamp_subsec_micros() * 1000
        ).unwrap();
        let now = TimestampUTC::from(truncated);

        let entity = TestEntity {
            id: 1,
            created_at: now,
        };

        diesel::insert_into(test_table::table)
            .values(&entity)
            .execute(conn)?;

        let result: TestEntity = test_table::table
            .find(1)
            .first(conn)?;

        assert_eq!(result.id, entity.id);
        assert_eq!(result.created_at, entity.created_at);
        Ok(())
    });
}
