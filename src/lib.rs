mod arg;
mod config;
mod cp;
pub mod error;

use config::Config;
use error::Error;

pub fn run() -> Result<(), Error> {
    let args = arg::parse_arg();

    let mut cfg = Config::load(&args.config)?;
    cfg.mix_args(args.gzip, args.drop, args.yes, &args.dump, &args.restore);

    println!("Configuration in struct:\n{:#?}", cfg);

    cp::do_copy(&cfg)?;

    Ok(())
}
