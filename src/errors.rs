use hex;
use log;
use reqwest;
use std::fmt;
use serde_json;
use std::error::Error;

#[derive(Debug)]
pub enum AppError {
    Custom(String),
    IOError(std::io::Error),
    HexError(hex::FromHexError),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    NoneError(std::option::NoneError),
    SetLoggerError(log::SetLoggerError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) =>
                format!("{}", msg),
            AppError::HexError(ref e) =>
                format!("✘ Hex Error!\n✘ {}", e),
            AppError::IOError(ref e) =>
                format!("✘ I/O Error!\n✘ {}", e),
            AppError::SerdeJsonError(ref e) =>
                format!("✘ Serde-Json Error!\n✘ {}", e),
            AppError::NoneError(ref e) =>
                format!("✘ Nothing to unwrap!\n✘ {:?}", e),
            AppError::SetLoggerError(ref e) =>
                format!("✘ Error setting up logger!\n✘ {}", e),
            AppError::ReqwestError(ref e) =>
                format!(
                    "\n✘ HTTP Reqwest Error!\n✘ {}\n{}",
                    e,
                    "✘ Please check your node & port settings and retry.\n"
                ),
        };
        f.write_fmt(format_args!("{}", msg))
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        "\n✘ Program Error!\n"
    }
}

impl From<hex::FromHexError> for AppError {
    fn from(e: hex::FromHexError) -> AppError {
        AppError::HexError(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::IOError(e)
    }
}

impl From<std::option::NoneError> for AppError {
    fn from(e: std::option::NoneError) -> AppError {
        AppError::NoneError(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> AppError {
        AppError::ReqwestError(e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> AppError {
        AppError::SerdeJsonError(e)
    }
}

impl From<log::SetLoggerError> for AppError {
    fn from(e: log::SetLoggerError) -> AppError {
        AppError::SetLoggerError(e)
    }
}
