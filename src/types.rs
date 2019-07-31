use std::result;
use crate::errors::AppError;

pub type Bytes = Vec<u8>;
pub type Result<T> = result::Result<T, AppError>;
