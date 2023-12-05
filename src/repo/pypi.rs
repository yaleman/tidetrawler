//! Repository hooks for PyPi
//!
use super::prelude::*;

// const PYPI_SEARCH_URL: &str = "https://pypi.org/search/";
/// Put the package name on the end of this URL

// const PYPI_REPO_URL: &str = "https://pypi.org/project/{}";

#[derive(Debug)]
pub struct PyPi;

#[async_trait]
impl Repository for PyPi {
    fn new() -> Self {
        Self {}
    }
    fn get_cache_dir(&self) -> String {
        "pypi/".to_string()
    }

    async fn search(&self, _query: &str) -> Result<Vec<Package>, Errors> {
        // make_cache_dir()?;
        todo!()
    }

    async fn get_package(&self, _name: &str) -> Result<Vec<Package>, Errors> {
        make_cache_dir()?;
        todo!()
    }

    async fn cacheable(&self) -> bool {
        todo!()
    }

    async fn update_cache(&self, _min_age: Option<u64>) -> Result<(), Errors> {
        make_cache_dir()?;
        todo!()
    }
}
