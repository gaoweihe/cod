use anyhow::Result;
use clap::Parser;
use log::{info, warn};
use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};
use std::fs::File;
use tokio::io::AsyncReadExt;
use google_drive::Client;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    config_path: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Conf {
    db_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config_path = tokio::fs::read(args.config_path).await?;
    let config_yml: Conf = serde_yml::from_str(std::str::from_utf8(&config_path).unwrap())?;
    let db_path = config_yml.db_path;
    let db = DB::open_default(db_path.clone()).unwrap();
    let magic_str = db.get("magic").unwrap();
    log::info!("magic: {:?}", magic_str);
    let _ = DB::destroy(&Options::default(), db_path.clone());

    let google_drive = Client::new(
        String::from("client-id"),
        String::from("client-secret"),
        String::from("redirect-uri"),
        String::from("token"),
        String::from("refresh-token"),
    );

    return Ok(());
}
