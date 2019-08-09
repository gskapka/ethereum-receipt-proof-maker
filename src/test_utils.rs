use std::fs;
use crate::state::State;
use ethereum_types::H256;
use crate::utils::convert_hex_to_h256;
use crate::make_rpc_call::deserialize_to_block_rpc_response;
use crate::get_block::deserialize_block_json_to_block_struct;
use crate::types::{
    Result,
    Block
};

#[cfg(test)]
pub const SAMPLE_BLOCK_JSON_PATH: &str = "./block_json_sample";

#[cfg(test)]
pub const WORKING_ENDPOINT: &str = "https://rpc.slock.it/mainnet";

#[cfg(test)]
pub const SAMPLE_TX_HASH: &str = "0xd6f577a93332e015438fcca4e73f538b1829acbd7eb0cf9ee5a0a73ff2752cc6";

#[cfg(test)]
pub const SAMPLE_BLOCK_HASH: &str = "0x1ddd540f36ea0ed23e732c1709a46c31ba047b98f1d99e623f1644154311fe10";

#[cfg(test)]
pub fn get_valid_tx_hash_hex() -> String {
    SAMPLE_TX_HASH.to_string()
}

#[cfg(test)]
pub fn get_valid_block_hash_hex() -> String {
    SAMPLE_BLOCK_HASH.to_string()
}

#[cfg(test)]
pub fn get_valid_block_hash_h256() -> Result<H256> {
    convert_hex_to_h256(get_valid_block_hash_hex())
}

#[cfg(test)]
pub fn get_valid_tx_hash_h256() -> Result<H256> {
     convert_hex_to_h256(get_valid_tx_hash_hex())
}

#[cfg(test)]
pub fn get_valid_initial_state() -> Result<State> {
    State::get_initial_state(
        get_valid_tx_hash_h256()?,
        get_valid_tx_hash_hex(),
        true
    )
}

#[cfg(test)]
pub fn get_valid_state_with_endpoint() -> Result<State> {
    get_valid_initial_state()
        .and_then(|state|
            State::set_endpoint_in_state(state, WORKING_ENDPOINT.to_string())
        )
}

#[cfg(test)]
pub fn get_expected_block() -> Block {
    let string = fs::read_to_string(SAMPLE_BLOCK_JSON_PATH).unwrap();
    let res = deserialize_to_block_rpc_response(string).unwrap();
    deserialize_block_json_to_block_struct(res.result).unwrap()
}

#[cfg(test)]
pub fn assert_block_is_correct(block: Block) {
    // TODO: Either implement == for blocks OR add more assertions here!
    let sample_block = get_expected_block();
    assert!(block.number == sample_block.number);
    assert!(block.gas_used == sample_block.gas_used);
    assert!(block.difficulty == sample_block.difficulty);
    assert!(block.transactions_root == sample_block.transactions_root);
    assert!(block.transactions.len() == sample_block.transactions.len());
}

#[cfg(test)]
mod tests {
    use hex;
    use super::*;
    use crate::state::State;
    use crate::errors::AppError;
    use crate::utils::get_not_in_state_err;
    use crate::validate_tx_hash::validate_tx_hash;

    #[test]
    fn should_get_expected_block_correctly() {
        let result = get_expected_block();
        assert_block_is_correct(result);
    }

    #[test]
    fn should_get_valid_tx_hash_as_hex() {
        let result = get_valid_tx_hash_hex();
        match validate_tx_hash(result) {
            Ok(_) => assert!(true),
            Err(_) => panic!("Hex tx hash should be valid!")
        }
    }

    #[test]
    fn should_get_valid_block_hash_as_hex() {
        let result = get_valid_block_hash_hex();
        match validate_tx_hash(result) {
            Ok(_) => assert!(true),
            Err(_) => panic!("Hex block hash should be valid!")
        }
    }

    #[test]
    fn should_get_valid_tx_hash_as_h256() {
        let result = get_valid_tx_hash_h256()
            .unwrap();
        let result_bytes = result.as_bytes();
        let result_hex = format!("0x{}", hex::encode(result_bytes));
        let expected_result = get_valid_tx_hash_hex();
        assert!(result_hex == expected_result)
    }

    #[test]
    fn should_get_valid_block_hash_as_h256() {
        let result = get_valid_block_hash_h256()
            .unwrap();
        let result_bytes = result.as_bytes();
        let result_hex = format!("0x{}", hex::encode(result_bytes));
        let expected_result = get_valid_block_hash_hex();
        assert!(result_hex == expected_result)
    }

    #[test]
    fn should_get_valid_intial_state_correctly() {
        let expected_verbosity = true;
        let expected_tx_hash = get_valid_tx_hash_h256()
            .unwrap();
        let result = get_valid_initial_state()
            .unwrap();
        assert!(result.tx_hash == expected_tx_hash);
        assert!(result.verbose == expected_verbosity);
        match State::get_endpoint_from_state(&result) {
            Err(AppError::Custom(e)) =>
                assert!(e == get_not_in_state_err("endpoint")),
            _ => panic!("Intial state should not have endpoint set!")
        }
        match State::get_block_from_state(&result) {
            Err(AppError::Custom(e)) =>
                assert!(e == get_not_in_state_err("block")),
            _ => panic!("Intial state should not have endpoint set!")
        }
    }

    #[test]
    fn should_get_valid_state_with_endpoint_correctly() {
        let expected_verbosity = true;
        let expected_endpoint = WORKING_ENDPOINT;
        let expected_tx_hash = get_valid_tx_hash_h256()
            .unwrap();
        let result = get_valid_state_with_endpoint()
            .unwrap();
        assert!(result.tx_hash == expected_tx_hash);
        assert!(result.verbose == expected_verbosity);
        match State::get_endpoint_from_state(&result) {
            Ok(endpoint) => assert!(endpoint == expected_endpoint),
            _ => panic!("Intial w/ endpoint should have endpoint set!")
        }
        match State::get_block_from_state(&result) {
            Err(AppError::Custom(e)) =>
                assert!(e == get_not_in_state_err("block")),
            _ => panic!("Intial state should not have endpoint set!")
        }
    }
}
