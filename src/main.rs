use anyhow::Result;
use clap::Parser;
use log::{info, warn};
use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};
use std::fs::File;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event};
use std::path::Path; 
use std::sync::mpsc::channel; 
use tokio::task; 
use onedrive_api::{OneDrive, FileName, DriveLocation, ItemLocation}; 

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
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

    let google_drive = google_drive::Client::new(
        String::from("client-id"),
        String::from("client-secret"),
        String::from("redirect-uri"),
        String::from("token"),
        String::from("refresh-token"),
    );

    // Create a channel to receive the events.
    let (tx, rx) = channel::<Event>();

    let mut watcher = notify::recommended_watcher(|res| {
        match res {
           Ok(event) => println!("event: {:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?; 

    add_to_startup(); 

    return Ok(());
}

#[cfg(target_os = "windows")]
fn add_to_startup() {
    // Windows-specific code here
}

#[cfg(target_os = "macos")]
fn add_to_startup() {
    // macOS-specific code here
}

#[cfg(target_os = "linux")]
fn add_to_startup() {
    // Linux-specific code here
}
