mod egg;

use std::{
    collections::HashMap,
    env,
    io::{Read, Write},
};
use walkdir::{DirEntry, WalkDir};

const QUEUE_FILE: &str = "river.queue.txt";
const TWEET: &str = "tweet";
const IMAGE: &str = "image";
const STATE: &str = "state";

const PENDING: &str = "Pending";
// const SENT: &str = "Send";
// const ERROR: &str = "Error";

#[derive(Debug)]
struct Tweet {
    text: String,
    image: String,
    state: String,
}

impl Tweet {
    pub fn new() -> Tweet {
        let text = String::new();
        let image = String::new();
        let state = String::new();

        Tweet { text, image, state }
    }
}

#[derive(Debug)]
struct Tweets {
    queue: Vec<Tweet>,
}

impl Tweets {
    pub fn new() -> Tweets {
        let queue = Vec::new();

        Tweets { queue }
    }
}

#[derive(Debug)]
struct Days {
    mon: Vec<String>,
    tue: Vec<String>,
    wed: Vec<String>,
    thu: Vec<String>,
    fri: Vec<String>,
    sat: Vec<String>,
    sun: Vec<String>,
}

impl Days {
    pub fn new() -> Days {
        let mon = Vec::<String>::new();
        let tue = Vec::<String>::new();
        let wed = Vec::<String>::new();
        let thu = Vec::<String>::new();
        let fri = Vec::<String>::new();
        let sat = Vec::<String>::new();
        let sun = Vec::<String>::new();

        Days {
            mon,
            tue,
            wed,
            thu,
            fri,
            sat,
            sun,
        }
    }
}

#[tokio::main]
async fn main() {
    // Twitter authentication.
    egg::Config::load().await;

    let current_dir = env::current_dir().unwrap();

    // Updates the config file with new files found.

    let mut config = String::new();

    let mut tweets = Tweets::new();
    let mut days = Days::new();

    if let Ok(mut file) = std::fs::File::open(QUEUE_FILE) {
        file.read_to_string(&mut config).unwrap();

        // Two lines defines our sections.
        for block in config.split("\n\n") {
            // One line is the key and the value.
            for line in block.split('\n') {
                if !line.contains(']') {
                    continue;
                }

                let mut kv = line.split(']');
                let k = kv.next().unwrap().trim();
                let v = kv.next().unwrap().trim();

                let v = remove_whitespace(v);

                // It's a day?
                match k.to_lowercase().as_str() {
                    "mon" => days.mon.push(v),
                    "tue" => days.tue.push(v),
                    "wed" => days.wed.push(v),
                    "thu" => days.thu.push(v),
                    "fri" => days.fri.push(v),
                    "sat" => days.sat.push(v),
                    "sun" => days.sun.push(v),

                    TWEET => {
                        // Tweeting in his own parsing dimension for the complete block.
                        let mut tweet = Tweet::new();

                        let mut key_value: HashMap<String, Vec<String>> = HashMap::new();

                        let mut last_key = TWEET.to_owned();

                        for line in block.split('\n') {
                            if !line.contains(']') {
                                key_value
                                    .entry(last_key.to_owned())
                                    .or_insert_with(Vec::new)
                                    .push(line.to_owned());

                                continue;
                            }

                            let mut kv = line.split(']');
                            let k = kv.next().unwrap().trim();
                            let v = kv.next().unwrap().trim();

                            last_key = k.to_owned();

                            key_value
                                .entry(k.to_owned())
                                .or_insert_with(Vec::new)
                                .push(v.to_owned());
                        }

                        let tweet_val = key_value.entry(TWEET.to_owned()).or_insert_with(Vec::new);
                        let text = tweet_val.join("");

                        let image_val = key_value.entry(IMAGE.to_owned()).or_insert_with(Vec::new);
                        let image = image_val.join("");

                        let state_val = key_value.entry(STATE.to_owned()).or_insert_with(Vec::new);
                        let state = state_val.join("");

                        tweet.text = text;
                        tweet.image = image;
                        tweet.state = state;

                        tweets.queue.push(tweet);
                    }

                    _ => continue,
                }
            }
        }

        println!("{:?}\n", tweets);
        println!("{:?}\n", days);
    } else {
        let mut files: Vec<DirEntry> = Vec::new();
        for entry in WalkDir::new(current_dir) {
            files.push(entry.unwrap());
        }

        let mut tweet = String::new();
        for file in files {
            let name = file.file_name().to_str().unwrap();
            let path = file.path().to_str().unwrap();
            let desc = name.replace('-', " ").replace('_', " ").replace('.', " ");

            tweet.push_str(&format!("{}] {}\n", TWEET, desc,));
            tweet.push_str(&format!("{}] {:?}\n", IMAGE, path));
            tweet.push_str(&format!("{}] {}\n", STATE, PENDING));
            tweet.push_str(&format!("\n"));
        }

        let mut file = std::fs::File::create(QUEUE_FILE).unwrap();
        file.write_all(tweet.trim().as_bytes()).unwrap();
    }

    println!("Done!");
}

fn remove_whitespace(s: &str) -> String {
    let sc: Vec<&str> = s.split_whitespace().collect();
    sc.join(" ")
}

// fn remove_whitespace_inplace(s: &mut String) {
//     s.retain(|c| !c.is_whitespace());
// }
