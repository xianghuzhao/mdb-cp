use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use log::debug;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

pub mod error;

const MONGODUMP_CMD: &str = "mongodump";
const MONGORESTORE_CMD: &str = "mongorestore";

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Extra {
    gzip: bool,
    yes: bool,
    dump: String,
    restore: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Connection {
    uri: String,
    host: String,
    port: u16,
    username: String,
    password: String,
    authdb: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct DBParam {
    conn: String,
    db: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct CopyModeDB {
    from: DBParam,
    to: DBParam,
    exclude_col: Vec<String>,
    exclude_col_prefix: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ColPair {
    Map(HashMap<String, String>),
    Str(String),
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct CopyModeCols {
    from: DBParam,
    to: DBParam,
    col: Vec<ColPair>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct CopyModeNS {
    from: DBParam,
    to: DBParam,
    ns_include: String,
    ns_from: String,
    ns_to: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "mode")]
pub enum CopyMode {
    DB(CopyModeDB),
    Cols(CopyModeCols),
    NS(CopyModeNS),
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub extra: Extra,
    pub connection: HashMap<String, Connection>,
    pub copy: Vec<CopyMode>,
}

impl Config {
    fn load_yaml_from_file(file_path: &str) -> Result<Value, error::Error> {
        let yaml_str = fs::read_to_string(file_path)
            .map_err(|err| error::Error::new(file_path, &err.to_string()))?;
        let yaml_value = serde_yaml::from_str(&yaml_str)
            .map_err(|err| error::Error::new(file_path, &err.to_string()))?;
        Ok(yaml_value)
    }

    fn patch_yaml_value(val: &mut Value, val_patch: Value) {
        if let Value::Mapping(val) = val {
            if let Value::Mapping(val_patch) = val_patch {
                for (k, v) in val_patch {
                    Self::patch_yaml_value(val.entry(k).or_insert(Value::Null), v);
                }
                return;
            }
        }
        *val = val_patch;
    }

    fn include_config(cfg: &mut Value, cfg_dir: &Path, inc_file: &str) -> Result<(), error::Error> {
        let mut inc_path = PathBuf::new();
        inc_path.push(cfg_dir);
        inc_path.push(inc_file);

        let Some(inc_path) = inc_path.to_str() else {
            return Err(error::Error::new(inc_file, "Invalid config file name"));
        };

        debug!("Load included config file: {}", inc_path);

        let cfg_inc = Self::load_yaml_from_file(inc_path)?;
        Self::patch_yaml_value(cfg, cfg_inc);
        Ok(())
    }

    fn merge_all_config(cfg: &mut Value, cfg_dir: &Path) -> Result<(), error::Error> {
        if let Value::Mapping(cfg_map) = cfg {
            if let Some(cfg_include) = cfg_map.remove("include") {
                match cfg_include {
                    Value::String(inc_file) => Self::include_config(cfg, cfg_dir, &inc_file)?,
                    Value::Sequence(inc_files) => {
                        for inc_file_value in inc_files {
                            if let Value::String(inc_file) = inc_file_value {
                                Self::include_config(cfg, cfg_dir, &inc_file)?;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }

    pub fn load(config_path: &str) -> Result<Self, error::Error> {
        debug!("Load main config file: {}", config_path);
        let mut cfg = Self::load_yaml_from_file(config_path)?;

        match cfg {
            Value::Mapping(_) => (),
            _ => return Err(error::Error::new(config_path, "Configuration is not a map")),
        };

        if let Some(cfg_dir) = Path::new(config_path).parent() {
            Self::merge_all_config(&mut cfg, &cfg_dir)?;
        }

        if let Ok(vv) = serde_yaml::to_string(&cfg) {
            debug!("Configuration from \"serde_yaml::Value\":\n{}", vv)
        }

        let cfg = serde_yaml::from_value(cfg)
            .map_err(|err| error::Error::new(config_path, &err.to_string()))?;

        if let Ok(vv) = serde_yaml::to_string(&cfg) {
            debug!("Configuration from struct \"Config\":\n{}", vv)
        }

        Ok(cfg)
    }

    pub fn mix_args(
        &mut self,
        gzip: bool,
        yes: bool,
        dump: &Option<String>,
        restore: &Option<String>,
    ) {
        self.extra.gzip |= gzip;
        self.extra.yes |= yes;

        if let Some(cmd) = dump {
            self.extra.dump = cmd.clone();
        } else if self.extra.dump.is_empty() {
            self.extra.dump = String::from(MONGODUMP_CMD);
        }

        if let Some(cmd) = restore {
            self.extra.restore = cmd.clone();
        } else if self.extra.restore.is_empty() {
            self.extra.restore = String::from(MONGORESTORE_CMD);
        }
    }
}
