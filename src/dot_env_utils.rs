use std::fs;
use std::path::Path;
use crate::types::Result;
use crate::constants::{
    DOT_ENV_PATH,
    DEFAULT_ENDPOINT
};


pub fn read_env_file() -> Result<String> {
    Ok(fs::read_to_string(&DOT_ENV_PATH)?)
}

pub fn write_env_file(endpoint_url: Option<&str>) -> Result<()> {
    let url = endpoint_url.unwrap_or(DEFAULT_ENDPOINT);
    let data = format!("ENDPOINT=\"{}\"", url);
    Ok(fs::write(&DOT_ENV_PATH, data)?)
}

pub fn delete_env_file() -> Result<()> {
    Ok(fs::remove_file(&DOT_ENV_PATH)?)
}

pub fn restore_env_file(data: String) -> Result<()> {
    Ok(fs::write(&DOT_ENV_PATH, data)?)
}

pub fn dot_env_file_exists() -> bool {
    Path::new(&DOT_ENV_PATH).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn should_return_true_if_dot_env_file_exists() {
        if Path::new(&DOT_ENV_PATH).exists() {
            assert!(dot_env_file_exists());
        } else {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_return_false_if_dot_env_file_does_not_exist() {
        if Path::new(&DOT_ENV_PATH).exists() {
            let file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            restore_env_file(file.clone()).unwrap();
            assert!(dot_env_file_exists());
            let result = read_env_file().unwrap();
            assert!(result == file);
        } else {
            assert!(!dot_env_file_exists())
        }
    }

    #[test]
    #[serial]
    fn should_delete_env_file_correctly_if_it_exists() {
        if dot_env_file_exists() {
            let original_file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            restore_env_file(original_file.clone()).unwrap();
            assert!(dot_env_file_exists());
            let file = read_env_file().unwrap();
            assert!(file == original_file);
        } else {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_read_existing_env_file_correctly() {
        if dot_env_file_exists() {
            let file = read_env_file().unwrap();
            assert!(file.contains("ENDPOINT"))
        }
    }


    #[test]
    #[serial]
    fn should_delete_env_file_correctly_if_it_does_not_exist() {
        if !dot_env_file_exists() {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_write_env_file_correctly_if_it_exists() {
        if dot_env_file_exists() {
            let original_file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            restore_env_file(original_file.clone()).unwrap();
            let file = read_env_file().unwrap();
            assert!(file == original_file)
        }
    }

    #[test]
    #[serial]
    fn should_write_env_file_correctly_if_it_does_not_exist() {
        if !dot_env_file_exists() {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_restore_env_file_correctly_if_it_exists() {
        if dot_env_file_exists() {
            let original_file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            restore_env_file(original_file.clone()).unwrap();
            assert!(dot_env_file_exists());
            let result = read_env_file().unwrap();
            assert!(result == original_file)
        }
    }

    #[test]
    #[serial]
    fn should_restore_env_file_correctly_if_it_does_not_exist() {
        if !dot_env_file_exists() {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            let file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            restore_env_file(file.clone()).unwrap();
            assert!(dot_env_file_exists());
            let result = read_env_file().unwrap();
            assert!(result == file);
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }
}
