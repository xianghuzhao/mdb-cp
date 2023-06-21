use std::collections::HashMap;

pub mod error;

use error::Error;

use crate::config::Config;
use crate::config::{Connection, Param};
use crate::config::{CopyMode, CopyModeCols, CopyModeDB, CopyModeNS};
use crate::config::{Dump, Restore};

trait CopyControlTrait {
    fn get_dump_config(&self) -> &Dump;
    fn get_restore_config(&self) -> &Restore;

    fn gen_mode_dump_args(&self) -> Vec<String> {
        vec![]
    }

    fn gen_mode_restore_args(&self) -> Vec<String> {
        vec![]
    }
}

impl CopyControlTrait for CopyModeDB {
    fn get_dump_config(&self) -> &Dump {
        &self.dump
    }

    fn get_restore_config(&self) -> &Restore {
        &self.restore
    }

    fn gen_mode_restore_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if !self.db.is_empty() {
            args.push("-d".to_owned());
            args.push(self.db.clone());
        }
        args
    }
}

impl CopyControlTrait for CopyModeCols {
    fn get_dump_config(&self) -> &Dump {
        &self.dump
    }

    fn get_restore_config(&self) -> &Restore {
        &self.restore
    }
}

impl CopyControlTrait for CopyModeNS {
    fn get_dump_config(&self) -> &Dump {
        &self.dump
    }

    fn get_restore_config(&self) -> &Restore {
        &self.restore
    }
}

fn gen_conn_args(conn: &Connection) -> Vec<String> {
    if !conn.uri.is_empty() {
        return vec!["--uri".to_owned(), conn.uri.clone()];
    }

    let mut args = Vec::new();

    if !conn.host.is_empty() {
        args.push("--host".to_owned());
        args.push(conn.host.clone());
    }
    if conn.port != 0 {
        args.push("--port".to_owned());
        args.push(conn.port.to_string());
    }

    args
}

fn gen_dump_args(dump: &Dump, conns: &HashMap<String, Connection>) -> Result<Vec<String>, Error> {
    let Some(conn) = conns.get(&dump.conn) else {
        return Err(Error::new(&format!(r#"Dump connection "{}" not found"#, &dump.conn)));
    };

    let mut args = gen_conn_args(conn);

    if !dump.db.is_empty() {
        args.push("--db".to_owned());
        args.push(dump.db.clone());
    }

    for ex_col in &dump.exclude_col {
        args.push("--excludeCollection".to_owned());
        args.push(ex_col.clone());
    }
    for ex_col_prefix in &dump.exclude_col_prefix {
        args.push("--excludeCollectionsWithPrefix".to_owned());
        args.push(ex_col_prefix.clone());
    }

    Ok(args)
}

fn gen_restore_args(restore: &Restore, conns: &HashMap<String, Connection>) -> Result<Vec<String>, Error> {
    let Some(conn) = conns.get(&restore.conn) else {
        return Err(Error::new(&format!(r#"Dump connection "{}" not found"#, &restore.conn)));
    };

    Ok(gen_conn_args(conn))
}

fn cp<T>(mode: &T, param: &Param, conns: &HashMap<String, Connection>) -> Result<(), Error>
where
    T: CopyControlTrait,
{
    let dump = mode.get_dump_config();
    let mut dump_args = gen_dump_args(dump, conns)?;

    let restore = mode.get_restore_config();
    let mut restore_args = gen_restore_args(restore, conns)?;


    Ok(())
}

fn run_one_copy(
    cp_mode: &CopyMode,
    param: &Param,
    conns: &HashMap<String, Connection>,
) -> Result<(), Error> {
    match cp_mode {
        CopyMode::DB(mode) => cp(mode, param, conns),
        CopyMode::Cols(mode) => cp(mode, param, conns),
        CopyMode::NS(mode) => cp(mode, param, conns),
    };
    Ok(())
}

pub fn do_copy(cfg: &Config) -> Result<(), Error> {
    let param = &cfg.param;
    let conns = &cfg.connection;

    for cp_mode in &cfg.copy {
        run_one_copy(&cp_mode, &param, &conns)?
    }

    Ok(())
}
