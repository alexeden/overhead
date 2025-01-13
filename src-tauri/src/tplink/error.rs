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
    /// Unknown device model
    UnknownModel(String),
    /// Tried to use a feature that is not supported by the device
    Unsupported(String),
    /// A generic error
    Unknown(String),
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
            TpError::Unknown(err) => f.write_str(&err),
            TpError::UnknownModel(model) => {
                f.write_str(&format!("Unknown device model: {:?}", model))
            }
            TpError::Unsupported(feature) => {
                f.write_str(&format!("Unsupported feature: {:?}", feature))
            }
        }
    }
}

impl error::Error for TpError {
    fn description(&self) -> &str {
        match self {
            TpError::IO(_) => "Error connecting to the device",
            TpError::Serde(_) => "Could not parse the response received from the device",
            TpError::TPLink(_) => "Response data error",
            TpError::Unknown(_) => "Unknown error",
            TpError::UnknownModel(_) => "Unknown device model",
            TpError::Unsupported(feature) => "Unsupported feature",
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
        TpError::Unknown(String::from(error))
    }
}

impl From<String> for TpError {
    fn from(error: String) -> Self {
        TpError::Unknown(error)
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
