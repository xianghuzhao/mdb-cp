use crate::config::CopyModeDB;
use crate::config::{Dump, Restore};

use super::CopyControlTrait;

impl CopyControlTrait for CopyModeDB {
    fn get_dump_config(&self) -> &Dump {
        &self.dump
    }

    fn get_restore_config(&self) -> &Restore {
        &self.restore
    }

    fn gen_mode_args(&self) -> Vec<(Vec<String>, Vec<String>)> {
        let mut args = Vec::new();
        if !self.db.is_empty() {
            args.push("--nsFrom".to_owned());
            args.push(format!("{}.*", self.dump.db));
            args.push("--nsTo".to_owned());
            args.push(format!("{}.*", self.db));
        }
        vec![(vec![], args)]
    }
}
