use std::{
    env,
    io::{Read, Write},
    path::PathBuf,
    process::exit,
    u32, u64,
};

use tokio;
use tokio::time;

use egg_mode::{
    media::{media_types, upload_media},
    tweet::DraftTweet,
};

use chrono::{DateTime, Datelike, Duration, Local, Timelike};
use clap::{App, AppSettings::ArgRequiredElseHelp, Arg, SubCommand};
use self_update::cargo_crate_version;
use walkdir::WalkDir;

mod egg;
mod river;
use river::River;

const RIVER_FILE: &str = "river.queue.txt";

#[tokio::main]
async fn main() {
    // Clap

    let matches = App::new("River")
        .version(cargo_crate_version!())
        .about("Tool that schedules tweets with images\ngithub.com/alvivar/river")
        .setting(ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("scan")
            .about("Scans the current dir and creates (or updates) river.queue.txt with all images found")
            .arg(Arg::with_name("name")
                .short("n")
                .help("Use the name as text")
            )
            .arg(Arg::with_name("instructions")
                .short("i")
                .help("Include instructions")
            )
        )
        .subcommand(SubCommand::with_name("start")
            .about("Starts tweeting from river.queue.txt")
        )
        .subcommand(SubCommand::with_name("update")
            .about("Self updates from the latest github release")
        )
        .get_matches();

    // Global

    let dir = env::current_dir().unwrap();
    let river_file = PathBuf::new().join(&dir).join(RIVER_FILE);

    // Scan

    // Create or updates the River file by scanning image files on the folder.
    if let Some(matches) = matches.subcommand_matches("scan") {
        let name_as_text = matches.is_present("name");
        let include_help = matches.is_present("instructions");

        // Parse the River file, if exists.
        let content = read_file(&river_file);
        let mut river = River::new();
        river.parse_load(content);

        // Update the River file with new found images.
        let only_images = &["bmp", "gif", "jpeg", "jpg", "png"];
        for entry in WalkDir::new(&dir) {
            let entry = entry.unwrap();

            let ext = match entry.path().extension() {
                Some(st) => st.to_str().unwrap(),
                None => continue,
            };

            if !only_images.contains(&ext.to_lowercase().as_str()) {
                continue;
            }

            let image = entry.path().to_str().unwrap();
            river.append_new(image);
        }

        // Create the River file.
        let content = river.to_text(name_as_text, include_help);
        write_file(content, &river_file);

        let count = river
            .tweets
            .iter()
            // .filter(|x| x.state.trim().to_lowercase() != river::SENT)
            .count();

        println!("Hi!\n");
        println!("{} updated", RIVER_FILE);
        println!("{} tweets pending\n", count);
    }

    // Start

    // Using the River file, lets start waiting to tweet.
    if let Some(_) = matches.subcommand_matches("start") {
        let auth = egg::Config::load().await;

        // Parse the River file, if exists.
        let content = read_file(&river_file);
        let mut river = River::new();
        river.parse_load(content);

        // Tweet at the next hour.
        let mut queue = river.tweets.clone(); // @todo There is probably a better way to do this.

        for tweet in &mut queue {
            // Continue if there is a valid date, or is pending.
            match DateTime::parse_from_rfc2822(&tweet.state.to_string()) {
                Ok(_) => continue,
                Err(_) => {
                    if tweet.state.trim().to_lowercase() == river::ERROR {
                        println!("Error on the {} file, take a look", RIVER_FILE);

                        break;
                    } else {
                        tweet.state = river::ERROR.to_owned();
                        println!("Error on the {} file, take a look", RIVER_FILE);

                        break;
                    }
                }
            }

            // @todo THIS IS WRONG ^

            // On the current day, which is the closest hour?
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
            let mut today_schedule = schedule.iter();

            // Some info.
            println!("{}, {} hour", today, hour);

            // Is there a next hour?
            match today_schedule.next() {
                Some(hour) => {
                    let mut hour = *hour;
                    let mut minute = 0;
                    let mut secs = 0;

                    if hour > 23 {
                        hour = 23;
                        minute = 59;
                        secs = 59;
                    }

                    // Time to the next hour.
                    let now = Local::now();
                    let next_hour = now.date().and_hms(hour, minute, secs);
                    let duration = next_hour.signed_duration_since(now).to_std();

                    // Tweeting or waiting.
                    match duration {
                        Ok(duration) => {
                            let secs = duration.as_secs() + 1;
                            let mins = secs / 60;

                            println!(" Next tweet at {}:00", hour);
                            println!(" {} pending today", count);
                            println!(" Waiting {} minutes to tweet", mins);

                            time::delay_for(time::Duration::from_secs(secs)).await
                        }
                        // The OutOfRangeError means that duration is less than 1 hour.
                        Err(_) => {
                            let text = tweet.text.to_owned();
                            let image = tweet.image.to_owned();
                            let image_path = PathBuf::from(&image);

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
                                    eprintln!(" Image format not recognized, must be one of [jpg, jpeg, gif, png, webp, mp4]");

                                    exit(1);
                                }
                            };

                            let mut post = DraftTweet::new(text.to_owned());

                            let data = std::fs::read(image_path.to_owned()).unwrap();
                            let handle =
                                upload_media(&data, &media_type, &auth.token).await.unwrap();

                            post.add_media(handle.id.clone());

                            println!(" Trying...");
                            println!("  > {}", text);
                            println!("  > {:?}", image_path);

                            let now = Local::now();

                            match post.send(&auth.token).await {
                                Ok(_) => {
                                    // Update the River file.
                                    tweet.state = now.to_rfc2822();

                                    river.last = now.to_rfc2822();
                                    river.update_state(image, now.to_rfc2822());
                                    let content = river.to_text(false, false);
                                    write_file(content, &river_file);

                                    println!(" Sent!");
                                }
                                Err(_) => {
                                    println!(" Tweet error!");

                                    exit(1);
                                }
                            }

                            let mins = 60 - now.minute();
                            let secs = ((mins * 60) + 1) as u64;

                            println!(" Waiting {} minutes until the next hour", mins);

                            time::delay_for(time::Duration::from_secs(secs)).await
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

                    time::delay_for(time::Duration::from_secs(secs)).await
                }
            }

            println!();
        }

        // Update the River file.
        river.tweets = queue;
        let content = river.to_text(false, false);
        write_file(content, &river_file);
    }

    // Update

    // Self updates.
    if let Some(_matches) = matches.subcommand_matches("update") {
        println!("Hm...\n");

        match update() {
            Ok(_) => println!("Updated!"),
            Err(_) => println!("Error updating!"),
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

fn write_file(content: String, filepath: &PathBuf) {
    let mut file = std::fs::File::create(filepath).unwrap();
    file.write_all(content.trim().as_bytes()).unwrap();
}

fn read_file(filepath: &PathBuf) -> String {
    let mut content = String::new();
    if let Ok(mut file) = std::fs::File::open(filepath) {
        file.read_to_string(&mut content).unwrap();
    }

    content
}
