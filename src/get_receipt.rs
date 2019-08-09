use crate::state::State;
use crate::get_rpc_call_jsons::get_transaction_receipt_json;
use crate::make_rpc_call::{
    make_rpc_call,
    get_response_text,
    deserialize_to_receipt_rpc_response,
};
use ethereum_types::{
    H256,
    Address,
};
use crate::utils::{
    convert_hex_to_u256,
    convert_hex_to_h256,
    convert_hex_to_address,
    convert_h256_to_prefixed_hex,
};
use crate::types::{
    Result,
    Receipt,
    ReceiptJson,
};
use crate::get_log::{
    get_logs_bloom_from_logs,
    get_logs_from_receipt_json,
};

pub fn deserialize_receipt_json_to_receipt_struct(
    receipt: ReceiptJson
) -> Result<Receipt> {
    let logs = get_logs_from_receipt_json(&receipt)?;
    Ok(
        Receipt {
            to: convert_hex_to_address(receipt.to)?,
            from: convert_hex_to_address(receipt.from)?,
            logs_bloom: get_logs_bloom_from_logs(&logs)?,
            gas_used: convert_hex_to_u256(receipt.gasUsed)?,
            block_hash: convert_hex_to_h256(receipt.blockHash)?,
            block_number: convert_hex_to_u256(receipt.blockNumber)?,
            transaction_hash: convert_hex_to_h256(receipt.transactionHash)?,
            transaction_index: convert_hex_to_u256(receipt.transactionIndex)?,
            cumulative_gas_used: convert_hex_to_u256(receipt.cumulativeGasUsed)?,
            status: match receipt.status.as_ref() {
                "0x1" => true,
                "0x0" => false,
                    _ => false
            },
            root: match receipt.contractAddress {
                serde_json::Value::Null => H256::zero(),
                _ => convert_hex_to_h256(receipt.root.to_string())?,
            },
            contract_address: match receipt.contractAddress {
                serde_json::Value::Null => Address::zero(),
                _ => convert_hex_to_address(receipt.contractAddress.to_string())?,
            },
            logs,
        }
    )
}

pub fn get_receipt_from_tx_hash(
    endpoint: &str,
    tx_hash: &str
) -> Result<Receipt> {
    get_transaction_receipt_json(&tx_hash)
        .and_then(|rpc_json| make_rpc_call(endpoint, rpc_json))
        .and_then(get_response_text)
        .and_then(deserialize_to_receipt_rpc_response)
        .and_then(|res| deserialize_receipt_json_to_receipt_struct(res.result))
}

fn get_receipts_from_tx_hashes(
    endpoint: &str,
    tx_hashes: &Vec<H256>,
) -> Result<Vec<Receipt>> {
    tx_hashes
        .iter()
        .map(|tx_hash|
             get_receipt_from_tx_hash(
                 endpoint,
                 &convert_h256_to_prefixed_hex(*tx_hash)?
             )
         )
        .collect::<Result<Vec<Receipt>>>()
}

pub fn get_all_receipts_from_block_in_state_and_set_in_state(
    state: State
) -> Result<State> {
    get_receipts_from_tx_hashes(
        &State::get_endpoint_from_state(&state)?,
        &State::get_block_from_state(&state)?.transactions,
    )
        .and_then(|receipts| State::set_receipts_in_state(state, receipts))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use crate::make_rpc_call::deserialize_to_receipt_rpc_response;
    use crate::test_utils::{
        TX_INDEX,
        SAMPLE_TX_HASH,
        WORKING_ENDPOINT,
        get_expected_block,
        get_valid_tx_hash_h256,
        SAMPLE_RECEIPT_JSON_PATH,
        assert_receipt_is_correct,
        get_valid_state_with_endpoint,
    };

    #[test]
    fn should_get_receipt_from_tx_hash() {
        let result = get_receipt_from_tx_hash(
            WORKING_ENDPOINT,
            SAMPLE_TX_HASH,
        ).unwrap();
        assert_receipt_is_correct(result)
    }

    #[test]
    fn should_deserialize_receipt_json_to_receipt_struct_correctly() {
        let receipt_string = fs::read_to_string(SAMPLE_RECEIPT_JSON_PATH)
            .unwrap();
        let receipt_json = deserialize_to_receipt_rpc_response(receipt_string)
            .unwrap();
        let result = deserialize_receipt_json_to_receipt_struct(
            receipt_json.result
        ).unwrap();
        assert_receipt_is_correct(result)
    }

    #[test]
    fn should_get_receipts_from_tx_hashes_correctly() {
        let tx_hash_h256 = get_valid_tx_hash_h256()
            .unwrap();
        let mut tx_hashes = Vec::new();
        tx_hashes.push(tx_hash_h256);
        tx_hashes.push(tx_hash_h256);
        let result = get_receipts_from_tx_hashes(
            WORKING_ENDPOINT,
            &tx_hashes
        ).unwrap();
        assert_receipt_is_correct(result[0].clone());
        assert_receipt_is_correct(result[1].clone());
    }

    #[test] #[ignore] // ~100 receipts to get ∴ too expensive! Run w/ cargo +nightly test --ignored
    fn should_get_all_receipts_and_set_in_state() {
        let initial_state = get_valid_state_with_endpoint()
            .unwrap();
        let block = get_expected_block();
        let state_with_block = State::set_block_in_state(initial_state, block)
            .unwrap();
        let resultant_state = get_all_receipts_from_block_in_state_and_set_in_state(
            state_with_block
        ).unwrap();
        let receipts_from_state = State::get_receipts_from_state(&resultant_state)
            .unwrap();
        assert_receipt_is_correct(receipts_from_state[TX_INDEX].clone());
    }
}
