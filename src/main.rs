#![feature(try_trait)]
#![feature(exclusive_range_pattern)]

extern crate serde;
extern crate reqwest;

mod utils;
mod state;
mod types;
mod errors;
mod constants;
mod usage_info;
mod get_config;
mod test_utils;
mod get_endpoint;
mod dot_env_utils;
mod parse_cli_args;
mod validate_tx_hash;
mod validate_cli_args;
mod get_rpc_call_jsons;
mod initialize_state_from_cli_args;

#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serial_test_derive;

use crate::state::State;
use crate::parse_cli_args::parse_cli_args;
use crate::validate_cli_args::validate_cli_args;
use crate::get_endpoint::get_endpoint_and_set_in_state;
use crate::initialize_state_from_cli_args::initialize_state_from_cli_args;

fn main() {
    match parse_cli_args()
        .and_then(validate_cli_args)
        .and_then(initialize_state_from_cli_args)
        .and_then(get_endpoint_and_set_in_state)
        .and_then(|_| Ok("Fin!"))
        {
            Ok(proof) => println!("{:?}", proof),
            Err(e) => println!("{}", e)
        }
}
