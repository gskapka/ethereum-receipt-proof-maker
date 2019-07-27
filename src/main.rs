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
struct GetBlockResponse {
    result: Block
}

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    transactions: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct GetReceiptResponse {
    result: Receipt
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct Receipt {
    to: String,
    from: String,
    logs: Vec<Log>,
    status: String,
    gasUsed: String,
    blockHash: String,
    logsBloom: String,
    blockNumber: String,
    root: serde_json::Value, // because it's null in this case!
    transactionHash: String,
    transactionIndex: String,
    cumulativeGasUsed: String,
    contractAddress: serde_json::Value, // because it's null in this case!
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct Log {
    data: String,
    removed: bool,
    r#type: String,
    address: String,
    logIndex: String,
    blockHash: String,
    blockNumber: String,
    topics: Vec<String>,
    transactionHash: String,
    transactionIndex: String,
}

fn main() {
    let client = reqwest::Client::new();

    let block_hash = "0x1ddd540f36ea0ed23e732c1709a46c31ba047b98f1d99e623f1644154311fe10";

    let mut res = client.post("https://rpc.slock.it/mainnet")
        .json(&get_get_block_by_block_hash_json(block_hash.to_string()))
        .send()
        .unwrap();

    let result = res.text().unwrap();

    let r: GetBlockResponse = serde_json::from_str(&result).unwrap();
    println!("{:?}", r.result.transactions);
    println!("Length: {:?}", r.result.transactions.len());

    let tx_hash = &r.result.transactions[0];
    let mut res = client.post("https://rpc.slock.it/mainnet")
        .json(&get_get_transaction_receipt_json(tx_hash.to_string()))
        .send()
        .unwrap();

    let result = res.text().unwrap();
    println!("{:?}", result);
    let rec: GetReceiptResponse = serde_json::from_str(&result).unwrap();

    println!("{:?}", rec.result.logs[0].data);
}
