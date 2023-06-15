use std::collections::HashMap;

use crate::config::CopyModeDB;
use crate::config::Connection;

use super::CopyControlTrait;
use super::error::Error;

pub struct CopyControlDB<'a, 'b> {
    mode: &'a CopyModeDB,
    conns: &'b HashMap<String, Connection>,
}

impl<'a, 'b> CopyControlDB<'a, 'b> {
    pub fn new(mode: &'a CopyModeDB, conns: &'b HashMap<String, Connection>) -> Self {
        Self {
            mode,
            conns,
        }
    }

    fn generate_cmd_pair_list(&self) -> Vec<(Vec<String>, Vec<String>)> {
        vec![]
    }
}

impl<'a, 'b> CopyControlTrait for CopyControlDB<'a, 'b> {
    fn cp(&self) -> Result<(), Error> {
        let _args = self.generate_cmd_pair_list();
        Ok(())
    }
}
