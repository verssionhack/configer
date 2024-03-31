#![allow(dead_code, unused)]

use std::{path::PathBuf, str::FromStr, fs::{create_dir_all, self}, ffi::OsStr, io};
use std::sync::{Arc, RwLock};

use serde::{Deserializer, Serializer, Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{Value, Error};
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

    pub fn set_root(&self, path: &str) {
        *self.path.write().unwrap() = PathBuf::from_str(path).unwrap();
    }

    pub fn hostname(&self) -> Option<&String> {
        self.hostname.as_ref()
    }

    pub fn path(&self, file: &str) -> PathBuf {
        let mut path = self.path.read().unwrap().as_path().join(file);
        if path.extension() != Some(OsStr::new("json")) {
            path.set_extension("json");
        }
        path
    }

    pub fn write<T: Serialize>(&self, filename: &str, v: &T) -> Result<(), io::Error> {
        let mut path = self.path(filename);
        path.parent().map(|p| {
            if !p.exists() {
                fs::create_dir_all(p);
            }
        });
        fs::write(path, serde_json::to_string_pretty(v).unwrap())?;
        Ok(())
    }

    pub fn read<T: DeserializeOwned>(&self, filename: &str) -> Option<Result<T, Error>> {
        let mut path = self.path(filename);
        if let Ok(content) = fs::read_to_string(path) {
            Some(serde_json::from_str(&content))
        } else {
            None
        }
    }
}
