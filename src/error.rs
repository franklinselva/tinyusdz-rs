//! Error types for tinyusdz-rs.

use std::ffi::NulError;

/// Error type for tinyusdz operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to load USD file.
    #[error("Failed to load USD file: {0}")]
    LoadError(String),

    /// Invalid prim path.
    #[error("Invalid prim path: {0}")]
    InvalidPath(String),

    /// Type mismatch when accessing values.
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    /// Null pointer returned from C API.
    #[error("Null pointer returned from C API")]
    NullPointer,

    /// Invalid UTF-8 string.
    #[error("Invalid UTF-8 string: {0}")]
    InvalidUtf8(#[from] std::str::Utf8Error),

    /// String contains interior null byte.
    #[error("String contains interior null byte: {0}")]
    NulError(#[from] NulError),

    /// Property not found.
    #[error("Property not found: {0}")]
    PropertyNotFound(String),

    /// Attribute not found.
    #[error("Attribute not found: {0}")]
    AttributeNotFound(String),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Index out of bounds.
    #[error("Index out of bounds: {index} >= {len}")]
    IndexOutOfBounds { index: usize, len: usize },
}

/// Result type for tinyusdz operations.
pub type Result<T> = std::result::Result<T, Error>;
