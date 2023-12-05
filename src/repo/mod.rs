use crate::{get_cache_dir, Errors};

pub mod crates;
pub mod npm;
pub(crate) mod prelude;
pub mod pypi;

use prelude::*;

#[derive(Serialize, Deserialize)]
pub struct PackageVersion {
    name: String,
    url: String,
    version: String,
    owner: Option<String>,
    release_date: DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    name: String,
    url: Option<String>,
    // version: String,
    owner: Option<String>,
    // release_date: DateTime<chrono::Utc>,
    other_metadata: HashMap<String, String>,
}

#[async_trait]
pub trait Repository {
    fn new() -> Self;
    async fn search(&self, query: &str) -> Result<Vec<Package>, Errors>;
    async fn get_package(&self, name: &str) -> Result<Vec<Package>, Errors>;
    async fn cacheable(&self) -> bool;
    async fn update_cache(&self, min_age: Option<u64>) -> Result<(), Errors>;
    fn get_cache_dir(&self) -> String;

    fn make_cache_dir(&self) -> Result<(), Errors> {
        let path = get_cache_dir().join(self.get_cache_dir());
        if path.exists() {
            Ok(())
        } else {
            std::fs::create_dir_all(path).map_err(|err| err.into())
        }
    }
}
