use super::prelude::*;

#[derive(Debug)]
pub struct Npm;

#[async_trait]
impl Repository for Npm {
    fn new() -> Self {
        Self {}
    }
    async fn search(&self, _query: &str) -> Result<Vec<Package>, Errors> {
        todo!()
    }
    async fn get_package(&self, _name: &str) -> Result<Vec<Package>, Errors> {
        todo!()
    }
    async fn cacheable(&self) -> bool {
        false
    }
    async fn update_cache(&self, _min_age: Option<u64>) -> Result<(), Errors> {
        todo!()
    }
    fn get_cache_dir(&self) -> String {
        todo!()
    }
}
