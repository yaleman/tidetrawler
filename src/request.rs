use reqwest::Client;

const USER_AGENT: &str = concat!("tidetrawler/", env!("CARGO_PKG_VERSION"));

pub struct WebClient {
    pub client: Client,
}

impl Default for WebClient {
    fn default() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .gzip(true)
            .build()
            .unwrap();
        Self { client }
    }
}
