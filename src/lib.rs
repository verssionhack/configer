#![allow(dead_code, unused)]

use std::{path::PathBuf, str::FromStr, fs::{create_dir_all, self}, ffi::OsStr};
use std::sync::{Arc, RwLock};

use serde::Deserializer;
use serde_json::Value;
pub struct Configer {
    hostname: Option<String>,
    path: Arc<RwLock<PathBuf>>,
}

impl Configer {
    pub fn new(path: &str) -> Self {
        let p = PathBuf::from_str(path).unwrap();
        if !p.as_path().exists() {
            create_dir_all(path).expect("Failed to complete dir");
        }
        Self { 
            hostname: hostname::get().map(|v| v.to_str().map(|v| v.to_string())).unwrap(),
            path: Arc::new(RwLock::new(p)),
        }
    }

    pub fn set_root(&self, path: impl ToString) {
        *self.path.write().unwrap() = PathBuf::from_str(&path.to_string()).unwrap();
    }

    pub fn hostname(&self) -> Option<&String> {
        self.hostname.as_ref()
    }

    pub fn path(&self, file: impl ToString) -> PathBuf {
        let mut path = self.path.read().unwrap().as_path().join(file.to_string());
        if path.extension() != Some(OsStr::new("json")) {
            path.set_extension("json");
        }
        path
    }

    pub fn write(&self, filename: impl ToString, v: &Value) {
        let mut path = self.path(filename);
        path.parent().map(|p| {
            if !p.exists() {
                fs::create_dir_all(p);
            }
        });
        fs::write(path, serde_json::to_string_pretty(v).expect("Failed to format json"));
    }

    pub fn read(&self, filename: impl ToString) -> Option<Value> {
        let mut path = self.path(filename);
        fs::read_to_string(path).map_or(None, |v| serde_json::from_str(&v).map_or(None, |v| v))
    }
}
