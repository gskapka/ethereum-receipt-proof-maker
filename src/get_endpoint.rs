use crate::constants::DEFAULT_ENDPOINT;
use crate::errors::AppError;
use crate::state::State;
use crate::types::Result;
use crate::utils::dot_env_file_exists;
use dotenv;

fn maybe_run_dot_env() -> Result<()> {
    match dot_env_file_exists() {
        true => match dotenv::dotenv() {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::Custom(format!("✘ DotEnv Error!\n{}", e))),
        },
        _ => Ok(()),
    }
}

fn get_endpoint_from_env_vars() -> Result<String> {
    maybe_run_dot_env().and_then(|_| match std::env::var("ENDPOINT") {
        Ok(endpoint) => Ok(endpoint),
        Err(_) => Ok(DEFAULT_ENDPOINT.to_string()),
    })
}

pub fn get_endpoint_and_set_in_state(state: State) -> Result<State> {
    info!("✔ Getting RPC endpoint from environment variables...");
    get_endpoint_from_env_vars().and_then(|endpoint| {
        info!("✔ Endpoint retrieved: {}", endpoint);
        State::set_endpoint_in_state(state, endpoint)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::DOT_ENV_PATH;
    use crate::test_utils::{
        delete_env_file, get_valid_initial_state, read_env_file, restore_env_file, write_env_file,
    };
    use std::fs;

    #[test]
    #[serial]
    fn maybe_run_dot_env_should_not_fail_if_no_env_file_present() {
        if dot_env_file_exists() {
            let original_file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            maybe_run_dot_env().unwrap();
            restore_env_file(original_file.clone()).unwrap();
            let file = read_env_file().unwrap();
            assert!(file == original_file)
        } else {
            maybe_run_dot_env().unwrap();
        }
    }

    #[test]
    #[serial]
    fn maybe_run_dot_env_should_not_fail_if_env_file_present() {
        if dot_env_file_exists() {
            let original_file = read_env_file().unwrap();
            maybe_run_dot_env().unwrap();
            let file = read_env_file().unwrap();
            assert!(file == original_file)
        } else {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            maybe_run_dot_env().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn maybe_run_dot_env_should_fail_if_file_malformed() {
        let expected_err = "✘ DotEnv Error!";
        if dot_env_file_exists() {
            let original_file = read_env_file().unwrap();
            let data = "ENDPOINT malformed";
            fs::write(&DOT_ENV_PATH, data).unwrap();
            assert!(dot_env_file_exists());
            let file = read_env_file().unwrap();
            assert!(data == file);
            match maybe_run_dot_env() {
                Err(AppError::Custom(e)) => {
                    assert!(e.contains(expected_err))
                }
                Err(e) => panic!(format!("Expected: {}\nBut got: {}", expected_err, e)),
                Ok(_) => panic!("Should fail w/ malformed file!"),
            }
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            restore_env_file(original_file.clone()).unwrap();
            let file = read_env_file().unwrap();
            assert!(dot_env_file_exists());
            assert!(original_file == file);
        } else {
            let data = "ENDPOINT malformed";
            fs::write(&DOT_ENV_PATH, data).unwrap();
            assert!(dot_env_file_exists());
            let file = read_env_file().unwrap();
            assert!(data == file);
            match maybe_run_dot_env() {
                Err(AppError::Custom(e)) => {
                    assert!(e.contains(expected_err))
                }
                Err(e) => panic!(format!("Expected: {}\nBut got: {}", expected_err, e)),
                Ok(_) => panic!("Should fail w/ malformed file!"),
            }
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_get_default_endpoint_correctly_if_no_env_file_exists() {
        if !dot_env_file_exists() {
            let result = get_endpoint_from_env_vars().unwrap();
            assert!(result == DEFAULT_ENDPOINT);
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_get_endpoint_from_env_file_if_extant_correctly() {
        if dot_env_file_exists() {
            let file = read_env_file().unwrap();
            let result = get_endpoint_from_env_vars().unwrap();
            assert!(file.contains(&result));
        }
    }

    #[test]
    #[serial]
    fn should_get_default_endpoint_and_set_in_state_if_no_env_file() {
        if !dot_env_file_exists() {
            let expected_err = "No endpoint in state";
            let initial_state = get_valid_initial_state().unwrap();
            match State::get_endpoint_from_state(&initial_state) {
                Err(AppError::Custom(e)) => assert!(e.contains(expected_err)),
                _ => panic!("State should not have endpoint yet!"),
            }
            let result_state = get_endpoint_and_set_in_state(initial_state).unwrap();
            match State::get_endpoint_from_state(&result_state) {
                Ok(endpoint) => assert!(endpoint == DEFAULT_ENDPOINT),
                _ => panic!("Default endpoint should be set in state!"),
            }
        }
    }

    #[test]
    #[serial]
    fn should_get_custom_endpoint_and_set_in_state_if_env_file() {
        if dot_env_file_exists() {
            let expected_err = "No endpoint in state";
            let initial_state = get_valid_initial_state().unwrap();
            match State::get_endpoint_from_state(&initial_state) {
                Err(AppError::Custom(e)) => assert!(e.contains(expected_err)),
                _ => panic!("State should not have endpoint yet!"),
            }
            let file = read_env_file().unwrap();
            let result_state = get_endpoint_and_set_in_state(initial_state).unwrap();
            match State::get_endpoint_from_state(&result_state) {
                Ok(endpoint) => assert!(file.contains(&endpoint)),
                _ => panic!("Custom endpoint should be set in state!"),
            }
        }
    }
}
