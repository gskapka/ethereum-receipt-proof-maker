use docopt::Docopt;
use crate::types::Result;
use crate::errors::AppError;
use crate::usage_info::USAGE_INFO;

#[derive(Debug, Deserialize)]
pub struct CliArgs {
    pub flag_verbose: bool,
    pub arg_txhash: String,
}

pub fn parse_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO)
        .and_then(|d| d.deserialize()) {
            Ok(cli_args) => Ok(cli_args),
            Err(e) =>
                Err(
                    AppError::Custom(
                        format!("✘ Error parsing CLI args!:\n{}", USAGE_INFO)
                    )
                )
        }
}
