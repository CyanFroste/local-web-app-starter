use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::from_str as json_from_str;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

impl Config {
    pub fn from(path: impl AsRef<Path>) -> Result<Self> {
        let mut args = env::args();
        let path = path.as_ref().to_path_buf();

        let mut contents = fs::read_to_string(&path)?;
        let mut value: Self = json_from_str(&contents)?;

        let ns = args
            .find(|arg| arg == "--ns")
            .and_then(|_| args.next())
            .unwrap_or_else(|| "default".to_string());

        if let Some(vars) = value.vars.get(&ns) {
            for (k, v) in vars {
                contents = contents.replace(&format!("${{{k}}}"), v);
            }
        }

        value = json_from_str(&contents)?;
        value.location = path;
        value.raw = contents;

        Ok(value)
    }

    pub fn find(name: &str, levels: usize) -> Result<Self> {
        let path = env::current_exe()?;

        for ancestor in path.ancestors().skip(1).take(levels) {
            let path = ancestor.join(name);

            if path.exists() {
                return Self::from(path);
            }
        }

        Err(Error::new(format!("config: {} not found", name)))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub db: DbConfig,
    pub vars: HashMap<String, HashMap<String, String>>,

    #[serde(skip)]
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Meta {
    pub path: PathBuf,
    pub contents: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbConfig {
    pub mongo: MongoConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MongoConfig {
    pub name: String,
    pub url: String,
    pub path: String,
}
