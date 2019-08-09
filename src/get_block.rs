use crate::state::State;
use serde_json::Value as Json;
use crate::get_receipt::get_receipt_from_tx_hash;
use ethereum_types::{
    H256,
    Bloom,
};
use crate::make_rpc_call::{
    make_rpc_call,
    get_response_text,
    deserialize_to_block_rpc_response
};
use crate::get_rpc_call_jsons::{
    get_block_by_block_hash_json,
    get_block_by_block_number_json,
};
use crate::types::{
    Block,
    Result,
    BlockJson,
};
use crate::utils::{
    convert_hex_to_u256,
    convert_hex_to_h256,
    convert_hex_to_bytes,
    convert_hex_to_address,
    convert_num_string_to_usize,
    convert_num_to_prefixed_hex,
    convert_hex_strings_to_h256s,
    convert_h256_to_prefixed_hex,
};

pub fn deserialize_block_json_to_block_struct(
    block_json: BlockJson
) -> Result<Block> {
    Ok(
        Block {
            author: convert_hex_to_address(block_json.author)?,
            difficulty: convert_hex_to_u256(block_json.difficulty)?,
            extra_data: convert_hex_to_bytes(block_json.extraData)?,
            gas_limit: convert_hex_to_u256(block_json.gasLimit)?,
            gas_used: convert_hex_to_u256(block_json.gasUsed)?,
            hash: convert_hex_to_h256(block_json.hash)?,
            logs_bloom: Bloom::from_slice(
                &convert_hex_to_bytes(block_json.logsBloom)?[..]
            ),
            miner: convert_hex_to_address(block_json.miner)?,
            mix_hash: convert_hex_to_h256(block_json.mixHash)?,
            nonce: convert_hex_to_u256(block_json.nonce)?,
            number: convert_hex_to_u256(block_json.number)?,
            parent_hash: convert_hex_to_h256(block_json.parentHash)?,
            receipts_root: convert_hex_to_h256(block_json.receiptsRoot)?,
            seal_fields: (
                convert_hex_to_bytes(block_json.sealFields.0)?,
                convert_hex_to_u256(block_json.sealFields.1)?
            ),
            sha3_uncles: convert_hex_to_h256(block_json.sha3Uncles)?,
            size: convert_hex_to_u256(block_json.size)?,
            state_root: convert_hex_to_h256(block_json.stateRoot)?,
            timestamp: convert_hex_to_u256(block_json.timestamp)?,
            total_difficulty: convert_hex_to_u256(block_json.totalDifficulty)?,
            transactions: convert_hex_strings_to_h256s(block_json.transactions)?,
            transactions_root: convert_hex_to_h256(block_json.transactionsRoot)?,
            uncles: convert_hex_strings_to_h256s(block_json.uncles)?,
        }
    )
}

fn get_block(endpoint: &str, rpc_json: Json) -> Result<Block> {
    make_rpc_call(endpoint, rpc_json)
        .and_then(get_response_text)
        .and_then(deserialize_to_block_rpc_response)
        .and_then(|res| deserialize_block_json_to_block_struct(res.result))
}

pub fn get_block_by_blockhash(
    endpoint: &str,
    block_hash: H256
) -> Result<Block> {
    get_block_by_block_hash_json(convert_h256_to_prefixed_hex(block_hash)?)
        .and_then(|json| get_block(endpoint, json))
}

pub fn get_block_by_transaction_hash(
    endpoint: &str,
    tx_hash: &str
) -> Result<Block> {
    get_receipt_from_tx_hash(endpoint, tx_hash)
        .and_then(|receipt| get_block_by_blockhash(endpoint, receipt.block_hash))
}

pub fn get_block_by_number(endpoint: &str, block_num: &str) -> Result<Block> {
    let num_hex: String;
    if block_num == "latest" {
        num_hex = block_num.to_string();
    } else {
        num_hex = convert_num_to_prefixed_hex(
            convert_num_string_to_usize(block_num)?
        )?;
    }
    get_block_by_block_number_json(num_hex)
        .and_then(|json| get_block(endpoint, json))
}

fn add_block_to_state(state: State, block: Block) -> Result<State> {
    Ok(State::set_block_in_state(state, block)?)
}

pub fn get_block_from_tx_hash_in_state_and_set_in_state(
    state: State
) -> Result<State> {
    let endpoint = &State::get_endpoint_from_state(&state)?;
    let tx_hash = &convert_h256_to_prefixed_hex(state.tx_hash)?;
    get_receipt_from_tx_hash(endpoint, tx_hash)
        .and_then(|receipt| get_block_by_blockhash(endpoint, receipt.block_hash))
        .and_then(|block| add_block_to_state(state, block))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use crate::test_utils::{
        SAMPLE_TX_HASH,
        WORKING_ENDPOINT,
        SAMPLE_BLOCK_HASH,
        SAMPLE_BLOCK_JSON_PATH,
        assert_block_is_correct,
        get_valid_block_hash_h256,
        get_valid_state_with_endpoint,
    };

    #[test]
    fn should_deserialize_block_json_to_struct_correctly() {
        let block_json = fs::read_to_string(SAMPLE_BLOCK_JSON_PATH).unwrap();
        let block_rpc = deserialize_to_block_rpc_response(block_json).unwrap();
        let result = deserialize_block_json_to_block_struct(
            block_rpc.result
        ).unwrap();
        assert_block_is_correct(result)
    }

    #[test]
    fn should_get_block_by_block_hash() {
        let result = get_block_by_blockhash(
            WORKING_ENDPOINT,
            get_valid_block_hash_h256().unwrap()
        ).unwrap();
        assert_block_is_correct(result);
    }

    #[test]
    fn should_get_block_by_block_number() {
        let num_str = "8233333";
        let result = get_block_by_number(
            WORKING_ENDPOINT,
            num_str
        ).unwrap();
        assert_block_is_correct(result);
    }

    #[test]
    fn should_get_block() {
        let reqwest_json = get_block_by_block_hash_json(
            SAMPLE_BLOCK_HASH.to_string()
        ).unwrap();
        let result = get_block(
            WORKING_ENDPOINT,
            reqwest_json
        ).unwrap();
        assert_block_is_correct(result);
    }

    #[test]
    fn should_get_block_by_transaction_hash() {
        let result = get_block_by_transaction_hash(
            WORKING_ENDPOINT,
            SAMPLE_TX_HASH
        ).unwrap();
        assert_block_is_correct(result);
    }

    #[test]
    fn should_add_block_to_state() {
        let block = get_block_by_transaction_hash(
            WORKING_ENDPOINT,
            SAMPLE_TX_HASH
        ).unwrap();
        let initial_state = get_valid_state_with_endpoint().unwrap();
        let resultant_state = add_block_to_state(initial_state, block).unwrap();
        let result = State::get_block_from_state(&resultant_state).unwrap();
        assert_block_is_correct(result.clone());
    }

    #[test]
    fn should_get_block_from_tx_hash_in_state() {
        let initial_state = get_valid_state_with_endpoint().unwrap();
        let resultant_state = get_block_from_tx_hash_in_state_and_set_in_state(
            initial_state
        ).unwrap();
        let result = State::get_block_from_state(&resultant_state).unwrap();
        assert_block_is_correct(result.clone())
    }
}
