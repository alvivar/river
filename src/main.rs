mod egg;

use std::env;
use walkdir::{DirEntry, WalkDir};

#[tokio::main]
async fn main() {
    // Twitter authentication.
    egg::Config::load().await;

    // Scan the current directory
    let current_dir = env::current_dir().unwrap();

    let mut files: Vec<DirEntry> = Vec::new();
    for entry in WalkDir::new(current_dir) {
        files.push(entry.unwrap());
    }

    let mut tweet_file = String::new();
    for file in files {
        let name = file.file_name().to_str().unwrap();
        let path = file.path().to_str().unwrap();
        let desc = name.replace('-', " ").replace('_', " ").replace('.', " ");

        tweet_file.push_str(&format!("tweet] {}\n", desc,));
        tweet_file.push_str(&format!("media] {:?}\n", path));
        tweet_file.push_str(&format!("state] Pending\n"));
        tweet_file.push_str(&format!("\n"));
    }
    println!("{}", tweet_file.trim());

    // Updates the config file with new files found
}
