use std::env;
use walkdir::{DirEntry, WalkDir};

mod egg;

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

// fn find_files(filepath: impl AsRef<Path>, extension: &str) -> Vec<DirEntry> {
//     let mut files: Vec<DirEntry> = Vec::new();
//     for entry in WalkDir::new(filepath) {
//         let entry = entry.expect("Couldn't walk the path.");
//         let name = entry.file_name().to_string_lossy().to_lowercase();

//         if name.ends_with(extension) {
//             files.push(entry);
//         }
//     }

//     files
// }

// fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
//     let file = File::open(filename).expect("I can't open the file.");
//     let buffer = BufReader::new(file);

//     buffer
//         .lines()
//         .map(|l| l.expect("I can't parse the line."))
//         .collect()
// }

// pub fn load_from_file(&self) {
//     fs::create_dir_all(DB_PATH).unwrap();

//     let mut file = OpenOptions::new()
//         .read(true)
//         .write(true)
//         .create(true)
//         .open(DB_FILE)
//         .unwrap();

//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();

//     if contents.len() > 0 {
//         let c = self.data.clone();
//         let mut map = c.lock().unwrap();
//         *map = serde_json::from_str(&contents).unwrap();
//     }
// }

// pub fn save_to_file(&self) {
//     let map = self.data.lock().unwrap();

//     let file = OpenOptions::new()
//         .read(true)
//         .write(true)
//         .create(true)
//         .truncate(true)
//         .open(DB_FILE);

//     let json = serde_json::to_string(&*map).unwrap();
//     file.unwrap().write_all(json.as_bytes()).unwrap();
// }
