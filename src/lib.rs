mod config;
pub mod error;

use config::Config;
use error::Error;

pub fn run() -> Result<(), Error> {
    let cfg = Config::load("example.yaml")?;

    println!("{:#?}", cfg);

    Ok(())
}
