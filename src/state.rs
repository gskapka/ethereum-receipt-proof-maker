use std::result;
use crate::types::Block;
use crate::errors::AppError;

type Result<T> = result::Result<T, AppError>;

pub struct State {
    block: Option<Block>,
    endpoint: Option<String>,
}

fn get_state_err_str(substring: &str) -> String {
    format!("✘ No {} in state!" , substring)
}

impl State {
    pub fn get_initial_state() -> Result<State> {
        Ok(
            State {
                block: None,
                endpoint: None,
            }
        )
    }

    pub fn add_block_to_state(mut self, block: Block) -> Result<State> {
        self.block = Some(block);
        Ok(self)
    }

    pub fn add_endpoint_to_state(mut self, endpoint: String) -> Result<State> {
        self.endpoint= Some(endpoint);
        Ok(self)
    }

    pub fn get_block_from_state(self) -> Result<Block> {
        match self.block {
            Some(block) => Ok(block),
            _ => Err(AppError::Custom(get_state_err_str("block")))
        }
    }

    pub fn get_endpoint_from_state(self) -> Result<String> {
        match self.endpoint {
            Some(endpoint) => Ok(endpoint),
            _ => Err(AppError::Custom(get_state_err_str("endpoint")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_err_string() {
        let thing = "thing".to_string();
        let expected_result = "✘ No thing in state!";
        let result = get_state_err_str(&thing);
        assert!(result == expected_result)

    }
    #[test]
    fn should_get_empty_state_successfully() {
        let state = State::get_initial_state().unwrap();
    }

    #[test]
    fn empty_state_should_have_no_block() {
        let state = State::get_initial_state().unwrap();
        match State::get_block_from_state(state) {
            Err(AppError::Custom(e)) => {
                let expectedErr = get_state_err_str("block");
                assert!(e == expectedErr);
            },
            Ok(_) => panic!("Block should not be initialised in state!"),
            Err(e) => panic!("Wrong error type received!")
        }
    }

    #[test]
    fn empty_state_should_have_no_endpoint() {
        let state = State::get_initial_state().unwrap();
        match State::get_endpoint_from_state(state) {
            Err(AppError::Custom(e)) => {
                let expectedErr = get_state_err_str("endpoint");
                assert!(e == expectedErr);
            },
            Ok(_) => panic!("Endpoint should not be initialised in state!"),
            Err(e) => panic!("Wrong error type received!")
        }
    }

    /*
    #[test]
    fn should_add_block_to_state() { // TODO: Implement! (Need an empty block getter! | sample one!)
        let expected_result = "expected endpoint".to_string();
        let state = State::get_initial_state().unwrap();
        let new_state = State::add_endpoint_to_state(state, expected_result.clone())
            .unwrap();
        let result = State::get_endpoint_from_state(new_state).unwrap();
        assert!(result == expected_result);
    }
    */

    #[test]
    fn should_add_endpoint_to_state() {
        let expected_result = "expected endpoint".to_string();
        let state = State::get_initial_state().unwrap();
        let new_state = State::add_endpoint_to_state(state, expected_result.clone())
            .unwrap();
        let result = State::get_endpoint_from_state(new_state).unwrap();
        assert!(result == expected_result);
    }
}
