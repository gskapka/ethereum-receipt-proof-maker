use crate::types::Block;
use ethereum_types::H256;
use crate::types::Result;
use crate::errors::AppError;

#[derive(Clone)]
pub struct State {
    block: Option<Block>,
    tx_hash: Option<H256>,
    endpoint: Option<String>,
}

fn get_not_in_state_err(substring: &str) -> String {
    format!("✘ No {} in state!" , substring)
}

fn get_no_overwrite_state_err(substring: &str) -> String {
    format!("✘ Cannot overwrite {} in state!" , substring)
}

impl State {
    pub fn get_initial_state() -> Result<State> {
        Ok(
            State {
                block: None,
                tx_hash: None,
                endpoint: None,
            }
        )
    }

    pub fn set_block_in_state(mut self, block: Block) -> Result<State> {
        match self.block {
            Some(_) => Err(AppError::Custom(get_no_overwrite_state_err("block"))),
            None => {
                self.block = Some(block);
                Ok(self)
            }
        }
    }

    pub fn set_endpoint_in_state(mut self, endpoint: String) -> Result<State> {
        match self.endpoint {
            Some(_) => Err(AppError::Custom(get_no_overwrite_state_err("endpoint"))),
            None => {
                self.endpoint = Some(endpoint);
                Ok(self)
            }
        }
    }

    pub fn set_tx_hash_in_state(mut self, tx_hash: H256)-> Result<State> {
        match self.tx_hash{
            Some(_) => Err(AppError::Custom(get_no_overwrite_state_err("transaction hash"))),
            None => {
                self.tx_hash= Some(tx_hash);
                Ok(self)
            }
        }
    }

    pub fn get_block_from_state(self) -> Result<Block> { // TODO: Should these return a tuple of state + thing?
        match self.block {
            Some(block) => Ok(block),
            None => Err(AppError::Custom(get_not_in_state_err("block")))
        }
    }

    pub fn get_endpoint_from_state(self) -> Result<String> { // TODO: Ibid. So as not to "consume" the state?
        match self.endpoint {
            Some(endpoint) => Ok(endpoint),
            None => Err(AppError::Custom(get_not_in_state_err("endpoint")))
        }
    }

    pub fn get_tx_hash_from_state(self) -> Result<H256> {
        match self.tx_hash {
            Some(tx_hash) => Ok(tx_hash),
            None => Err(AppError::Custom(get_not_in_state_err("transaction hash")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_no_state_err_string() {
        let thing = "thing".to_string();
        let expected_result = "✘ No thing in state!";
        let result = get_not_in_state_err(&thing);
        assert!(result == expected_result)

    }

    #[test]
    fn should_get_no_overwrite_err_string() {
        let thing = "thing".to_string();
        let expected_result = "✘ Cannot overwrite thing in state!";
        let result = get_no_overwrite_state_err(&thing);
        assert!(result == expected_result)

    }

    #[test]
    fn should_get_empty_state_successfully() {
        State::get_initial_state()
            .unwrap();
    }

    #[test]
    fn empty_state_should_have_no_block() {
        let expected_err = get_not_in_state_err("block");
        let state = State::get_initial_state()
            .unwrap();
        match State::get_block_from_state(state) {
            Err(AppError::Custom(e)) => assert!(e == expected_err) ,
            _ => panic!("Block should not be initialised in state!"),
        }
    }

    #[test]
    fn empty_state_should_have_no_endpoint() {
        let expected_err = get_not_in_state_err("endpoint");
        let state = State::get_initial_state()
            .unwrap();
        match State::get_endpoint_from_state(state) {
            Err(AppError::Custom(e)) => assert!(e == expected_err),
            _ => panic!("Endpoint should not be initialised in state!"),
        }
    }

    #[test]
    fn empty_state_should_have_no_tx_hash() {
        let expected_err = get_not_in_state_err("transaction hash");
        let state = State::get_initial_state()
            .unwrap();
        match State::get_tx_hash_from_state(state) {
            Err(AppError::Custom(e)) => assert!(e == expected_err),
            _ => panic!("Endpoint should not be initialised in state!"),
        }
    }

    /*
    #[test]
    fn should_add_block_to_state() { // TODO: Implement! (Need an empty block getter! | sample one! | default block struct)
        let expected_result = "expected endpoint".to_string();
        let state = State::get_initial_state()
            .unwrap();
        let new_state = State::set_endpoint_in_state(state, expected_result.clone())
            .unwrap();
        let result = State::get_endpoint_from_state(new_state)
            .unwrap();
        assert!(result == expected_result);
    }
    */

    #[test]
    fn should_set_endpoint_to_state() {
        let expected_result = "expected endpoint".to_string();
        let state = State::get_initial_state()
            .unwrap();
        let new_state = State::set_endpoint_in_state(state, expected_result.clone())
            .unwrap();
        let result = State::get_endpoint_from_state(new_state)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_err_when_attempting_to_overwrite_endpoint_in_state() {
        let expected_err = "✘ Cannot overwrite endpoint in state!";
        let dummy_endpoint = "dummy endpoint".to_string();
        let initial_state = State::get_initial_state()
            .unwrap();
        let state_with_endpoint = State::set_endpoint_in_state(
            initial_state,
            dummy_endpoint.clone()
        )
            .unwrap();
        let endpoint_from_state = State::get_endpoint_from_state(
            state_with_endpoint.clone()
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
    fn should_set_tx_hash_to_state() {
        let dummy_tx_hash = H256::zero();
        let state = State::get_initial_state()
            .unwrap();
        let new_state = State::set_tx_hash_in_state(state, dummy_tx_hash.clone())
            .unwrap();
        let result = State::get_tx_hash_from_state(new_state)
            .unwrap();
        assert!(result == dummy_tx_hash);
    }

    #[test]
    fn should_err_when_attempting_to_overwrite_tx_hash_in_state() {
        let expected_err = "✘ Cannot overwrite transaction hash in state!";
        let dummy_tx_hash = H256::zero();
        let initial_state = State::get_initial_state()
            .unwrap();
        let state_with_tx_hash = State::set_tx_hash_in_state(
            initial_state,
            dummy_tx_hash.clone()
        )
            .unwrap();

        let tx_hash_from_state = State::get_tx_hash_from_state(
            state_with_tx_hash.clone()
        )
            .unwrap();
        assert!(tx_hash_from_state == dummy_tx_hash);
        match State::set_tx_hash_in_state(
            state_with_tx_hash,
            dummy_tx_hash.clone()
        ) {
            Err(AppError::Custom(e)) => assert!(e == expected_err),
            _ => panic!("Overwriting state should not have succeeded!"),
        }
    }
}
