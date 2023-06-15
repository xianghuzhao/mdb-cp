mod arg;
mod config;
pub mod error;

use config::Config;
use error::Error;

pub fn run() -> Result<(), Error> {
    let args = arg::parse_arg();

    let cfg = Config::load(&args.config)?;

    println!("{:#?}", cfg);

    Ok(())
}
