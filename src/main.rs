mod egg;

mod river;
use river::River;

use std::{
    env,
    io::{Read, Write},
};

use clap::{App, AppSettings::ArgRequiredElseHelp, Arg, SubCommand};
use self_update::cargo_crate_version;
use walkdir::WalkDir;

const QUEUE_FILE: &str = "river.queue.txt";

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

    // Create or @todo updates the river file by scanning image files on the folder.
    if let Some(matches) = matches.subcommand_matches("scan") {
        let name_as_text = matches.is_present("name");

        // Parse the River file, if exists.
        let mut content = String::new();
        if let Ok(mut file) = std::fs::File::open(QUEUE_FILE) {
            file.read_to_string(&mut content).unwrap();
        }

        let mut river = River::new();
        river.parse_load(content);

        // Look for new files and update River with the new files.
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
        let content = river.to_content(name_as_text);

        let mut file = std::fs::File::create(QUEUE_FILE).unwrap();
        file.write_all(content.trim().as_bytes()).unwrap();
    }

    // Using the river file,
    if let Some(_) = matches.subcommand_matches("start") {
        // Twitter authentication.
        egg::Config::load().await;

        // Parse the river file.
        let mut content = String::new();
        if let Ok(mut file) = std::fs::File::open(QUEUE_FILE) {
            file.read_to_string(&mut content).unwrap();
        }

        let mut river = River::new();
        river.parse_load(content);

        println!("{:?}\n", river.tweets);
        println!("{:?}\n", river.days);

        // @todo Send the river file to the Tweeting thread.
    }

    // Self updates.
    if let Some(_matches) = matches.subcommand_matches("update") {
        println!();

        match update() {
            Ok(_) => {}
            Err(_) => println!("Error updating."),
        }
    }

    println!();
    println!("File {} generated\n", QUEUE_FILE);
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
