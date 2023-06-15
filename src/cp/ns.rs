use std::collections::HashMap;

use crate::config::CopyModeNS;
use crate::config::Connection;

use super::CopyControlTrait;
use super::error::Error;

pub struct CopyControlNS {}

impl CopyControlNS {
    pub fn new(_mode: &CopyModeNS, _conns: &HashMap<String, Connection>) -> Self {
        Self {}
    }
}

impl CopyControlTrait for CopyControlNS {
    fn cp(&self) -> Result<(), Error> {
        Ok(())
    }
}
