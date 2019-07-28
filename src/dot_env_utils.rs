use std::{fs, result};
use crate::errors::AppError;
use crate::constants::DOT_ENV_PATH;

type Result<T> = result::Result<T, AppError>;

pub fn read_env_file() -> Result<String> {
    Ok(fs::read_to_string(&DOT_ENV_PATH)?)
}

pub fn write_env_file() -> Result<()> {
    let data = "ENDPOINT=\"https://rpc.slock.it/mainnet\"";
    Ok(fs::write(&DOT_ENV_PATH, data)?)
}

pub fn delete_env_file() -> Result<()> {
    Ok(fs::remove_file(&DOT_ENV_PATH)?)
}

pub fn restore_env_file(data: String) -> Result<()> {
    Ok(fs::write(&DOT_ENV_PATH, data)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;


    #[test]
    fn should_read_existing_env_file_correctly() {
        if (Path::new(&DOT_ENV_PATH).exists()) {
            let file = read_env_file().unwrap();
            assert!(file.contains("ENDPOINT"))
        }
    }

    #[test]
    fn should_delete_env_file_correctly() {
        match (Path::new(&DOT_ENV_PATH).exists()) {
            true => {
                let original_file = read_env_file().unwrap();
                delete_env_file().unwrap();
                assert!(!Path::new(&DOT_ENV_PATH).exists());
                restore_env_file(original_file.clone()).unwrap();
                assert!(Path::new(&DOT_ENV_PATH).exists());
                let file = read_env_file().unwrap();
                assert!(file == original_file);
            },
            false => {
                write_env_file().unwrap();
                assert!(Path::new(&DOT_ENV_PATH).exists());
                delete_env_file().unwrap();
                assert!(!Path::new(&DOT_ENV_PATH).exists());
            }
        }
    }

    #[test]
    fn should_write_env_file_correctly() {
        match (Path::new(&DOT_ENV_PATH).exists()) {
            true => {
                let original_file = read_env_file().unwrap();
                delete_env_file().unwrap();
                assert!(!Path::new(&DOT_ENV_PATH).exists());
                write_env_file().unwrap();
                assert!(Path::new(&DOT_ENV_PATH).exists());
                delete_env_file().unwrap();
                restore_env_file(original_file).unwrap();
            },
            false => {
                write_env_file().unwrap();
                assert!(Path::new(&DOT_ENV_PATH).exists());
                delete_env_file().unwrap();
                assert!(!Path::new(&DOT_ENV_PATH).exists());
            }
        }
    }

    #[test]
    fn should_restore_env_file_correctly() {
        match Path::new(&DOT_ENV_PATH).exists() {
            true => {
                let mut original_file = "none".to_string();
                original_file = read_env_file().unwrap();
                delete_env_file().unwrap();
                assert!(!Path::new(&DOT_ENV_PATH).exists());
                restore_env_file(original_file.clone());
                assert!(Path::new(&DOT_ENV_PATH).exists());
                let result = read_env_file().unwrap();
                assert!(result == original_file)
            },
            false => {
                write_env_file().unwrap();
                assert!(Path::new(&DOT_ENV_PATH).exists());
                let file = read_env_file().unwrap();
                delete_env_file().unwrap();
                assert!(!Path::new(&DOT_ENV_PATH).exists());
                restore_env_file(file.clone()).unwrap();
                assert!(Path::new(&DOT_ENV_PATH).exists());
                let result = read_env_file().unwrap();
                assert!(result == file);
                delete_env_file().unwrap();
                assert!(!Path::new(&DOT_ENV_PATH).exists());
            }
        }
    }
}
