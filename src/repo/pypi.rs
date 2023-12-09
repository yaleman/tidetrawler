//! Repository hooks for PyPi
//!

use super::prelude::*;

// possible implementation here? https://github.com/victorgarric/pip_search/blob/master/pip_search/pip_search.py

// const PYPI_SEARCH_URL: &str = "https://pypi.org/search/";
/// Put the package name on the end of this URL

// const PYPI_REPO_URL: &str = "https://pypi.org/project/{}";

#[derive(Debug)]
pub struct PyPi;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct PyPiPackageInfo {
    pub author: String,
    pub author_email: String,
    pub bugtrack_url: Option<String>,
    pub classifiers: Vec<String>,
    pub description: String,
    pub description_content_type: String,
    pub docs_url: Option<String>,
    pub download_url: String,
    pub downloads: HashMap<String, i64>,
    pub home_page: Option<String>,
    pub keywords: String,
    pub license: String,
    pub maintainer: String,
    pub maintainer_email: String,
    pub name: String,
    pub package_url: String,
    pub platform: Option<String>,
    pub project_url: Option<String>,
    pub project_urls: Option<HashMap<String, String>>,
    pub release_url: String,
    pub requires_dist: Vec<String>,
    pub requires_python: Option<String>,
    pub summary: Option<String>,
    pub version: String,
    pub yanked: bool,
    pub yanked_reason: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct PyPiPackage {
    pub info: PyPiPackageInfo,
    pub last_serial: u64,
    pub releases: HashMap<String, Vec<Value>>,
    pub urls: Vec<Value>,
    pub vulnerabilities: Vec<Value>,
}

impl From<PyPiPackage> for Package {
    fn from(value: PyPiPackage) -> Self {
        let mut other_metadata: HashMap<String, Value> = HashMap::new();

        let info_as_value: Value = serde_json::to_value(value.info.clone())
            .expect("Couldn't serialise our own struct into a value");

        for (key, value) in info_as_value.as_object().unwrap() {
            if let Some(value) = value.as_str() {
                if !value.is_empty() {
                    other_metadata.insert(key.to_string(), Value::String(value.to_string()));
                }
            } else {
                other_metadata.insert(key.to_string(), value.clone());
            }
        }

        let mut owner = value.info.maintainer;
        owner = match owner.is_empty() {
            true => value.info.author,
            false => owner,
        };

        Package {
            name: value.info.name,
            url: Some(value.info.package_url),
            owner: Some(owner),
            other_metadata,
            repo_type: crate::RepoType::PyPi,
        }
    }
}

#[async_trait]
impl Repository for PyPi {
    fn repo_type() -> crate::RepoType {
        crate::RepoType::PyPi
    }
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

    async fn get_package(&self, name: &str) -> Result<Vec<Package>, Errors> {
        let client = WebClient::default();
        let url = format!("https://pypi.org/pypi/{}/json", name);
        let res = client.client.get(&url).send().await?;
        let res_text = res.text().await?;
        let package: PyPiPackage = serde_json::from_str(&res_text)?;

        Ok(vec![package.into()])
    }

    async fn cacheable(&self) -> bool {
        todo!()
    }

    async fn update_cache(&self, _min_age: Option<u64>) -> Result<(), Errors> {
        make_cache_dir()?;
        todo!()
    }
}
