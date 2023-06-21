use crate::config::CopyModeDB;

use super::error::Error;
use super::CopyControlTrait;

pub struct CopyControlDB<'a> {
    mode: &'a CopyModeDB,
}

impl<'a> CopyControlDB<'a> {
    pub fn new(mode: &'a CopyModeDB) -> Self {
        Self { mode }
    }

    fn generate_copy_commands(&self) -> Vec<CopyCommand> {
        let cmd = CopyCommand::new();
        vec![]
    }
}

impl<'a> CopyControlTrait for CopyControlDB<'a> {
    fn cp(&self) -> Result<(), Error> {
        let _args = self.generate_cmd_pair_list();
        Ok(())
    }
}
