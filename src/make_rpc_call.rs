use reqwest;
use std::time::Duration;
use crate::errors::AppError;
use serde_json::Value as Json;
use crate::constants::REQWEST_TIMEOUT_TIME;
use crate::types::{
    Result,
    BlockRpcResponse,
    ReceiptRpcResponse,
};

pub fn make_rpc_call(endpoint: &str, json: Json) -> Result<reqwest::Response> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(REQWEST_TIMEOUT_TIME))
        .build()?;
    Ok(
        client.post(endpoint)
            .json(&json)
            .send()?
    )
}

pub fn get_response_text(mut res: reqwest::Response) -> Result<String> {
    let res_text = res.text()?;
    match res_text.contains("error") {
        false => Ok(res_text),
        true => Err(
            AppError::Custom(
                format!("✘ RPC call failed!\n✘ {}", res_text)
            )
        )
    }
}

pub fn deserialize_to_block_rpc_response(rpc_call_result: String) -> Result<BlockRpcResponse> {
    Ok(serde_json::from_str(&rpc_call_result)?)
}

pub fn deserialize_to_receipt_rpc_response(rpc_call_result: String) -> Result<ReceiptRpcResponse> {
    Ok(serde_json::from_str(&rpc_call_result)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_block::deserialize_block_json_to_block_struct;
    use crate::get_receipt::deserialize_receipt_json_to_receipt_struct;
    use crate::get_rpc_call_jsons::{
        get_block_by_block_hash_json,
        get_transaction_receipt_json,
    };
    use crate::test_utils::{
        SAMPLE_TX_HASH,
        WORKING_ENDPOINT,
        SAMPLE_BLOCK_HASH,
        assert_block_is_correct,
        assert_receipt_is_correct,
    };

    #[test]
    fn should_make_rpc_call_correctly() {
        let block_hash = SAMPLE_BLOCK_HASH.to_string();
        let rpc_call_json = get_block_by_block_hash_json(block_hash)
            .unwrap();
        let result = make_rpc_call(WORKING_ENDPOINT, rpc_call_json)
            .unwrap();
        assert!(result.status() == 200);
    }

    #[test]
    fn should_get_response_text_correctly() {
        let block_hash = SAMPLE_BLOCK_HASH.to_string();
        let rpc_call_json = get_block_by_block_hash_json(block_hash)
            .unwrap();
        let reqwest_response = make_rpc_call(WORKING_ENDPOINT, rpc_call_json)
            .unwrap();
        let result = get_response_text(reqwest_response)
            .unwrap();
        let rpc_result_struct = deserialize_to_block_rpc_response(result)
            .unwrap();
        let result_as_block = deserialize_block_json_to_block_struct(
            rpc_result_struct.result
        ).unwrap();
        assert_block_is_correct(result_as_block)
    }

    #[test]
    fn should_deserialize_rpc_call_to_block_rpc_response_correctly() {
        let block_hash = SAMPLE_BLOCK_HASH.to_string();
        let rpc_call_json = get_block_by_block_hash_json(block_hash)
            .unwrap();
        let reqwest_response = make_rpc_call(WORKING_ENDPOINT, rpc_call_json)
            .unwrap();
        let response_text = get_response_text(reqwest_response)
            .unwrap();
        let rpc_result_struct = deserialize_to_block_rpc_response(response_text)
            .unwrap();
        let result_as_block = deserialize_block_json_to_block_struct(
            rpc_result_struct.result
        ).unwrap();
        assert_block_is_correct(result_as_block)
    }

    #[test]
    fn should_deserialize_rpc_call_to_receipt_rpc_response_correctly() {
        let tx_hash = SAMPLE_TX_HASH.to_string();
        let rpc_call_json = get_transaction_receipt_json(&tx_hash)
            .unwrap();
        let reqwest_response = make_rpc_call(WORKING_ENDPOINT, rpc_call_json)
            .unwrap();
        let response_text = get_response_text(reqwest_response)
            .unwrap();
        let rpc_result_struct = deserialize_to_receipt_rpc_response(response_text)
            .unwrap();
        let result_as_receipt = deserialize_receipt_json_to_receipt_struct(
            rpc_result_struct.result
        ).unwrap();
        assert_receipt_is_correct(result_as_receipt)
    }
}
