use docopt::Docopt;
use log::LevelFilter;
use simplelog::*;

use crate::{errors::AppError, types::Result, usage_info::USAGE_INFO, utils::convert_hex_to_h256};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CliArgs {
    pub flag_verbose: bool,
    pub arg_txhash: String,
}

pub fn parse_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO).and_then(|d| d.deserialize::<CliArgs>()) {
        Ok(cli_args) => {
            TermLogger::init(
                if cli_args.flag_verbose {
                    LevelFilter::Trace
                } else {
                    LevelFilter::Info
                },
                Config::default(),
                TerminalMode::Mixed,
            )?;
            info!("✔ CLI Args parsed successfully!");
            info!("✔ Verbose mode: {}", cli_args.flag_verbose);
            info!(
                "✔ Transaction hash: {}",
                convert_hex_to_h256(cli_args.arg_txhash.clone())?
            );
            Ok(cli_args)
        }
        Err(e) => Err(AppError::Custom(e.to_string())),
    }
}
