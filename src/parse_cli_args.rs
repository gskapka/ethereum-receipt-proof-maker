use crate::constants::LOG_FILE_PATH;
use crate::errors::AppError;
use crate::types::Result;
use crate::usage_info::USAGE_INFO;
use crate::utils::convert_hex_to_h256;
use chrono::Utc;
use docopt::Docopt;
use log;
use log::LevelFilter;
use simplelog::*;
use std::fs::File;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CliArgs {
    pub flag_verbose: bool,
    pub arg_txhash: String,
    pub flag_disableLogs: bool,
}

pub fn parse_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO).and_then(|d| d.deserialize()) {
        Ok(cli_args) => {
            let log_file_path = format!("{}{}.log", LOG_FILE_PATH, Utc::now());
            let args: CliArgs = cli_args;
            if args.flag_verbose && !args.flag_disableLogs {
                CombinedLogger::init(vec![
                    TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed)
                        .expect("✘ Error setting up terminal logger!"),
                    WriteLogger::new(
                        LevelFilter::Trace,
                        Config::default(),
                        File::create(log_file_path)?,
                    ),
                ])
                .expect("✘ Error setting up combined logger!");
            } else if args.flag_verbose && args.flag_disableLogs {
                TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed)
                    .expect("✘ Error setting up terminal logger!");
            } else if !args.flag_disableLogs {
                WriteLogger::init(
                    LevelFilter::Trace,
                    Config::default(),
                    File::create(log_file_path)?,
                )?;
            }
            info!("✔ CLI Args parsed successfully!");
            info!("✔ Verbose mode: {}", args.flag_verbose);
            info!(
                "✔ Transaction hash: {}",
                convert_hex_to_h256(args.arg_txhash.clone())?
            );
            Ok(args)
        }
        Err(e) => Err(AppError::Custom(e.to_string())),
    }
}
