#[derive(Debug, Deserialize)]
pub struct CliArgs {
    pub arg_verbose: bool,
    pub arg_txhash: String,
}
