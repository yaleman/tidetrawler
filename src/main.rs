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

    let cargo = Cargo::new();

    let packages = cargo.search(&opts.query).await.unwrap();
    println!("{}", serde_json::to_string_pretty(&packages).unwrap());
}
