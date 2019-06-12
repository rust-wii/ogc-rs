//! Custom Error Implementation for ``ogc-rs``.

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

/// Custom Result Type that uses the error type.
pub type Result<T> = std::result::Result<T, OgcError>;

/// Custom Error Type
pub enum OgcError {
    Network(String),
}

impl Debug for OgcError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            OgcError::Network(err) => write!(f, "[ OGC - Network ]: {}", err),
        }
    }
}

impl Display for OgcError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            OgcError::Network(err) => write!(f, "[ OGC - Network ]: {}", err),
        }
    }
}
