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

        let vars = args
            .find(|arg| arg == "--config-vars")
            .and_then(|_| args.next())
            .unwrap_or_else(|| "default".to_string());

        if let Some(vars) = value.vars.get(&vars) {
            for (key, value) in vars {
                contents = contents.replace(&format!("${{{key}}}"), value);
            }
        }

        value = json_from_str(&contents)?;
        value.meta.path = path;
        value.meta.contents = contents;

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
    pub db: Db,
    pub vars: HashMap<String, HashMap<String, String>>,

    #[serde(skip)]
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Meta {
    pub path: PathBuf,
    pub contents: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Db {
    pub mongo: Mongo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mongo {
    pub name: String,
    pub url: String,
    pub path: String,
}
