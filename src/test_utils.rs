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

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;

    fn restore_original_config_file(data: String) -> Result<()> {
        Ok(fs::write(&DOT_ENV_PATH, data)?)
    }

    #[test]
    fn should_read_existing_env_file_correctly() {
        if (Path::new(&DOT_ENV_PATH).exists()) {
            let file = read_env_file().unwrap();
            assert!(file.contains("ENDPOINT"))
        }
    }

    #[test]
    fn should_delete_env_file_correctly() {
        let mut original_file: String = "none".to_string();
        if (Path::new(&DOT_ENV_PATH).exists()) {
            original_file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!Path::new(&DOT_ENV_PATH).exists());
            restore_original_config_file(original_file).unwrap();
            assert!(Path::new(&DOT_ENV_PATH).exists());
        } else {
            write_env_file().unwrap();
            assert!(Path::new(&DOT_ENV_PATH).exists());
            delete_env_file().unwrap();
            assert!(!Path::new(&DOT_ENV_PATH).exists());
        }
    }

    #[test]
    fn should_write_env_file_correctly() {
        let mut original_file: String = "none".to_string();
        if (Path::new(&DOT_ENV_PATH).exists()) {
            original_file = read_env_file().unwrap();
            delete_env_file().unwrap();
        }
        assert!(!Path::new(&DOT_ENV_PATH).exists());
        write_env_file().unwrap();
        assert!(Path::new(&DOT_ENV_PATH).exists());
        delete_env_file().unwrap();
        restore_original_config_file(original_file).unwrap();
    }
}
