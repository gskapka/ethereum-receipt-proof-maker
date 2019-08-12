use std::result;
use serde::Deserialize;
use crate::errors::AppError;
use rlp::{
    RlpStream,
    Encodable
};
use ethereum_types::{
    U256,
    H256,
    Bloom,
    Address,
};

pub type Bytes = Vec<u8>;
pub type Result<T> = result::Result<T, AppError>;

#[derive(Debug, Deserialize)]
pub struct BlockRpcResponse { pub result: BlockJson }

#[derive(Debug, Deserialize)]
pub struct ReceiptRpcResponse { pub result: ReceiptJson }

#[derive(Clone, Debug, Deserialize)]
pub struct Block {
    pub author: Address,
    pub difficulty: U256,
    pub extra_data: Bytes,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub hash: H256,
    pub logs_bloom: Bloom,
    pub miner: Address,
    pub mix_hash: H256,
    pub nonce: U256,
    pub number: U256,
    pub parent_hash: H256,
    pub receipts_root: H256,
    pub seal_fields: (Bytes, U256),
    pub sha3_uncles: H256,
    pub size: U256,
    pub state_root: H256,
    pub timestamp: U256,
    pub total_difficulty: U256,
    pub transactions: Vec<H256>,
    pub transactions_root: H256,
    pub uncles: Vec<H256>,
}


#[derive(Clone, Debug, Deserialize)]
pub struct Receipt {
    pub to: Address,
    pub from: Address,
    pub status: bool,
    pub gas_used: U256,
    pub block_hash: H256,
    pub transaction_hash: H256,
    pub cumulative_gas_used: U256,
    pub block_number: U256,
    pub transaction_index: U256,
    pub contract_address: Address,
    pub logs: Vec<Log>,
    pub root: H256,
    pub logs_bloom: Bloom,
}

impl Encodable for Receipt {
    fn rlp_append(&self, rlp_stream: &mut RlpStream) {
        rlp_stream
            .begin_list(4)
            .append(&self.status)
            .append(&self.cumulative_gas_used)
            .append(&self.logs_bloom)
            .append_list(&self.logs);
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Log {
    pub address: Address,
    pub topics: Vec<H256>,
    pub data: Bytes,
    /*
    removed: bool,
    r#type: String,
    logIndex: String,
    blockHash: String,
    blockNumber: String,
    transactionHash: String,
    transactionIndex: String,
    */
}

impl Encodable for Log {
    fn rlp_append(&self, rlp_stream: &mut RlpStream) {
        rlp_stream
            .begin_list(3)
            .append(&self.address)
            .append_list(&self.topics)
            .append(&self.data);
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BlockJson {
    pub author: String,
    pub difficulty: String,
    pub extraData: String,
    pub gasLimit: String,
    pub gasUsed: String,
    pub hash: String,
    pub logsBloom: String,
    pub miner: String,
    pub mixHash: String,
    pub nonce: String,
    pub number: String,
    pub parentHash: String,
    pub receiptsRoot: String,
    pub sealFields: (String, String),
    pub sha3Uncles: String,
    pub size: String,
    pub stateRoot: String,
    pub timestamp: String,
    pub totalDifficulty: String,
    pub transactions: Vec<String>,
    pub transactionsRoot: String,
    pub uncles: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ReceiptJson {
    pub to: String,
    pub from: String,
    pub status: String,
    pub gasUsed: String,
    pub blockHash: String,
    pub logsBloom: String,
    pub logs: Vec<LogJson>,
    pub blockNumber: String,
    pub root: serde_json::Value,
    pub transactionHash: String,
    pub transactionIndex: String,
    pub cumulativeGasUsed: String,
    pub contractAddress: serde_json::Value,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct LogJson {
    pub data: String,
    pub removed: bool,
    pub r#type: String,
    pub address: String,
    pub logIndex: String,
    pub blockHash: String,
    pub blockNumber: String,
    pub topics: Vec<String>,
    pub transactionHash: String,
    pub transactionIndex: String,
}

