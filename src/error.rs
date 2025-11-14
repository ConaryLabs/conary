// src/error.rs

use thiserror::Error;

/// Core error types for Conary
#[derive(Error, Debug)]
pub enum Error {
    /// Database-related errors
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Database initialization error
    #[error("Failed to initialize database: {0}")]
    InitError(String),

    /// Database not found
    #[error("Database not found at path: {0}")]
    DatabaseNotFound(String),
}

/// Result type alias using Conary's Error type
pub type Result<T> = std::result::Result<T, Error>;
