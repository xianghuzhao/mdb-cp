use crate::config::CopyModeNS;
use crate::config::{Dump, Restore};

use super::CopyControlTrait;

impl CopyControlTrait for CopyModeNS {
    fn get_dump_config(&self) -> &Dump {
        &self.dump
    }

    fn get_restore_config(&self) -> &Restore {
        &self.restore
    }
}
