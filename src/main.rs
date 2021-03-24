mod egg;

mod river;
use river::River;

use egg_mode::{
    media::{media_types, upload_media},
    tweet::DraftTweet,
};

use std::{
    env,
    io::{Read, Write},
    path::PathBuf,
    thread, u32, u64,
};

use chrono::{Datelike, Duration, Local, Timelike};
use clap::{App, AppSettings::ArgRequiredElseHelp, Arg, SubCommand};
use self_update::cargo_crate_version;
use walkdir::WalkDir;

const QUEUE_FILE: &str = "river.queue.txt";

#[tokio::main]
async fn main() {
    // CLAP

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

    // SCAN

    // Create or updates the river file by scanning image files on the folder.
    if let Some(matches) = matches.subcommand_matches("scan") {
        let name_as_text = matches.is_present("name");

        // Parse the River file, if exists.
        let mut content = String::new();
        if let Ok(mut file) = std::fs::File::open(QUEUE_FILE) {
            file.read_to_string(&mut content).unwrap();
        }

        let mut river = River::new();
        river.parse_load(content);

        // Update the River file with new found images.
        let current_dir = env::current_dir().unwrap();

        let only_images = &["bmp", "gif", "jpeg", "jpg", "png"];
        for entry in WalkDir::new(current_dir) {
            let entry = entry.unwrap();

            let ext = match entry.path().extension() {
                Some(st) => st.to_str().unwrap(),
                None => continue,
            };

            if !only_images.contains(&ext.to_lowercase().as_str()) {
                continue;
            }

            let image = entry.path().to_str().unwrap();
            river.append_new(image.to_owned());
        }

        // Create the River file.
        let content = river.to_text(name_as_text);
        write_file(content, QUEUE_FILE);

        println!();
        println!("File {} generated\n", QUEUE_FILE);
    }

    // START

    // Using the river file, lets start waiting to tweet.
    if let Some(_) = matches.subcommand_matches("start") {
        // Twitter authentication.

        let config = egg::Config::load().await;

        // Parse the river file.

        let mut content = String::new();
        if let Ok(mut file) = std::fs::File::open(QUEUE_FILE) {
            file.read_to_string(&mut content).unwrap();
        }

        let mut river = River::new();
        river.parse_load(content);

        // Tweet at the next hour.

        // @todo There is probably a better way to do this.
        let mut queue = river.tweets.queue.clone();

        for tweet in &mut queue {
            // Skip already tweeted tweets.
            if tweet.state.to_lowercase().trim() != river::PENDING {
                continue;
            }

            // For the current day, which is the closest hour?
            let local = Local::now();
            let today = local.weekday();

            let hour = local.hour();
            let hours = river.get_day(&today.to_string().as_str());

            let mut schedule: Vec<u32> = hours
                .split(" ")
                .filter_map(|x| match x.parse::<u32>() {
                    Ok(x) => Some(x),
                    Err(_) => None,
                })
                .filter(|x| x >= &hour && *x <= 24)
                .collect();

            schedule.sort();
            schedule.dedup();

            let count = schedule.iter().len();
            let mut schedule_today = schedule.iter();

            // Some info.

            println!("{}, {} hour", today, hour);

            // Is there a next hour?
            match schedule_today.next() {
                Some(hour) => {
                    let mut hour = *hour;
                    let mut minute = 0;

                    if hour > 23 {
                        hour = 23;
                        minute = 59;
                    }

                    // Time until the next hour.
                    let now = Local::now();
                    let next_hour = now.date().and_hms(hour, minute, 0);
                    let duration = next_hour.signed_duration_since(now).to_std();

                    // Tweeting or waiting.
                    match duration {
                        Ok(duration) => {
                            let secs = duration.as_secs() + 1;
                            let mins = secs / 60;

                            println!(" Tweeting at {}:00", hour);
                            println!(" {} pending today", count);
                            println!(" Waiting {} minutes until the next tweet", mins);

                            thread::sleep(std::time::Duration::from_secs(secs));
                        }
                        Err(_) => {
                            let image_path = PathBuf::from(tweet.image.to_owned());
                            let mut post = DraftTweet::new(tweet.text.to_owned());

                            let media_type = match image_path
                                .as_path()
                                .extension()
                                .and_then(|os| os.to_str())
                                .unwrap_or("")
                            {
                                "jpg" | "jpeg" => media_types::image_jpg(),
                                "gif" => media_types::image_gif(),
                                "png" => media_types::image_png(),
                                "webp" => media_types::image_webp(),
                                "mp4" => media_types::video_mp4(),
                                _ => {
                                    eprintln!(" Format not recognized, must be one of jpg, jpeg, gif, png, webp, mp4");
                                    std::process::exit(1);
                                }
                            };

                            let bytes = std::fs::read(image_path).unwrap();
                            let handle = upload_media(&bytes, &media_type, &config.token)
                                .await
                                .unwrap();

                            post.add_media(handle.id.clone());

                            let sent = post.send(&config.token).await;

                            match sent {
                                Ok(_) => {
                                    tweet.state = river::SENT.to_owned();

                                    println!(" Sent!")
                                }
                                Err(_) => println!(" Tweet error!"),
                            }

                            let now = Local::now();
                            let mins = 60 - now.minute();
                            let secs = ((mins * 60) + 1) as u64;

                            println!(" Waiting {} minutes until the next hour", mins);

                            thread::sleep(std::time::Duration::from_secs(5));
                            // thread::sleep(std::time::Duration::from_secs(secs));
                        }
                    }
                }

                // No tweets today.
                None => {
                    let now = Local::now();
                    let tomorrow = (now + Duration::days(1)).date().and_hms(0, 0, 0);
                    let duration = tomorrow.signed_duration_since(now).to_std().unwrap();

                    let secs = duration.as_secs() + 1;
                    let hours = secs / 60 / 60;

                    println!(" No tweets today");
                    println!(" Waiting {} hours until tomorrow", hours);

                    thread::sleep(std::time::Duration::from_secs(secs));
                }
            }

            println!();
        }

        // Update the current file.

        river.tweets.queue = queue;

        let content = river.to_text(false);
        write_file(content, QUEUE_FILE);
    }

    // UPDATE

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

fn write_file(content: String, filepath: &str) {
    let mut file = std::fs::File::create(filepath).unwrap();
    file.write_all(content.trim().as_bytes()).unwrap();
}
