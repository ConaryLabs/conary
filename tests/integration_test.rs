// tests/integration_test.rs

//! Integration tests for Conary
//!
//! These tests verify end-to-end functionality across modules.

use conary::db;
use tempfile::NamedTempFile;

#[test]
fn test_database_lifecycle() {
    // Create a temporary database
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_str().unwrap().to_string();

    // Remove the temp file so init can create it
    drop(temp_file);

    // Initialize the database
    let init_result = db::init(&db_path);
    assert!(
        init_result.is_ok(),
        "Database initialization should succeed"
    );

    // Verify database file exists
    assert!(
        std::path::Path::new(&db_path).exists(),
        "Database file should exist after initialization"
    );

    // Open the database
    let conn_result = db::open(&db_path);
    assert!(conn_result.is_ok(), "Opening database should succeed");

    // Verify we can execute a simple query
    let conn = conn_result.unwrap();
    let result: Result<i32, _> = conn.query_row("SELECT 1", [], |row| row.get(0));
    assert_eq!(result.unwrap(), 1, "Should be able to execute queries");
}

#[test]
fn test_database_init_creates_parent_directories() {
    let temp_dir = tempfile::tempdir().unwrap();
    let db_path = temp_dir
        .path()
        .join("nested/path/to/conary.db")
        .to_str()
        .unwrap()
        .to_string();

    let result = db::init(&db_path);
    assert!(result.is_ok(), "Should create parent directories");
    assert!(
        std::path::Path::new(&db_path).exists(),
        "Database should exist in nested path"
    );
}

#[test]
fn test_database_pragmas_are_set() {
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_str().unwrap().to_string();
    drop(temp_file);

    db::init(&db_path).unwrap();
    let conn = db::open(&db_path).unwrap();

    // Verify foreign keys are enabled
    let foreign_keys: i32 = conn
        .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
        .unwrap();
    assert_eq!(foreign_keys, 1, "Foreign keys should be enabled");

    // Verify WAL mode (on a fresh init)
    let journal_mode: String = conn
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .unwrap();
    assert_eq!(
        journal_mode.to_lowercase(),
        "wal",
        "Journal mode should be WAL"
    );
}
