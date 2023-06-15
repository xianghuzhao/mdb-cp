use std::collections::HashMap;

mod cols;
mod db;
pub mod error;
mod ns;

use error::Error;

use crate::config::Config;
use crate::config::Connection;
use crate::config::CopyMode;

trait CopyControlTrait {
    fn cp(&self) -> Result<(), Error>;
}

fn run_one_copy(cp_mode: &CopyMode, conns: &HashMap<String, Connection>) -> Result<(), Error> {
    match cp_mode {
        CopyMode::DB(mode) => db::CopyControlDB::new(mode, conns).cp()?,
        CopyMode::Cols(mode) => cols::CopyControlCols::new(mode, conns).cp()?,
        CopyMode::NS(mode) => ns::CopyControlNS::new(mode, conns).cp()?,
    };
    Ok(())
}

pub fn do_copy(cfg: &Config) -> Result<(), Error> {
    let conns = &cfg.connection;

    for cp_mode in &cfg.copy {
        run_one_copy(&cp_mode, &conns)?
    }

    Ok(())
}
