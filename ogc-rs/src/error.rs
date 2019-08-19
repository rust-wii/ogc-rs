//! Custom Error Implementation for ``ogc-rs``.

use alloc::string::String;
use core::fmt;

/// Custom Result Type that uses the error type.
pub type Result<T> = core::result::Result<T, OgcError>;

/// Custom Error Type
pub enum OgcError {
    Network(String),
    Audio(String),
    Console(String),
    System(String),
}

impl fmt::Debug for OgcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OgcError::Network(err) => write!(f, "[ OGC - Network ]: {}", err),
            OgcError::Audio(err) => write!(f, "[ OGC - Audio ]: {}", err),
            OgcError::Console(err) => write!(f, "[ OGC - Console ]: {}", err),
            OgcError::System(err) => write!(f, "[ OGC - System ]: {}", err),
        }
    }
}

impl fmt::Display for OgcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OgcError::Network(err) => write!(f, "[ OGC - Network ]: {}", err),
            OgcError::Audio(err) => write!(f, "[ OGC - Audio ]: {}", err),
            OgcError::Console(err) => write!(f, "[ OGC - Console ]: {}", err),
            OgcError::System(err) => write!(f, "[ OGC - System ]: {}", err),
        }
    }
}
