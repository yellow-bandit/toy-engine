//! Errors definition module.

use std::io;

/// Structure for representing errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// IO error from the standard library.
    #[error("IO error")]
    IOError(#[from] io::Error),
    /// CSV error.
    #[error("CSV error")]
    CSVError(#[from] csv::Error),
    /// An unknown error.
    #[error("unknown error")]
    Unknown,
}
