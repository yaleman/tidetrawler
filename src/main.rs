use clap::Parser;
use tidetrawler::repo::crates::Cargo;
use tidetrawler::repo::Repository;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliOpts {
    query: String,
}

#[tokio::main]
async fn main() {
    // println!("Starting up");

    let opts = CliOpts::parse();

    let mut packages = Vec::new();
    let cargo = Cargo::new();

    match cargo.search(&opts.query).await {
        Ok(val) => packages.extend(val),
        Err(err) => println!("Error getting cargo results: {:?}", err),
    };

    println!("{}", serde_json::to_string_pretty(&packages).unwrap());
}
