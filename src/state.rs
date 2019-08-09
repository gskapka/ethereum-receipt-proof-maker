use ethereum_types::H256;
use crate::errors::AppError;
use crate::utils::{
    get_not_in_state_err,
    get_no_overwrite_state_err,
};
use crate::types::{
    Block,
    Result,
    Receipt,
};

pub struct State {
    pub verbose: bool,
    pub tx_hash: H256,
    pub block: Option<Block>,
    pub tx_hash_string: String,
    pub receipt: Option<Receipt>,
    pub endpoint: Option<String>,
}

impl State {
    pub fn get_initial_state( // TODO: Rename to init()!
        tx_hash: H256,
        tx_hash_string: String,
        verbosity: bool
    ) -> Result<State> {
        Ok(
            State {
                tx_hash,
                block: None,
                receipt: None,
                endpoint: None,
                tx_hash_string,
                verbose: verbosity,
            }
        )
    }

    pub fn set_block_in_state(mut self, block: Block) -> Result<State> {
        match self.block {
            Some(_) =>
                Err(AppError::Custom(get_no_overwrite_state_err("block"))),
            None => {
                self.block = Some(block);
                Ok(self)
            }
        }
    }

    pub fn set_receipt_in_state(mut self, receipt: Receipt) -> Result<State> {
        match self.receipt{
            Some(_) =>
                Err(AppError::Custom(get_no_overwrite_state_err("receipt"))),
            None => {
                self.receipt = Some(receipt);
                Ok(self)
            }
        }
    }

    pub fn set_endpoint_in_state(mut self, endpoint: String) -> Result<State> {
        match self.endpoint {
            Some(_) =>
                Err(AppError::Custom(get_no_overwrite_state_err("endpoint"))),
            None => {
                self.endpoint = Some(endpoint);
                Ok(self)
            }
        }
    }

    pub fn get_block_from_state(&self) -> Result<&Block> {
        match &self.block {
            Some(block) => Ok(&block),
            None => Err(AppError::Custom(get_not_in_state_err("block")))
        }
    }

    pub fn get_endpoint_from_state(&self) -> Result<&str> {
        match &self.endpoint {
            Some(endpoint) => Ok(endpoint),
            None => Err(AppError::Custom(get_not_in_state_err("endpoint")))
        }
    }

    // TODO: Test
    pub fn get_receipt_from_state(&self) -> Result<&Receipt> {
        match &self.receipt{
            Some(receipt) => Ok(receipt),
            None => Err(AppError::Custom(get_not_in_state_err("receipt")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils::{
        get_expected_block,
        get_valid_tx_hash_h256,
        get_valid_initial_state,
        assert_block_is_correct,
    };

    #[test]
    fn should_get_initial_state_correctly() {
        let expected_verbosity = true;
        let expected_tx_hash = get_valid_tx_hash_h256()
            .unwrap();
        let state = get_valid_initial_state()
            .unwrap();
        assert!(state.tx_hash== expected_tx_hash);
        assert!(state.verbose == expected_verbosity);
    }

    #[test]
    fn initial_state_should_have_no_block() {
        let expected_err = get_not_in_state_err("block");
        let state = get_valid_initial_state()
            .unwrap();
        match State::get_block_from_state(&state) {
            Err(AppError::Custom(e)) => assert!(e == expected_err) ,
            _ => panic!("Block should not be initialised in state!"),
        }
    }

    #[test]
    fn initial_state_should_have_no_endpoint() {
        let expected_err = get_not_in_state_err("endpoint");
        let state = get_valid_initial_state()
            .unwrap();
        match State::get_endpoint_from_state(&state) {
            Err(AppError::Custom(e)) => assert!(e == expected_err),
            _ => panic!("Endpoint should not be initialised in state!"),
        }
    }

    #[test]
    fn initial_state_should_have_tx_hash_set_correctly() {
        let expected_tx_hash = get_valid_tx_hash_h256()
            .unwrap();
        let state = get_valid_initial_state()
            .unwrap();
        assert!(state.tx_hash == expected_tx_hash);
    }

    #[test]
    fn initial_state_should_have_verbosity_set_correctly() {
        let expected_verbosity = true;
        let state = get_valid_initial_state()
            .unwrap();
        assert!(state.verbose == expected_verbosity);
    }

    #[test]
    fn should_set_endpoint_to_state() {
        let expected_result = "expected endpoint".to_string();
        let state = get_valid_initial_state()
            .unwrap();
        let new_state = State::set_endpoint_in_state(state, expected_result.clone())
            .unwrap();
        let result = State::get_endpoint_from_state(&new_state)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_err_when_attempting_to_overwrite_endpoint_in_state() {
        let expected_err = "✘ Cannot overwrite endpoint in state!";
        let dummy_endpoint = "dummy endpoint".to_string();
        let initial_state = get_valid_initial_state()
            .unwrap();
        let state_with_endpoint = State::set_endpoint_in_state(
            initial_state,
            dummy_endpoint.clone()
        )
            .unwrap();
        let endpoint_from_state = State::get_endpoint_from_state(
            &state_with_endpoint
        )
            .unwrap();
        assert!(endpoint_from_state == dummy_endpoint);
        match State::set_endpoint_in_state(
            state_with_endpoint,
            dummy_endpoint.clone()
        ) {
            Err(AppError::Custom(e)) => assert!(e == expected_err),
            _ => panic!("Overwriting state should not have succeeded!"),
        }
    }

    #[test]
    fn should_set_block_in_state() {
        let expected_block = get_expected_block();
        let state = get_valid_initial_state()
            .unwrap();
        let new_state = State::set_block_in_state(state, expected_block)
            .unwrap();
        let result = State::get_block_from_state(&new_state)
            .unwrap();
        assert_block_is_correct(result.clone())
    }

    #[test]
    fn should_err_when_attempting_to_overwrite_block_in_state() {
        let expected_err = "✘ Cannot overwrite block in state!";
        let expected_block = get_expected_block();
        let state = get_valid_initial_state()
            .unwrap();
        let state_with_block = State::set_block_in_state(
            state,
            expected_block.clone()
        )
            .unwrap();
        match State::set_block_in_state(
            state_with_block,
            expected_block
        ) {
            Err(AppError::Custom(e)) => assert!(e == expected_err),
            _ => panic!("Overwriting state should not have succeeded!"),
        }
    }
}
