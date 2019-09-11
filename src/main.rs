#![feature(try_trait)]
#![feature(const_vec_new)]
#![feature(exclusive_range_pattern)]

mod trie;
mod utils;
mod state;
mod types;
mod errors;
mod get_log;
mod rlp_codec;
mod constants;
mod get_block;
mod trie_nodes;
mod usage_info;
mod test_utils;
mod path_codec;
mod get_receipt;
mod nibble_utils;
mod get_database;
mod get_tx_index;
mod get_endpoint;
mod make_rpc_call;
mod parse_cli_args;
mod get_keccak_hash;
mod connect_to_node;
mod validate_tx_hash;
mod validate_cli_args;
mod get_receipts_trie;
mod get_rpc_call_jsons;
mod initialize_state_from_cli_args;

extern crate simple_logger;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[cfg(test)] #[macro_use] extern crate serial_test_derive;

use crate::state::State;
use crate::parse_cli_args::parse_cli_args;
use crate::connect_to_node::connect_to_node;
use crate::validate_cli_args::validate_cli_args;
use crate::get_endpoint::get_endpoint_and_set_in_state;
use crate::get_tx_index::get_tx_index_and_add_to_state;
use crate::get_receipts_trie::get_receipts_trie_and_set_in_state;
use crate::get_block::get_block_from_tx_hash_in_state_and_set_in_state;
use crate::initialize_state_from_cli_args::initialize_state_from_cli_args;
use crate::get_receipt::get_all_receipts_from_block_in_state_and_set_in_state;

fn main() {
    match parse_cli_args()
        .and_then(validate_cli_args)
        .and_then(initialize_state_from_cli_args)
        .and_then(get_endpoint_and_set_in_state)
        .and_then(connect_to_node)
        .and_then(get_block_from_tx_hash_in_state_and_set_in_state)
        .and_then(get_all_receipts_from_block_in_state_and_set_in_state)
        .and_then(get_tx_index_and_add_to_state)
        .and_then(get_receipts_trie_and_set_in_state)
        //.and_then(extract_branch_from_trie_and_set_in_state)
        //.and_then(create_hex_proof_from_branch_in_state)
        //.and_then(check_proof)
        .and_then(|state| {
            let block = State::get_block_from_state(&state)?;
            let receipts = State::get_receipts_from_state(&state)?;
            let index = State::get_index_from_state(&state)?;
            println!("✔ Tx hash is in block: {:?}", block.number);
            println!("✔ Num txs in block: {:?}", block.transactions.len());
            println!("✔ Receipts in state: {:?}", receipts.len());
            println!("✔ Index in state: {:?}", index);
            Ok(state)
        })
        {
            Ok(_proof) => println!("\n✔ Fin!"),
            Err(e) => println!("{}", e)
        }
}
