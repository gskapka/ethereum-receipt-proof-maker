use crate::state::State;
use crate::types::Result;
use crate::errors::AppError;
use crate::cli_arg_struct::CliArgs;
use crate::get_endpoint::get_endpoint_and_set_in_state;
use crate::check_tx_hash::check_tx_hash_add_set_in_state;

pub fn make_merkle_receipt_proof(cli_args: CliArgs) -> Result<String> {
    State::get_initial_state()
        .and_then(get_endpoint_and_set_in_state)
        .and_then(|state| check_tx_hash_add_set_in_state(state, cli_args.arg_txhash))
        .and_then(|_| Ok("Finito!".to_string()))
}
