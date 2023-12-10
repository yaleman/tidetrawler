//! Caching things
//!
use chrono::{DateTime, Duration};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::Errors;

pub(crate) fn hash_url(url: &str) -> String {
    sha256::digest(url)
}

#[derive(Debug)]
pub struct Cache {
    pub cache_dir: PathBuf,
    /// The key is the source URL, sha256'd to a hex string
    pub data: HashMap<String, CacheData>,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            cache_dir: home_dir().unwrap().join(".cache").join("tidetrawler"),
            data: HashMap::new(),
        }
    }
}

impl Cache {
    /// Get the cache path for an item
    fn cache_path(&self, url: &str) -> PathBuf {
        let cache_key = hash_url(url);
        self.cache_dir.join(format!("{}.json", cache_key))
    }

    pub fn save(&self, data: CacheData) -> Result<(), Errors> {
        let cache_path = self.cache_path(&data.url);
        let cache_data = serde_json::to_string(&data)?;
        std::fs::write(cache_path, cache_data)?;
        Ok(())
    }

    pub fn clean_cache(&mut self, max_age: Duration) {
        self.data.clear();
        for file in self.cache_dir.read_dir().unwrap() {
            let file = file.unwrap();
            let path = file.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".json") {
                let cache_path = self.cache_path(file_name);
                match self.get_cache(file_name, Some(max_age), Some(true)) {
                    Some(data) => {
                        self.data.insert(data.get_hash(), data);
                    }
                    None => {
                        std::fs::remove_file(cache_path).ok();
                    }
                }
            } else {
                eprintln!(
                    "Uh, what's this extra file in the cache dir? {:?}",
                    file_name
                );
            }
        }
    }

    fn load_file(&self, cache_path: &PathBuf) -> Option<CacheData> {
        let file = File::open(cache_path).ok()?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file into the object
        serde_json::from_reader(reader).ok()
    }

    pub fn get_cache(
        &self,
        url: &str,
        max_age: Option<Duration>,
        delete_expired: Option<bool>,
    ) -> Option<CacheData> {
        let cache_path = self.cache_path(url);
        match self.cache_path(url).exists() {
            false => None,
            true => {
                let data = self.load_file(&cache_path)?;
                if let Some(max_age) = max_age {
                    if data.updated + max_age < chrono::Utc::now() {
                        if let Some(delete_expired) = delete_expired {
                            if delete_expired {
                                std::fs::remove_file(cache_path).ok();
                            }
                        }
                        return None;
                    }
                }

                Some(data)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheData {
    /// Typically the etag or whatever expiry data you've got
    pub cache_id: String,
    /// Update time
    pub updated: DateTime<chrono::Utc>,
    /// The source URL
    pub url: String,
    /// The data
    pub content: String,
}

impl CacheData {
    pub fn new(url: String, cache_id: String, content: String) -> Self {
        Self {
            url,
            cache_id,
            content,
            updated: chrono::Utc::now(),
        }
    }
    pub fn get_hash(&self) -> String {
        hash_url(&self.url)
    }
}
