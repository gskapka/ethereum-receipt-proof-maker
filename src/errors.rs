use hex;
use std::error::Error;
use std::{fmt, option};

#[derive(Debug)]
pub enum AppError {
    Custom(String),
    HexError(hex::FromHexError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) =>
                format!("\n{}\n", msg),
            AppError::HexError(ref e) =>
                format!("\n✘ Hex Error!\n✘ {}\n", e),
        };
        f.write_fmt(format_args!("{}", msg))
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        "\n✘ Program Error!\n"
    }
}
