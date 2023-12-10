use std::str::FromStr;

use super::prelude::*;

#[derive(Debug)]
pub struct Npm {
    #[allow(dead_code)]
    cache: Arc<RwLock<Cache>>,
}

// {
//     "package": {
//         "name": "api",
//         "scope": "unscoped",
//         "version": "6.1.1",
//         "description": "Magical SDK generation from an OpenAPI definition 🪄",
//         "keywords": [
//             "api",
//             "openapi",
//             "sdk",
//             "swagger"
//         ],
//         "date": {
//             "ts": 1692389286370,
//             "rel": "4 months ago"
//         },
//         "links": {
//             "npm": "https://www.npmjs.com/package/api",
//             "homepage": "https://api.readme.dev",
//             "repository": "https://github.com/readmeio/api",
//             "bugs": "https://github.com/readmeio/api/issues"
//         },

//         "publisher": {
//             "name": "jonursenbach",
//             "avatars": {
//                 "small": "/npm-avatar/eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdmF0YXJVUkwiOiJodHRwczovL3MuZ3JhdmF0YXIuY29tL2F2YXRhci9jMDhhMDY3ZTc1ODQ4MmYxY2VkMzU1NzQ5NDFjN2I3Yj9zaXplPTUwJmRlZmF1bHQ9cmV0cm8ifQ.4Q9K0oksXOxNDkavd24I2-0JSC3n9nxoH7nhkVTEO8U",
//                 "medium": "/npm-avatar/eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdmF0YXJVUkwiOiJodHRwczovL3MuZ3JhdmF0YXIuY29tL2F2YXRhci9jMDhhMDY3ZTc1ODQ4MmYxY2VkMzU1NzQ5NDFjN2I3Yj9zaXplPTEwMCZkZWZhdWx0PXJldHJvIn0.6J0jzXWjQUz4bxTVN3kt_iNGknGL0QCrF1gtTTUnKzg",
//                 "large": "/npm-avatar/eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdmF0YXJVUkwiOiJodHRwczovL3MuZ3JhdmF0YXIuY29tL2F2YXRhci9jMDhhMDY3ZTc1ODQ4MmYxY2VkMzU1NzQ5NDFjN2I3Yj9zaXplPTQ5NiZkZWZhdWx0PXJldHJvIn0.-XYeDmFQ4Jp5S3Vsu76vP5KwgkY9MGqgPRuQtqATf1A"
//             }
//         },
//         "maintainers": [
//             {
//                 "username": "gratcliff",
//                 "email": "gabriel@readme.io"
//             },
//             {
//                 "username": "dannobytes",
//                 "email": "dannlee@gmail.com"
//             },
//             {
//                 "username": "gkoberger",
//                 "email": "gkoberger@gmail.com"
//             },
//             {
//                 "username": "domharrington",
//                 "email": "domharrington+npm@protonmail.com"
//             },
//             {
//                 "username": "mjcuva",
//                 "email": "marc@readme.io"
//             },
//             {
//                 "username": "kanadgupta",
//                 "email": "npm@kanad.dev"
//             },
//             {
//                 "username": "jonursenbach",
//                 "email": "jon@ursenba.ch"
//             },
//             {
//                 "username": "rafegoldberg",
//                 "email": "rafegoldberg@gmail.com"
//             },
//             {
//                 "username": "dashron",
//                 "email": "ahh@fastmail.com"
//             },
//             {
//                 "username": "iliast",
//                 "email": "iliastsangaris@gmail.com"
//             },
//             {
//                 "username": "owlbert",
//                 "email": "support@readme.io"
//             }
//         ],
//         "keywordsTruncated": false
//     },
//     "flags": {
//         "insecure": 0
//     },
//     "score": {
//         "final": 0.2920477571998378,
//         "detail": {
//             "quality": 0.42657724768369254,
//             "popularity": 0.13545118922303823,
//             "maintenance": 0.3333333333333333
//         }
//     },

// }

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmSearchScore {
    #[serde(alias = "final")]
    pub final_score: f64,
    pub detail: NpmSearchScoreDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmSearchScoreDetail {
    pub quality: f64,
    pub popularity: f64,
    pub maintenance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmPackage {
    name: String,
    scope: String,
    version: String,
    description: String,
    author: NpmPerson,
    links: HashMap<String, String>,

    #[serde(alias = "searchScore")]
    pub score: Option<f64>,
    #[serde(alias = "searchScore")]
    pub search_score: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmPerson {
    name: String,
    email: Option<String>,
}

impl std::fmt::Display for NpmPerson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(email) = self.email.clone() {
            write!(f, " <{}>", email)?
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmPackageObject {
    package: NpmPackage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmPagination {
    #[serde(alias = "perPage")]
    per_page: u32,
    page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmSearchResponse {
    pub objects: Vec<NpmPackageObject>,
    pub total: u64,
    pub pagination: NpmPagination,
    // pub time: "Sat Dec 09 2023 23:01:23 GMT+0000 (Coordinated Universal Time)",
    pub user: Option<String>,
    #[serde(alias = "auditLogEnabled")]
    pub audit_log_enabled: bool,
    #[serde(alias = "userEmailVerified")]
    pub user_email_verified: Option<bool>,
    pub url: String,
    pub csrftoken: String,
    pub notifications: Vec<String>,
    #[serde(alias = "npmExpansions")]
    pub npm_expansions: Vec<String>,
    pub flags: Option<HashMap<String, Value>>,
}

impl From<NpmPackage> for Package {
    fn from(value: NpmPackage) -> Self {
        let mut other_metadata: HashMap<String, Value> = HashMap::new();
        other_metadata.insert("version".to_string(), Value::String(value.version));

        Package {
            name: value.name.clone(),
            url: None,
            owner: Some(value.author.to_string()),
            other_metadata,
            repo_type: RepoType::Npm,
        }
    }
}

#[async_trait]
impl Repository for Npm {
    fn repo_type() -> RepoType {
        RepoType::Npm
    }
    fn new(cache: Arc<RwLock<Cache>>) -> Self {
        Self { cache }
    }
    async fn search(&mut self, query: &str) -> Result<Vec<Package>, Errors> {
        let mut url = reqwest::Url::from_str("https://www.npmjs.com/search")
            .map_err(|err| Errors::Generic(format!("Error parsing url: {:?}", err)))?;
        url.query_pairs_mut().append_pair("q", query);
        let client = WebClient::default();

        let res = client.client.get(url).send().await?;

        let body = res.text().await?;
        let data: NpmSearchResponse = serde_json::from_str(&body)?;

        let mut packages = Vec::new();
        data.objects.into_iter().for_each(|obj| {
            let package: Package = obj.package.into();
            packages.push(package);
        });

        Ok(packages)
    }
    async fn get_package(&mut self, _name: &str) -> Result<Vec<Package>, Errors> {
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
