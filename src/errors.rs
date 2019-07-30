use hex;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AppError {
    Custom(String),
    IOError(std::io::Error),
    HexError(hex::FromHexError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) =>
                format!("\n{}\n", msg),
            AppError::HexError(ref e) =>
                format!("\n✘ Hex Error!\n✘ {}\n", e),
            AppError::IOError(ref e) =>
                format!("\n✘ I/O Error!\n✘ {}\n", e),
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
