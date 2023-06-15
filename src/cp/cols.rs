use std::collections::HashMap;

use crate::config::CopyModeCols;
use crate::config::Connection;

use super::CopyControlTrait;
use super::error::Error;

pub struct CopyControlCols {}

impl CopyControlCols {
    pub fn new(_mode: &CopyModeCols, _conns: &HashMap<String, Connection>) -> Self {
        Self {}
    }
}

impl CopyControlTrait for CopyControlCols {
    fn cp(&self) -> Result<(), Error> {
        Ok(())
    }
}
