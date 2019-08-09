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

fn add_receipt_to_state(state: State, receipt: Receipt) -> Result<State> {
    Ok(State::set_receipt_in_state(state, receipt)?)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use crate::make_rpc_call::deserialize_to_receipt_rpc_response;
    use crate::test_utils::{
        SAMPLE_TX_HASH,
        WORKING_ENDPOINT,
        get_expected_receipt,
        get_valid_initial_state,
        SAMPLE_RECEIPT_JSON_PATH,
        assert_receipt_is_correct,
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
    fn should_add_receipt_to_state_correctly() {
        let receipt = get_expected_receipt();
        let state = get_valid_initial_state().unwrap();
        let resultant_state = add_receipt_to_state(state, receipt).unwrap();
        let result = State::get_receipt_from_state(&resultant_state).unwrap();
        assert_receipt_is_correct(result.clone());
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
}
