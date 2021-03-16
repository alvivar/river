mod egg;

use std::{
    collections::HashMap,
    env,
    io::{Read, Write},
};

use clap::{App, AppSettings::ArgRequiredElseHelp, Arg, SubCommand};
use self_update::cargo_crate_version;
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
    // Command Line
    let matches = App::new("river")
        .version(cargo_crate_version!())
        .about("Check out github.com/alvivar/river for more info!")
        .setting(ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("scan")
            .about("Scans the current dir and creates (or updates) the 'river.queue.txt' file with all images found, ready to be tweeted")
            .arg(Arg::with_name("name")
                .short("n")
                .help("Use the file name as text in the river file")
            )
        )
        .subcommand(SubCommand::with_name("start")
            .about("Starts tweeting from the 'river.queue.txt' file")
        )
        .subcommand(SubCommand::with_name("update")
            .about("Self updates to the latest release on GitHub")
        )
        .get_matches();

    // Clap

    // Start waiting and posting accordinly to the queue file on the folder.

    // Create the river file by scanning image files on the folder.
    if let Some(matches) = matches.subcommand_matches("scan") {
        let has_name = matches.is_present("name");

        let current_dir = env::current_dir().unwrap();

        let mut files: Vec<DirEntry> = Vec::new();
        for entry in WalkDir::new(current_dir) {
            files.push(entry.unwrap());
        }

        let mut content = String::new();

        content.push_str("schedule]\n\n");

        content.push_str("# Custom times and tags per day, as much as you like.\n\n");

        content.push_str("mon]  8a   10a  #monday\n");
        content.push_str("tue]  9a   11a  #tuesday\n");
        content.push_str("wed]  10a  12p  #wednesday\n");
        content.push_str("thu]  11a  1p   #thursday\n");
        content.push_str("fri]  12p  2p   #friday\n");
        content.push_str("sat]  1p   3p   #saturday\n");
        content.push_str("sun]  2p   4p   #sunday\n\n\n");

        content.push_str("tweets]\n\n");

        content.push_str("# tweet] Message to tweet! #cool\n");
        content.push_str("# image] Image-to.tweet\n");
        content.push_str("# state] Pending | Sent | Error <- Handled by the application.\n\n");

        content.push_str("# All fields are optional.\n");
        content.push_str("# If you want you can send a single tweet] or a single image].\n\n");

        for file in files {
            let name = file.file_name().to_str().unwrap();
            let path = file.path().to_str().unwrap();

            let mut desc = name.replace('-', " ").replace('_', " ").replace('.', " ");
            if !has_name {
                desc = "".to_owned();
            }

            content.push_str(&format!("{}] {}\n", TWEET, desc,));
            content.push_str(&format!("{}] {:?}\n", IMAGE, path));
            content.push_str(&format!("{}] {}\n", STATE, PENDING));
            content.push_str(&format!("\n"));
        }

        let mut file = std::fs::File::create(QUEUE_FILE).unwrap();
        file.write_all(content.trim().as_bytes()).unwrap();
    }

    // Using the
    if let Some(_) = matches.subcommand_matches("start") {
        // Twitter authentication.
        egg::Config::load().await;

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

                            let tweet_val =
                                key_value.entry(TWEET.to_owned()).or_insert_with(Vec::new);
                            let text = tweet_val.join("");

                            let image_val =
                                key_value.entry(IMAGE.to_owned()).or_insert_with(Vec::new);
                            let image = image_val.join("");

                            let state_val =
                                key_value.entry(STATE.to_owned()).or_insert_with(Vec::new);
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
        }
    }

    // Self updates.
    if let Some(_matches) = matches.subcommand_matches("update") {
        println!();

        match update() {
            Ok(_) => {}
            Err(_) => println!("Error updating."),
        }
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

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("alvivar")
        .repo_name("river")
        .bin_name("river")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    println!("Current version... v{}", status.version());

    Ok(())
}
