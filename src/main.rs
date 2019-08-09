#![feature(try_trait)]
#![feature(exclusive_range_pattern)]

extern crate serde;
extern crate reqwest;

mod utils;
mod state;
mod types;
mod errors;
mod get_log;
mod constants;
mod get_block;
mod usage_info;
mod test_utils;
mod get_receipt;
mod get_endpoint;
mod make_rpc_call;
mod parse_cli_args;
mod connect_to_node;
mod validate_tx_hash;
mod validate_cli_args;
mod get_rpc_call_jsons;
mod initialize_state_from_cli_args;

#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[cfg(test)] #[macro_use] extern crate serial_test_derive;

use crate::state::State;
use crate::parse_cli_args::parse_cli_args;
use crate::connect_to_node::connect_to_node;
use crate::validate_cli_args::validate_cli_args;
use crate::get_endpoint::get_endpoint_and_set_in_state;
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
        .and_then(get_all_receipts_from_block_in_state_and_set_in_state) {
            Ok(_proof) => println!("\n✔ Fin!"),
            Err(e) => println!("{}", e)
        }
}
