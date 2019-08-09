#![cfg(test)]
#![allow(unused_imports)]

use std::fs;
use crate::state::State;
use ethereum_types::H256;
use crate::constants::DOT_ENV_PATH;
use crate::utils::convert_hex_to_h256;
use crate::constants::DEFAULT_ENDPOINT;
use crate::get_block::deserialize_block_json_to_block_struct;
use crate::get_receipt::deserialize_receipt_json_to_receipt_struct;
use crate::make_rpc_call::{
    deserialize_to_block_rpc_response,
    deserialize_to_receipt_rpc_response,
};
use crate::types::{
    Log,
    Block,
    Result,
    Receipt,
};

pub const WORKING_ENDPOINT: &str = "https://rpc.slock.it/mainnet";

pub const SAMPLE_BLOCK_JSON_PATH: &str = "./test_utils/sample_block_json";

pub const SAMPLE_RECEIPT_JSON_PATH: &str = "./test_utils/sample_receipt_json";

pub const SAMPLE_TX_HASH: &str = "0xd6f577a93332e015438fcca4e73f538b1829acbd7eb0cf9ee5a0a73ff2752cc6";

pub const SAMPLE_BLOCK_HASH: &str = "0x1ddd540f36ea0ed23e732c1709a46c31ba047b98f1d99e623f1644154311fe10";

pub fn get_valid_tx_hash_hex() -> String {
    SAMPLE_TX_HASH.to_string()
}

pub fn get_valid_block_hash_hex() -> String {
    SAMPLE_BLOCK_HASH.to_string()
}

pub fn get_valid_block_hash_h256() -> Result<H256> {
    convert_hex_to_h256(get_valid_block_hash_hex())
}

pub fn get_valid_tx_hash_h256() -> Result<H256> {
     convert_hex_to_h256(get_valid_tx_hash_hex())
}

pub fn get_valid_initial_state() -> Result<State> {
    State::init(
        get_valid_tx_hash_h256()?,
        get_valid_tx_hash_hex(),
        true
    )
}

pub fn get_valid_state_with_endpoint() -> Result<State> {
    get_valid_initial_state()
        .and_then(|state|
            State::set_endpoint_in_state(state, WORKING_ENDPOINT.to_string())
        )
}

pub fn get_expected_block() -> Block {
    let string = fs::read_to_string(SAMPLE_BLOCK_JSON_PATH).unwrap();
    let res = deserialize_to_block_rpc_response(string).unwrap();
    deserialize_block_json_to_block_struct(res.result).unwrap()
}

pub fn get_expected_receipt() -> Receipt {
    let string = fs::read_to_string(SAMPLE_RECEIPT_JSON_PATH).unwrap();
    let res = deserialize_to_receipt_rpc_response(string).unwrap();
    deserialize_receipt_json_to_receipt_struct(res.result).unwrap()
}

pub fn get_expected_log() -> Log {
    // TODO: Implement == for logs!
    get_expected_receipt().logs[0].clone()
}

pub fn assert_log_is_correct(log: Log) {
    let sample_log = get_expected_log();
    assert!(sample_log.address == log.address);
    assert!(sample_log.topics.len() == log.topics.len());
    assert!(sample_log.data == log.data);
}

pub fn assert_block_is_correct(block: Block) {
    // TODO: Implement == for blocks!
    let sample_block = get_expected_block();
    assert!(block.author == sample_block.author);
    assert!(block.difficulty == sample_block.difficulty);
    assert!(block.extra_data == sample_block.extra_data);
    assert!(block.gas_limit == sample_block.gas_limit);
    assert!(block.gas_used == sample_block.gas_used);
    assert!(block.hash == sample_block.hash);
    assert!(block.miner == sample_block.miner);
    assert!(block.mix_hash == sample_block.mix_hash);
    assert!(block.nonce == sample_block.nonce);
    assert!(block.number == sample_block.number);
    assert!(block.parent_hash == sample_block.parent_hash);
    assert!(block.receipts_root == sample_block.receipts_root);
    assert!(block.seal_fields.0 == sample_block.seal_fields.0);
    assert!(block.seal_fields.1 == sample_block.seal_fields.1);
    assert!(block.sha3_uncles == sample_block.sha3_uncles);
    assert!(block.size == sample_block.size);
    assert!(block.state_root == sample_block.state_root);
    assert!(block.timestamp == sample_block.timestamp);
    assert!(block.total_difficulty == sample_block.total_difficulty);
    assert!(block.transactions.len() == sample_block.transactions.len());
    assert!(block.transactions_root == sample_block.transactions_root);
    assert!(block.uncles.len() == sample_block.uncles.len());
}

pub fn assert_receipt_is_correct(receipt: Receipt) {
    // TODO: Implement == for receipts
    let sample_receipt = get_expected_receipt();
    assert!(receipt.to == sample_receipt.to);
    assert!(receipt.from == sample_receipt.from);
    assert!(receipt.status == sample_receipt.status);
    assert!(receipt.block_hash == sample_receipt.block_hash);
    assert!(receipt.transaction_hash == sample_receipt.transaction_hash);
    assert!(receipt.cumulative_gas_used == sample_receipt.cumulative_gas_used);
    assert!(receipt.block_number == sample_receipt.block_number);
    assert!(receipt.transaction_index == sample_receipt.transaction_index);
    assert!(receipt.contract_address == sample_receipt.contract_address);
    assert!(receipt.root == sample_receipt.root);
    assert!(receipt.logs.len() == sample_receipt.logs.len());
}

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

mod tests {
    use hex;
    use std::fs;
    use super::*;
    use crate::state::State;
    use crate::errors::AppError;
    use crate::utils::dot_env_file_exists;
    use crate::utils::get_not_in_state_err;
    use crate::validate_tx_hash::validate_tx_hash;

    #[test]
    fn should_get_expected_block_correctly() {
        let result = get_expected_block();
        assert_block_is_correct(result);
    }

    #[test]
    fn should_get_expected_log_correctly() {
        let result = get_expected_log();
        assert_log_is_correct(result);
    }

    #[test]
    fn should_get_expected_receipt_correctly() {
        let result = get_expected_receipt();
        assert_receipt_is_correct(result);
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
