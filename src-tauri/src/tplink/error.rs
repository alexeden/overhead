#![allow(unused)]
//! Error types
use serde::{Deserialize, Serialize};
use std::{convert::From, error, fmt, io, result};

/// TPLinker result type with [Error](enum.Error.html)
pub type TpResult<T> = result::Result<T, TpError>;

/// Error type for TPLinker
#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub enum TpError {
    /// Wrapped errors from std::io
    IO(String),
    /// Wrapped errors from serde_json
    Serde(String),
    /// Error decoding a section of the JSON response
    TPLink(SectionError),
    /// A generic error
    Other(String),
}

impl fmt::Display for TpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TpError::IO(_) => f.write_str("Error connecting to the device"),
            TpError::Serde(_) => {
                f.write_str("Could not parse the response received from the device")
            }
            TpError::TPLink(SectionError { err_code, err_msg }) => f.write_str(&format!(
                "Response data error: ({:?}) {:?}",
                err_code, err_msg,
            )),
            TpError::Other(err) => f.write_str(&err),
        }
    }
}

impl error::Error for TpError {
    fn description(&self) -> &str {
        match self {
            TpError::IO(_) => "Error connecting to the device",
            TpError::Serde(_) => "Could not parse the response received from the device",
            TpError::TPLink(_) => "Response data error",
            TpError::Other(err) => err.as_str(),
        }
    }
}

impl From<io::Error> for TpError {
    fn from(error: io::Error) -> Self {
        // error.fmt(f)
        TpError::IO(format!("{:#?}", error))
    }
}

impl From<serde_json::Error> for TpError {
    fn from(error: serde_json::Error) -> Self {
        TpError::Serde(format!("{:#?}", error))
    }
}

impl From<&str> for TpError {
    fn from(error: &str) -> Self {
        TpError::from(String::from(error))
    }
}

impl From<String> for TpError {
    fn from(error: String) -> Self {
        TpError::Other(error)
    }
}

impl From<SectionError> for TpError {
    fn from(error: SectionError) -> Self {
        TpError::TPLink(error)
    }
}

/// Error response for a section of the JSON response
#[derive(Debug, Deserialize, Serialize, Clone, specta::Type)]
pub struct SectionError {
    /// The error code. Zero if no error.
    pub err_code: Option<i16>,

    /// The error message.
    pub err_msg: Option<String>,
}

impl fmt::Display for SectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{:?}: {:?}", self.err_code, self.err_msg))
    }
}

impl error::Error for SectionError {
    fn description(&self) -> &str {
        "TPLink section error"
    }
}
