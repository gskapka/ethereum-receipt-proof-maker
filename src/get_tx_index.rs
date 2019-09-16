use crate::state::State;
use crate::types::Result;
use ethereum_types::H256;
use crate::errors::AppError;

fn get_tx_index_from_transactions(
    tx_hash: &H256,
    transactions: &Vec<H256>,
) -> Result<usize> {
    match transactions
        .iter()
        .position(|hash| tx_hash == hash) {
        Some(index) => Ok(index),
        None =>
            Err(
                AppError::Custom(
                    "✘ Cannot find transaction has in block!".to_string()
                )
            )
    }
}

pub fn get_tx_index_and_add_to_state(state: State) -> Result<State> {
    info!("✔ Getting transaction index of hash: {}", state.tx_hash);
    State::get_block_from_state(&state)
        .and_then(|block|
            get_tx_index_from_transactions(
                &state.tx_hash,
                &block.transactions,
            )
        )
        .and_then(|index| State::set_index_in_state(state, index))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        TX_INDEX,
        get_expected_block,
        get_valid_tx_hash_h256,
        get_valid_initial_state,
    };

    #[test]
    fn should_get_tx_index_from_transactions() {
        let tx_hash = get_valid_tx_hash_h256()
            .unwrap();
        let transactions = get_expected_block().transactions;
        let result = get_tx_index_from_transactions(
            &tx_hash,
            &transactions,
        ).unwrap();
        assert!(result == TX_INDEX);
    }

    #[test]
    fn should_get_tx_index_and_add_to_state_correctly() {
        let block = get_expected_block();
        let initial_state = get_valid_initial_state()
            .unwrap();
        let state_with_block = State::set_block_in_state(initial_state, block)
            .unwrap();
        let resultant_state = get_tx_index_and_add_to_state(state_with_block)
            .unwrap();
        let index_from_state = State::get_index_from_state(&resultant_state)
            .unwrap();
        assert!(index_from_state == &TX_INDEX);
    }
}
