mod connect_to_node;
mod constants;
mod errors;
mod get_block;
mod get_branch_from_trie;
mod get_database;
mod get_endpoint;
mod get_hex_proof_from_branch;
mod get_keccak_hash;
mod get_log;
mod get_receipts;
mod get_receipts_trie;
mod get_rpc_call_jsons;
mod get_tx_index;
mod initialize_state_from_cli_args;
mod make_rpc_call;
mod nibble_utils;
mod parse_cli_args;
mod path_codec;
mod rlp_codec;
mod state;
mod test_utils;
mod trie;
mod trie_nodes;
mod types;
mod usage_info;
mod utils;
mod validate_cli_args;
mod validate_tx_hash;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate serial_test_derive;

use crate::connect_to_node::connect_to_node;
use crate::get_block::get_block_from_tx_hash_in_state_and_set_in_state;
use crate::get_branch_from_trie::get_branch_from_trie_and_put_in_state;
use crate::get_endpoint::get_endpoint_and_set_in_state;
use crate::get_hex_proof_from_branch::get_hex_proof_from_branch_in_state;
use crate::get_receipts::get_all_receipts_from_block_in_state_and_set_in_state;
use crate::get_receipts_trie::get_receipts_trie_and_set_in_state;
use crate::get_tx_index::get_tx_index_and_add_to_state;
use crate::initialize_state_from_cli_args::initialize_state_from_cli_args;
use crate::parse_cli_args::parse_cli_args;
use crate::validate_cli_args::validate_cli_args;

fn main() {
    println!("x");
    match parse_cli_args()
        .and_then(|state| {
            println!("poop");
            validate_cli_args(state)
        })
        .and_then(initialize_state_from_cli_args)
        .and_then(get_endpoint_and_set_in_state)
        .and_then(connect_to_node)
        .and_then(get_block_from_tx_hash_in_state_and_set_in_state)
        .and_then(get_all_receipts_from_block_in_state_and_set_in_state)
        .and_then(get_tx_index_and_add_to_state)
        .and_then(get_receipts_trie_and_set_in_state)
        .and_then(get_branch_from_trie_and_put_in_state)
        .and_then(get_hex_proof_from_branch_in_state)
    {
        Ok(hex_proof) => {
            info!("✔ Hex Proof:\n");
            trace!("{}", hex_proof);
            println!("{}", hex_proof);
        }
        Err(e) => {
            error!("{}", e);
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
