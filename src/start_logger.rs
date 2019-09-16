use simple_logger;
use crate::types::Result;

pub fn start_logger() -> Result<()> {
    simple_logger::init()?;
    Ok(())
}
