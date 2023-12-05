use std::str::FromStr;

use super::prelude::*;

use async_trait::async_trait;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{make_cache_dir, Errors};

use super::Repository;

const CARGO_API_URL: &str = "https://crates.io/api/v1/crates";

#[derive(Deserialize, Serialize, Debug)]
struct IndexPackage {
    name: String,
    #[serde(alias = "vers")]
    version: String,
    deps: Vec<String>,
    features: HashMap<String, String>,
    cksum: String,
    yanked: bool,
}

impl From<IndexPackage> for Package {
    fn from(pkg: IndexPackage) -> Self {
        let mut other_metadata = HashMap::new();
        other_metadata.insert("checksum".to_string(), pkg.cksum);
        other_metadata.insert("yanked".to_string(), pkg.yanked.to_string());
        if !pkg.features.is_empty() {
            other_metadata.insert("features".to_string(), format!("{:?}", pkg.features));
        }
        Self {
            name: pkg.name.clone(),
            url: Some(format!("https://crates.io/crates/{}", pkg.name)),
            owner: None,
            other_metadata,
        }
    }
}

#[derive(Debug)]
pub struct Cargo;

#[async_trait]
impl Repository for Cargo {
    fn new() -> Self {
        Self {}
    }

    async fn search(&self, query: &str) -> Result<Vec<Package>, Errors> {
        let mut url = reqwest::Url::from_str(CARGO_API_URL)
            .expect("Failed to turn static crates URL into a URL object!");
        url.query_pairs_mut().append_pair("q", query);

        let res = WebClient::default().client.get(url).send().await?;
        let mut packages = Vec::new();

        let cratedata: CratesResponse = res.json().await?;

        if let Some(crates) = cratedata.crates {
            crates.into_iter().for_each(|crt| {
                let package: Package = crt.into();
                packages.push(package);
            });
        }

        Ok(packages)
    }

    fn get_cache_dir(&self) -> String {
        "crates/".to_string()
    }

    async fn get_package(&self, name: &str) -> Result<Vec<Package>, Errors> {
        let url = if name.is_empty() {
            return Err(Errors::Generic("Specify a name!".to_string()));
        } else if name.len() < 5 {
            // https://index.crates.io/2/a-
            format!("https://index.crates.io/{}/{}", name.len(), name)
        } else {
            let first_bits = &name[0..=1];
            let second_bits = &name[2..=3];
            format!(
                "https://index.crates.io/{}/{}/{}",
                first_bits, second_bits, name
            )
        };

        let client = WebClient::default();
        let resp = client.client.get(&url).send().await?;
        println!("{:?}", resp);
        // finding base64 gets you to
        // https://index.crates.io/ba/se/base64

        let res: Vec<Package> = resp
            .text()
            .await?
            .lines()
            .filter_map(|line| {
                let val: Option<IndexPackage> = serde_json::from_str(line).ok();
                val.map(|v| v.into())
            })
            .collect();
        // let res: Vec<Package> = resp.json().await?;
        Ok(res)
    }

    async fn cacheable(&self) -> bool {
        true
    }

    async fn update_cache(&self, _min_age: Option<u64>) -> Result<(), Errors> {
        make_cache_dir()?;
        todo!();

        // for crate_releases in index.crates() {
        //     let _ = crate_releases.most_recent_version(); // newest version
        //     let crate_version = crate_releases.highest_version(); // max version by semver
        //     println!("crate name: {}", crate_version.name());
        //     println!("crate version: {}", crate_version.version());
        // }

        // // check the file is older than min_age seconds
        // if let Some(min_age) = min_age {
        //     if file_older_than(&cache_path, min_age)? {
        //         return Ok(());
        //     }
        // }

        // let mut querystring = "".to_string();
        // let mut cratecache: Vec<Crate> = Vec::new();
        // loop {
        //     let url = format!("{}?{}", CARGO_API_URL, querystring);

        //     let resp = client.client.get(url).send().await?;
        //     let resp_content = resp.text().await?;

        //     let parsed: CratesResposne = match serde_json::from_str(&resp_content) {
        //         Ok(val) => val,
        //         Err(err) => {
        //             println!("Got error parsing response: {:?}", err);
        //             println!("Response: {}", resp_content);
        //             return Err(err.into());
        //         }
        //     };

        //     if let Some(errors) = parsed.errors {
        //         println!("Got errors: {:?}", errors);
        //         break;
        //     }

        //     if let Some(crates) = parsed.crates {
        //         println!(
        //             "Got another {} packages, up to {}",
        //             crates.len(),
        //             cratecache.len()
        //         );
        //         cratecache.extend(crates);
        //     }

        //     if !parsed.meta.has_more() {
        //         break;
        //     } else {
        //         if let Some(next_page) = parsed.meta.next_page {
        //             querystring = next_page;
        //         }
        //     }
        // }

        // let mut file = std::fs::File::create(cache_path)?;
        // let file_contents = serde_json::to_string(&cratecache)?;
        // file.write_all(file_contents.as_bytes())?;

        // println!("done!");
        // Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CratesMeta {
    pub next_page: Option<String>,
    pub prev_page: Option<String>,
    pub total: u64,
}

impl CratesMeta {
    pub fn has_more(&self) -> bool {
        self.next_page.is_some()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Crate {
    name: String,
    homepage: Option<String>,

    id: String,
    badges: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    created_at: DateTime<chrono::Utc>,
    description: Option<String>,
    documentation: Option<String>,
    downloads: Option<u64>,
    exact_match: bool,
    keywords: Option<Vec<String>>,
    links: HashMap<String, String>,
    max_stable_version: String,
    max_version: String,
    newest_version: String,
    recent_downloads: u64,
    repository: Option<String>,
    updated_at: DateTime<chrono::Utc>,
    versions: Option<Vec<String>>,
}

impl From<Crate> for Package {
    fn from(value: Crate) -> Self {
        let mut other_metadata = HashMap::new();

        other_metadata.insert("id".to_string(), value.id.clone());
        if let Some(badges) = value.badges {
            other_metadata.insert("badges".to_string(), badges.join(","));
        }
        if let Some(categories) = value.categories {
            other_metadata.insert("categories".to_string(), categories.join(","));
        }
        other_metadata.insert(
            "created_at".to_string(),
            value.created_at.clone().to_rfc3339(),
        );
        if let Some(description) = value.description {
            other_metadata.insert("description".to_string(), description);
        }
        if let Some(documentation) = value.documentation {
            other_metadata.insert("documentation".to_string(), documentation);
        }
        if let Some(downloads) = value.downloads {
            other_metadata.insert("downloads".to_string(), downloads.to_string());
        }
        other_metadata.insert("exact_match".to_string(), value.exact_match.to_string());
        if let Some(keywords) = value.keywords {
            other_metadata.insert("keywords".to_string(), keywords.join(","));
        }
        for (link_name, link_value) in value.links {
            // TODO: prefix this with the crates URL
            other_metadata.insert(link_name, link_value);
        }
        other_metadata.insert(
            "max_stable_version".to_string(),
            value.max_stable_version.clone(),
        );
        other_metadata.insert("max_version".to_string(), value.max_version.clone());
        other_metadata.insert("newest_version".to_string(), value.newest_version.clone());

        other_metadata.insert(
            "recent_downloads".to_string(),
            value.recent_downloads.to_string(),
        );

        if let Some(repository) = value.repository {
            other_metadata.insert("repository".to_string(), repository);
        }

        other_metadata.insert("updated_at".to_string(), value.updated_at.to_string());

        if let Some(versions) = value.versions {
            other_metadata.insert("versions".to_string(), versions.join(","));
        }

        Package {
            name: value.name,
            url: value.homepage,
            owner: None,
            other_metadata,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CratesResponse {
    pub meta: CratesMeta,
    pub crates: Option<Vec<Crate>>,
    pub errors: Option<Value>,
}
