use std::collections::HashMap;

use crate::config::CopyModeCols;
use crate::config::Connection;

use super::CopyControlTrait;
use super::error::Error;

pub struct CopyControlCols {}

impl CopyControlCols {
    pub fn new(_mode: &CopyModeCols) -> Self {
        Self {}
    }
}

impl CopyControlTrait for CopyControlCols {
    fn cp(&self) -> Result<(), Error> {
        Ok(())
    }
}
