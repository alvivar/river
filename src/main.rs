mod egg;

mod river;
use river::River;

use std::{
    env,
    io::{Read, Write},
};

use clap::{App, AppSettings::ArgRequiredElseHelp, Arg, SubCommand};
use self_update::cargo_crate_version;
use walkdir::{DirEntry, WalkDir};

const QUEUE_FILE: &str = "river.queue.txt";

const PENDING: &str = "Pending";
// const SENT: &str = "Send";
// const ERROR: &str = "Error";

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
        let has_name = matches.is_present("name");

        // Parse the river file, if exists.
        let mut content = String::new();
        if let Ok(mut file) = std::fs::File::open(QUEUE_FILE) {
            file.read_to_string(&mut content).unwrap();
        }

        let mut river = River::new();
        river.parse(content);

        // Look for new files.
        let current_dir = env::current_dir().unwrap();

        let mut files: Vec<DirEntry> = Vec::new();
        for entry in WalkDir::new(current_dir) {
            // @todo Check for images only.
            files.push(entry.unwrap());
        }

        // @todo Update River with the new files.

        // @todo Create the River file.

        // Content
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

            content.push_str(&format!("{}] {}\n", river::TWEET, desc,));
            content.push_str(&format!("{}] {:?}\n", river::IMAGE, path));
            content.push_str(&format!("{}] {}\n", river::STATE, PENDING));
            content.push_str(&format!("\n"));
        }

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
        river.parse(content);

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

    println!("Done!");
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
