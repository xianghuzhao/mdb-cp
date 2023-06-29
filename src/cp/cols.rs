use crate::config::CopyModeCols;
use crate::config::{Dump, Restore};

use super::CopyControlTrait;

impl CopyControlTrait for CopyModeCols {
    fn get_dump_config(&self) -> &Dump {
        &self.dump
    }

    fn get_restore_config(&self) -> &Restore {
        &self.restore
    }
}
