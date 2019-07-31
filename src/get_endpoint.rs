use crate::constants::{
    DOT_ENV_PATH,
    DEFAULT_ENDPOINT,
};

use crate::dot_env_utils::{
    read_env_file,
    write_env_file,
    delete_env_file,
    restore_env_file,
    dot_env_file_exists,
};

use dotenv;
use std::result;
use std::path::Path;
use crate::state::State;
use crate::errors::AppError;

type Result<T> = result::Result<T, AppError>;

fn maybe_run_dot_env() -> Result<()> {
    match dot_env_file_exists() {
        true => {
            match dotenv::dotenv() {
                Ok(_) => Ok(()),
                Err(e) => Err(
                    AppError::Custom(
                        format!("✘ DotEnv Error!\n{}", e)
                    )
                )
            }
        },
        _ => Ok(())
    }
}

fn get_endpoint_from_env_vars() -> Result<String> {
    maybe_run_dot_env()
        .and_then(|_|
            match std::env::var("ENDPOINT") {
                Ok(endpoint) => Ok(endpoint),
                Err(_) => Ok(DEFAULT_ENDPOINT.to_string())
            }
    )
}

pub fn get_endpoint_and_set_in_state(state: State) -> Result<State> {
    get_endpoint_from_env_vars()
        .and_then(|endpoint| State::set_endpoint_in_state(state, endpoint))
}
