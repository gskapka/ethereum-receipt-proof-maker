extern crate reqwest;

#[macro_use]
extern crate serde_json;
use serde::{Deserialize, Serialize};

fn get_get_block_by_block_hash_json(block_hash: String) -> serde_json::Value {
    json!({
        "id": "1",
        "jsonrpc": "2.0",
        "method": "eth_getBlockByHash",
        "params": [ block_hash, false ],
    })
}

fn get_get_transaction_receipt_json(tx_hash: String) -> serde_json::Value {
    json!({
        "id": "1",
        "jsonrpc": "2.0",
        "method": "eth_getTransactionReceipt",
        "params": [ tx_hash ],
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    result: Block
}

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    transactions: Vec<Transaction>
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    blockHash: String
}

fn main() {

    let client = reqwest::Client::new();

    let block_hash = "0x1ddd540f36ea0ed23e732c1709a46c31ba047b98f1d99e623f1644154311fe10";

    let mut res = client.post("https://rpc.slock.it/mainnet")
        .json(&get_json(block_hash.to_string()))
        .send()
        .unwrap();

    let result = res.text().unwrap();

    let r: Response = serde_json::from_str(&result).unwrap();
    println!("{:?}", r.result.transactions)
}
