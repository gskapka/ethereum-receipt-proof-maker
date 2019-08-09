use serde_json::Value;
use crate::types::Result;

pub fn get_block_by_block_hash_json(block_hash: String) -> Result<Value> {
    Ok(
        json!({
            "id": "1",
            "jsonrpc": "2.0",
            "method": "eth_getBlockByHash",
            "params": [ block_hash, false ],
        })
    )
}

pub fn get_block_by_block_number_json(block_number: String) -> Result<Value> {
    Ok(
        json!({
            "id": "1",
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": [ block_number, false ],
        })
    )
}

pub fn get_transaction_receipt_json(tx_hash: &str) -> Result<Value> {
    Ok(
        json!({
            "id": "1",
            "jsonrpc": "2.0",
            "method": "eth_getTransactionReceipt",
            "params": [ tx_hash ],
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_block_by_block_hash_json_correctly() {
        let dummy_hash = "0xc0ffee".to_string();
        let expected_result = format!("\"{}\"", dummy_hash.clone());
        let result = get_block_by_block_hash_json(dummy_hash)
            .unwrap();
        assert!("\"1\"" == result["id"].to_string());
        assert!("\"2.0\"" == result["jsonrpc"].to_string());
        assert!("\"eth_getBlockByHash\"" == result["method"].to_string());
        assert!("false" == result["params"][1].to_string());
        assert!(expected_result == result["params"][0].to_string());
    }

    #[test]
    fn should_get_block_by_block_number_json_correctly() {
        let dummy_number = "1337".to_string();
        let expected_result = format!("\"{}\"", dummy_number.clone());
        let result = get_block_by_block_number_json(dummy_number)
            .unwrap();
        assert!("\"1\"" == result["id"].to_string());
        assert!("\"2.0\"" == result["jsonrpc"].to_string());
        assert!("\"eth_getBlockByNumber\"" == result["method"].to_string());
        assert!("false" == result["params"][1].to_string());
        assert!(expected_result == result["params"][0].to_string());
    }

    #[test]
    fn should_get_transaction_receipt_json_correctly() {
        let dummy_hash = "0xc0ffee".to_string();
        let expected_result = format!("\"{}\"", &dummy_hash);
        let result = get_transaction_receipt_json(&dummy_hash)
            .unwrap();
        assert!("\"1\"" == result["id"].to_string());
        assert!("\"2.0\"" == result["jsonrpc"].to_string());
        assert!("\"eth_getTransactionReceipt\"" == result["method"].to_string());
        assert!(expected_result == result["params"][0].to_string());
    }
}

