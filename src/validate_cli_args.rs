use crate::types::Result;
use crate::parse_cli_args::CliArgs;
use crate::validate_tx_hash::validate_tx_hash;

pub fn validate_cli_args(cli_args: CliArgs) -> Result<CliArgs> {
    info!("✔ Validating CLI args...");
    validate_tx_hash(cli_args.arg_txhash.clone())
        .and_then(|_| Ok(cli_args))
}
