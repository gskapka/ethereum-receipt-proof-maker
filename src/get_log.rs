use crate::types::{Log, LogJson, ReceiptJson, Result};
use crate::utils::{convert_hex_strings_to_h256s, convert_hex_to_address, convert_hex_to_bytes};
use ethereum_types::{Bloom, BloomInput};

fn calculate_bloom_from_log(log: &Log) -> Bloom {
    log.topics.iter().fold(
        Bloom::from(BloomInput::Raw(log.address.as_bytes())),
        |mut bloom, topic| {
            bloom.accrue(BloomInput::Raw(topic.as_bytes()));
            bloom
        },
    )
}

fn get_log_from_json(log_json: &LogJson) -> Result<Log> {
    Ok(Log {
        address: convert_hex_to_address(log_json.address.clone())?,
        topics: convert_hex_strings_to_h256s(log_json.topics.clone())?,
        data: convert_hex_to_bytes(log_json.data.clone())?,
    })
}

pub fn get_logs_from_receipt_json(receipt_json: &ReceiptJson) -> Result<Vec<Log>> {
    Ok(receipt_json
        .logs
        .iter()
        .map(|x| get_log_from_json(x).unwrap())
        .collect())
}

pub fn get_logs_bloom_from_logs(logs: &[Log]) -> Result<Bloom> {
    Ok(logs.iter().fold(Bloom::default(), |mut bloom, log| {
        bloom.accrue_bloom(&calculate_bloom_from_log(log));
        bloom
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_rpc_call::deserialize_to_receipt_rpc_response;
    use crate::test_utils::{
        assert_log_is_correct, get_expected_receipt, SAMPLE_RECEIPT_JSON_PATH,
    };
    use hex;
    use std::fs;

    #[test]
    fn should_get_logs_from_receipt_json() {
        let receipt_string = fs::read_to_string(SAMPLE_RECEIPT_JSON_PATH).unwrap();
        let receipt_json = deserialize_to_receipt_rpc_response(receipt_string).unwrap();
        let result = get_logs_from_receipt_json(&receipt_json.result).unwrap();
        assert_log_is_correct(result[0].clone());
    }

    #[test]
    fn should_get_logs_bloom_from_logs_correctly() {
        let receipt = get_expected_receipt();
        let logs = receipt.logs.clone();
        let result = get_logs_bloom_from_logs(&logs).unwrap();
        assert!(result == receipt.logs_bloom);
    }

    #[test]
    fn should_get_log_from_log_json_correctly() {
        let receipt_string = fs::read_to_string(SAMPLE_RECEIPT_JSON_PATH).unwrap();
        let receipt_json = deserialize_to_receipt_rpc_response(receipt_string).unwrap();
        let log_json = receipt_json.result.logs[0].clone();
        let result = get_log_from_json(&log_json).unwrap();
        assert_log_is_correct(result);
    }

    #[test]
    fn should_calculate_bloom_from_log_correctly() {
        let expected_bloom = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000200000000000000000000000000000";
        let expected_bloom_bytes = &hex::decode(expected_bloom).unwrap()[..];
        let receipt = get_expected_receipt();
        let log = receipt.logs[0].clone();
        let result = calculate_bloom_from_log(&log);
        assert!(result.as_bytes() == expected_bloom_bytes)
    }
}
