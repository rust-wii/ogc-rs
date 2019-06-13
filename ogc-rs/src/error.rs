//! Custom Error Implementation for ``ogc-rs``.

use std::fmt;

/// Custom Result Type that uses the error type.
pub type Result<T> = std::result::Result<T, OgcError>;

/// Custom Error Type
pub enum OgcError {
    Network(String),
}

impl fmt::Debug for OgcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OgcError::Network(err) => write!(f, "[ OGC - Network ]: {}", err),
        }
    }
}

impl fmt::Display for OgcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OgcError::Network(err) => write!(f, "[ OGC - Network ]: {}", err),
        }
    }
}
