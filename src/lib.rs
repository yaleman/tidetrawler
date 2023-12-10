use std::path::PathBuf;
use std::time::SystemTimeError;

use serde::{Deserialize, Serialize};

pub mod cache;
pub mod repo;
pub mod request;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum Errors {
    Generic(String),
    Reqwest(reqwest::Error),
}

impl From<reqwest::Error> for Errors {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}
impl From<&reqwest::Error> for Errors {
    fn from(err: &reqwest::Error) -> Self {
        Self::Generic(format!("reqwest error: {:?}", err))
    }
}

impl From<std::io::Error> for Errors {
    fn from(err: std::io::Error) -> Self {
        Self::Generic(format!("{:?}", err))
    }
}
impl From<SystemTimeError> for Errors {
    fn from(err: SystemTimeError) -> Self {
        Self::Generic(format!("{:?}", err))
    }
}
impl From<serde_json::Error> for Errors {
    fn from(err: serde_json::Error) -> Self {
        Self::Generic(format!("{:?}", err))
    }
}

pub fn get_cache_dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".cache");
    path.push("tidetrawler");
    path
}

pub fn make_cache_dir() -> Result<(), Errors> {
    let path = get_cache_dir();
    if path.exists() {
        Ok(())
    } else {
        std::fs::create_dir_all(path).map_err(|err| err.into())
    }
}

pub fn file_older_than(filepath: &PathBuf, min_age: u64) -> Result<bool, Errors> {
    if filepath.exists() {
        let metadata = std::fs::metadata(filepath)?;
        let modified = metadata.modified()?;
        let now = std::time::SystemTime::now();
        let age = now.duration_since(modified)?;
        dbg!(min_age, age.as_secs());
        Ok(age.as_secs() < min_age)
    } else {
        Ok(false)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RepoType {
    Cargo,
    PyPi,
    Npm,
}
