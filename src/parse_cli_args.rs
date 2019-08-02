use docopt::Docopt;
use crate::types::Result;
use crate::errors::AppError;
use crate::usage_info::USAGE_INFO;
use crate::utils::convert_hex_to_h256;

#[derive(Debug, Deserialize)]
pub struct CliArgs {
    pub flag_verbose: bool,
    pub arg_txhash: String,
}

pub fn parse_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO)
        .and_then(|d| d.deserialize()) {
            Ok(cli_args) => {
                let args: CliArgs = cli_args;
                if args.flag_verbose {
                    println!("\n✔ CLI Args parsed successfully!");
                    println!("✔ Verbose mode: {}", args.flag_verbose);
                    println!(
                        "✔ Transaction hash: {}",
                        convert_hex_to_h256(args.arg_txhash.clone())?
                    );
                }
                Ok(args)
            },
            Err(_e) =>
                Err(
                    AppError::Custom(
                        format!("✘ Error parsing CLI args!:\n{}", USAGE_INFO)
                    )
                )
        }
}
