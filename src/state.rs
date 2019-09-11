use crate::trie::Trie;
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
    Database,
};

pub struct State {
    pub verbose: bool,
    pub tx_hash: H256,
    pub database: Database,
    pub block: Option<Block>,
    pub index: Option<usize>,
    pub tx_hash_string: String,
    pub endpoint: Option<String>,
    pub receipts_trie: Option<Trie>,
    pub receipts: Option<Vec<Receipt>>,
}

impl State {
    pub fn init(
        tx_hash: H256,
        tx_hash_string: String,
        verbosity: bool
    ) -> Result<State> {
        Ok(
            State {
                tx_hash,
                block: None,
                index: None,
                endpoint: None,
                receipts: None,
                tx_hash_string,
                verbose: verbosity,
                receipts_trie: None,
                database: std::collections::HashMap::new(),
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

    pub fn set_index_in_state(mut self, index: usize) -> Result<State> {
        match self.index {
            Some(_) =>
                Err(AppError::Custom(get_no_overwrite_state_err("index"))),
            None => {
                self.index = Some(index);
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

    pub fn set_receipts_in_state(mut self, receipts: Vec<Receipt>) -> Result<State> {
        match self.receipts {
            Some(_) =>
                Err(AppError::Custom(get_no_overwrite_state_err("receipts"))),
            None => {
                self.receipts= Some(receipts);
                Ok(self)
            }
        }
    }

    pub fn set_receipts_trie_in_state(mut self, receipts_trie: Trie) -> Result<State> {
        match self.receipts_trie {
            Some(_) =>
                Err(AppError::Custom(get_no_overwrite_state_err("receipts_trie"))),
            None => {
                self.receipts_trie = Some(receipts_trie);
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

    pub fn get_receipts_from_state(&self) -> Result<&Vec<Receipt>> {
        match &self.receipts {
            Some(receipts) => Ok(receipts),
            None => Err(AppError::Custom(get_not_in_state_err("receipts")))
        }
    }

    pub fn get_index_from_state(&self) -> Result<&usize> {
        match &self.index {
            Some(index) => Ok(index),
            None => Err(AppError::Custom(get_not_in_state_err("index")))
        }
    }

    pub fn get_receipts_trie_from_state(&self) -> Result<&Trie> {
        match &self.receipts_trie{
            Some(receipts_trie) => Ok(receipts_trie),
            None => Err(AppError::Custom(get_not_in_state_err("receipts_trie")))
        }
    }
}

pub fn update_database_in_state(
    mut state: State,
    updated_database: Database
) -> Result<State> {
    state.database = updated_database;
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_database::get_thing_from_database;
    use crate::test_utils::{
        get_expected_block,
        get_expected_receipt,
        get_valid_tx_hash_h256,
        get_valid_initial_state,
        assert_block_is_correct,
        assert_receipt_is_correct,
        get_thing_to_put_in_database,
        get_database_with_thing_in_it,
        get_expected_key_of_thing_in_database,
    };

    #[test]
    fn should_get_initial_state_correctly() {
        let expected_verbosity = true;
        let expected_tx_hash = get_valid_tx_hash_h256()
            .unwrap();
        let state = get_valid_initial_state()
            .unwrap();
        assert!(state.tx_hash == expected_tx_hash);
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
    fn initial_state_should_have_no_receipts_trie() {
        let expected_err = get_not_in_state_err("receipts_trie");
        let state = get_valid_initial_state()
            .unwrap();
        match State::get_receipts_trie_from_state(&state) {
            Err(AppError::Custom(e)) => assert!(e == expected_err),
            _ => panic!("Receipts trie should not be initialised in state!"),
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
    fn should_set_receipts_trie_to_state() {
        let trie = Trie::get_new_trie().unwrap();
        let expected_root = trie.root;
        let state = get_valid_initial_state()
            .unwrap();
        let new_state = State::set_receipts_trie_in_state(state, trie)
            .unwrap();
        let result = State::get_receipts_trie_from_state(&new_state)
            .unwrap();
        assert!(result.root == expected_root);
    }

    #[test]
    fn should_err_when_attempting_to_overwrite_receipts_trie_in_state() {
        let expected_err = "✘ Cannot overwrite receipts_trie in state!";
        let trie = Trie::get_new_trie().unwrap();
        let expected_root = trie.root;
        let initial_state = get_valid_initial_state()
            .unwrap();
        let state_with_trie = State::set_receipts_trie_in_state(
            initial_state,
            trie.clone()
        ).unwrap();
        let trie_from_state = State::get_receipts_trie_from_state(
            &state_with_trie
        ).unwrap();
        assert!(trie_from_state.root == expected_root);
        match State::set_receipts_trie_in_state(
            state_with_trie,
            trie.clone()
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

   #[test]
   fn should_set_receipts_into_state() {
       let receipt = get_expected_receipt();
       let mut vec_of_receipts = Vec::new();
       vec_of_receipts.push(receipt.clone());
       vec_of_receipts.push(receipt);
       let state = get_valid_initial_state()
           .unwrap();
       let state_with_receipts = State::set_receipts_in_state(
           state,
           vec_of_receipts
       ).unwrap();
       let receipts_from_state = State::get_receipts_from_state(
           &state_with_receipts
       ).unwrap();
       let _result: Vec<_> = receipts_from_state
            .iter()
            .map(|receipt| assert_receipt_is_correct(receipt.clone()))
            .collect();
   }

   #[test]
   fn should_err_when_attempting_to_overwrite_receipts_in_state() {
       let expected_err = "✘ Cannot overwrite receipts in state!";
       let receipt = get_expected_receipt();
       let mut vec_of_receipts = Vec::new();
       vec_of_receipts.push(receipt.clone());
       vec_of_receipts.push(receipt);
       let state = get_valid_initial_state()
           .unwrap();
       let state_with_receipts = State::set_receipts_in_state(
           state,
           vec_of_receipts.clone(),
       ).unwrap();
       match State::set_receipts_in_state(
           state_with_receipts,
           vec_of_receipts,
       ) {
           Err(AppError::Custom(e)) => assert!(e == expected_err),
           _ => panic!("Expected error not received!")
       }
   }

    #[test]
    fn should_set_index_to_state() {
        let expected_index: usize = 1337;
        let state = get_valid_initial_state()
            .unwrap();
        let new_state = State::set_index_in_state(state, expected_index)
            .unwrap();
        let result = State::get_index_from_state(&new_state)
            .unwrap();
        assert!(result == &expected_index);
    }

    #[test]
    fn should_err_when_attempting_to_overwrite_index_in_state() {
        let expected_index: usize = 1337;
        let expected_err = "✘ Cannot overwrite index in state!";
        let initial_state = get_valid_initial_state()
            .unwrap();
        let state_with_index = State::set_index_in_state(
            initial_state,
            expected_index,
        ).unwrap();
        let index_from_state = State::get_index_from_state(
            &state_with_index
        ).unwrap();
        assert!(index_from_state == &expected_index);
        match State::set_index_in_state(
            state_with_index,
            expected_index.clone()
        ) {
            Err(AppError::Custom(e)) => assert!(e == expected_err),
            _ => panic!("Overwriting state should not have succeeded!"),
        }
    }

    #[test]
    fn should_update_database_in_state() {
        let expected_thing = get_thing_to_put_in_database();
        let expected_key = get_expected_key_of_thing_in_database();
        let state = get_valid_initial_state()
            .unwrap();
        match get_thing_from_database(&state.database, &expected_key) {
            Some(_) => panic!("Thing should not be in database!"),
            None => assert!(true)
        }
        let database_with_thing_in_it = get_database_with_thing_in_it()
            .unwrap();
        let updated_state = update_database_in_state(
            state,
            database_with_thing_in_it
        ).unwrap();
        match get_thing_from_database(&updated_state.database, &expected_key) {
            Some(thing) => assert!(thing == expected_thing),
            None => panic!("Thing should be in database!")
        }
    }
}
