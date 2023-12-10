use std::sync::Arc;

use clap::Parser;
use tidetrawler::cache::Cache;
use tidetrawler::repo::crates::Cargo;
use tidetrawler::repo::npm::Npm;
use tidetrawler::repo::pypi::PyPi;
use tidetrawler::repo::Repository;
use tokio::sync::RwLock;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliOpts {
    query: String,
}

#[tokio::main]
async fn main() {
    let opts = CliOpts::parse();

    let cache = Arc::new(RwLock::new(Cache::default()));

    let mut packages = Vec::new();
    let mut cargo = Cargo::new(cache.clone());
    let mut _cargo1 = Cargo::new(cache.clone());

    match cargo.search(&opts.query).await {
        Ok(val) => packages.extend(val),
        Err(err) => println!("Error getting cargo results: {:?}", err),
    };

    let mut pypi = PyPi::new(cache.clone());
    match pypi.search(&opts.query).await {
        Ok(val) => packages.extend(val),
        Err(err) => println!("Error getting pypi results: {:?}", err),
    };
    let mut npm = Npm::new(cache.clone());
    match npm.search(&opts.query).await {
        Ok(val) => packages.extend(val),
        Err(err) => println!("Error getting npm results: {:?}", err),
    };

    println!("{}", serde_json::to_string_pretty(&packages).unwrap());
}
