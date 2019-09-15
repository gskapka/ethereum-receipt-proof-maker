use log;
use docopt::Docopt;
use log::LevelFilter;
use crate::types::Result;
use crate::simple_logger;
use crate::errors::AppError;
use crate::usage_info::USAGE_INFO;
use crate::utils::convert_hex_to_h256;

#[derive(Debug, Deserialize)]
pub struct CliArgs {
    pub flag_trace: bool,
    pub flag_verbose: bool,
    pub arg_txhash: String,
}

pub fn parse_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO)
        .and_then(|d| d.deserialize()) {
            Ok(cli_args) => {
                log::set_max_level(LevelFilter::Error);
                let args: CliArgs = cli_args;
                // TODO: Factor out log level stuff into own module!
                if args.flag_verbose {
                    log::set_max_level(LevelFilter::Info);
                }
                if args.flag_trace {
                    log::set_max_level(LevelFilter::Trace);
                }
                info!("✔ CLI Args parsed successfully!");
                info!("✔ Verbose mode: {}", args.flag_verbose);
                info!(
                    "✔ Transaction hash: {}",
                    convert_hex_to_h256(args.arg_txhash.clone())?
                );
                Ok(args)
            },
            Err(e) =>
                Err(
                    AppError::Custom(
                        format!("✘ Error parsing CLI args!:\n{}\n{}", e, USAGE_INFO)
                    )
                )
        }
}
