use crate::state::State;
use crate::types::Result;
use crate::parse_cli_args::CliArgs;
use crate::utils::convert_hex_to_h256;

pub fn initialize_state_from_cli_args(cli_args: CliArgs) -> Result<State> {
    info!("✔ Initializing state from CLI args...");
    State::init(
        convert_hex_to_h256(cli_args.arg_txhash.clone())?,
        cli_args.arg_txhash,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_valid_tx_hash_hex;

    #[test]
    fn should_create_state_from_cli_args() {
        let expected_tracing = false;
        let expected_verbosity = true;
        let tx_hash = get_valid_tx_hash_hex();
        let expected_tx_hash = convert_hex_to_h256(tx_hash.clone())
            .unwrap();
        let cli_args = CliArgs {
            arg_txhash: tx_hash,
            flag_trace: expected_tracing,
            flag_verbose: expected_verbosity,
        };
        let state = initialize_state_from_cli_args(cli_args)
            .unwrap();
        assert!(state.tx_hash == expected_tx_hash);
    }
}
