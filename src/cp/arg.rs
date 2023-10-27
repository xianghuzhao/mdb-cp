use std::collections::HashMap;

use crate::config::{Connection, Param};
use crate::config::{Dump, Restore};

use super::error::Error;
use super::CopyControlTrait;

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
    if !conn.username.is_empty() {
        args.push("--username".to_owned());
        args.push(conn.username.clone());
    }
    if !conn.password.is_empty() {
        args.push("--password".to_owned());
        args.push(conn.password.clone());
    }
    if !conn.authdb.is_empty() {
        args.push("--authenticationDatabase".to_owned());
        args.push(conn.authdb.clone());
    }

    args
}

fn gen_dump_conn_args(
    dump: &Dump,
    conns: &HashMap<String, Connection>,
) -> Result<Vec<String>, Error> {
    let Some(conn) = conns.get(&dump.conn) else {
        return Err(Error::new(&format!(r#"Dump connection "{}" not found"#, &dump.conn)));
    };

    let mut args = gen_conn_args(conn);

    args.push("--db".to_owned());
    args.push(dump.db.clone());

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

fn gen_restore_conn_args(
    restore: &Restore,
    conns: &HashMap<String, Connection>,
) -> Result<Vec<String>, Error> {
    let Some(conn) = conns.get(&restore.conn) else {
        return Err(Error::new(&format!(r#"Restore connection "{}" not found"#, &restore.conn)));
    };

    Ok(gen_conn_args(conn))
}

fn gen_dump_extra_args(param: &Param) -> Vec<String> {
    let mut args = vec!["--archive".to_owned()];
    if param.gzip {
        args.push("--gzip".to_owned());
    }
    args
}

fn gen_restore_extra_args(param: &Param) -> Vec<String> {
    let mut args = vec!["--archive".to_owned()];
    if param.gzip {
        args.push("--gzip".to_owned());
    }
    if param.drop {
        args.push("--drop".to_owned());
    }
    args
}

pub fn gen_dump_common_args<T>(
    mode: &T,
    param: &Param,
    conns: &HashMap<String, Connection>,
) -> Result<Vec<String>, Error>
where
    T: CopyControlTrait,
{
    let dump = mode.get_dump_config();
    let dump_conn_args = gen_dump_conn_args(dump, conns)?;

    let args = gen_dump_extra_args(param)
        .into_iter()
        .chain(dump_conn_args.into_iter());

    Ok(args.collect())
}

pub fn gen_restore_common_args<T>(
    mode: &T,
    param: &Param,
    conns: &HashMap<String, Connection>,
) -> Result<Vec<String>, Error>
where
    T: CopyControlTrait,
{
    let restore = mode.get_restore_config();
    let restore_conn_args = gen_restore_conn_args(restore, conns)?;

    let args = gen_restore_extra_args(param)
        .into_iter()
        .chain(restore_conn_args.into_iter());

    Ok(args.collect())
}
