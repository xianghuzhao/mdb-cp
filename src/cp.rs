use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::config::Config;
use crate::config::CopyMode;
use crate::config::{Connection, Param};
use crate::config::{Dump, Restore};

pub mod error;

mod arg;
mod cols;
mod db;
mod ns;

use error::Error;

pub trait CopyControlTrait {
    fn get_dump_config(&self) -> &Dump;
    fn get_restore_config(&self) -> &Restore;

    fn gen_mode_args(&self) -> Vec<(Vec<String>, Vec<String>)> {
        vec![]
    }
}

fn show_cmd(cmd: &str, args: &Vec<String>) {
    println!("{}: {}", cmd, args.join(" "));
}

fn run_cp(
    param: &Param,
    dump_common_args: &Vec<String>,
    restore_common_args: &Vec<String>,
    dump_mode_args: &Vec<String>,
    restore_mode_args: &Vec<String>,
) -> Result<(), Error> {
    let dump_args = dump_common_args
        .clone()
        .into_iter()
        .chain(dump_mode_args.clone().into_iter())
        .collect();
    let restore_args = restore_common_args
        .clone()
        .into_iter()
        .chain(restore_mode_args.clone().into_iter())
        .collect();

    show_cmd(&param.dump, &dump_args);
    show_cmd(&param.restore, &restore_args);

    print!("Confirm copy? (y/n) ");
    std::io::stdout()
        .flush()
        .map_err(|err| Error::new(&format!("Flush stdout error -> {}", err)))?;

    let mut confirm_input = String::new();
    std::io::stdin()
        .read_line(&mut confirm_input)
        .map_err(|err| Error::new(&format!("Read user input error -> {}", err)))?;

    let confirm_input = confirm_input.trim();
    let confirm = confirm_input == "y" || confirm_input == "Y";
    if !confirm {
        return Ok(());
    }

    let dump_cmd = Command::new(&param.dump)
        .args(&dump_args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| Error::new(&format!("Dump command error -> {}", err)))?;

    let mut restore_cmd = Command::new(&param.restore)
        .args(&restore_args)
        .stdin(dump_cmd.stdout.unwrap())
        .spawn()
        .map_err(|err| Error::new(&format!("Restore command error -> {}", err)))?;

    restore_cmd
        .wait()
        .map_err(|err| Error::new(&format!("Wait error -> {}", err)))?;

    Ok(())
}

fn cp<T>(mode: &T, param: &Param, conns: &HashMap<String, Connection>) -> Result<(), Error>
where
    T: CopyControlTrait,
{
    let dump_common_args = arg::gen_dump_common_args(mode, param, conns)?;
    let restore_common_args = arg::gen_restore_common_args(mode, param, conns)?;

    for mode_args in mode.gen_mode_args() {
        run_cp(
            param,
            &dump_common_args,
            &restore_common_args,
            &mode_args.0,
            &mode_args.1,
        )?;
    }

    Ok(())
}

pub fn do_copy(cfg: &Config) -> Result<(), Error> {
    let param = &cfg.param;
    let conns = &cfg.connection;

    for cp_mode in &cfg.copy {
        match cp_mode {
            CopyMode::DB(mode) => cp(mode, param, conns)?,
            CopyMode::Cols(mode) => cp(mode, param, conns)?,
            CopyMode::NS(mode) => cp(mode, param, conns)?,
        };
    }

    Ok(())
}
