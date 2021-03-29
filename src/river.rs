use std::{collections::HashMap, path::Path};

use chrono::DateTime;

pub const TWEET: &str = "tweet";
pub const IMAGE: &str = "image";
pub const STATE: &str = "state";

pub const READY: &str = "ready";
pub const ERROR: &str = "error";

#[derive(Debug)]
pub struct Days {
    mon: String,
    tue: String,
    wed: String,
    thu: String,
    fri: String,
    sat: String,
    sun: String,
}

impl Days {
    pub fn new() -> Days {
        let mon = String::new();
        let tue = String::new();
        let wed = String::new();
        let thu = String::new();
        let fri = String::new();
        let sat = String::new();
        let sun = String::new();

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

#[derive(Debug, Clone)]
pub struct Tweet {
    pub text: String,
    pub image: String,
    pub state: String,
}

impl Tweet {
    pub fn new() -> Tweet {
        let text = String::new();
        let image = String::new();
        let state = String::new();

        Tweet { text, image, state }
    }
}

pub struct River {
    pub days: Days,
    pub last: String,
    pub tweets: Vec<Tweet>,
}

impl River {
    pub fn new() -> River {
        let days = Days::new();
        let latest = String::new();
        let tweets = Vec::new();

        River {
            days,
            last: latest,
            tweets,
        }
    }

    pub fn get_day(&self, day: &str) -> String {
        let chosen = match day.to_lowercase().as_str() {
            "mon" => self.days.mon.to_owned(),
            "tue" => self.days.tue.to_owned(),
            "wed" => self.days.wed.to_owned(),
            "thu" => self.days.thu.to_owned(),
            "fri" => self.days.fri.to_owned(),
            "sat" => self.days.sat.to_owned(),
            "sun" => self.days.sun.to_owned(),
            _ => String::new(),
        };

        chosen
    }

    pub fn parse_load(&mut self, content: String) {
        // Two lines defines our sections.
        for block in content.split("\n\n") {
            // One line is the key and the value.
            for line in block.split('\n') {
                if !line.contains(']') {
                    continue;
                }

                let mut kv = line.split(']');
                let k = kv.next().unwrap().trim();
                let v = kv.next().unwrap().trim();

                let val = remove_whitespace(v);

                match k.to_lowercase().as_str() {
                    // The last tweet sent.
                    "last" => self.last = val,

                    // It's a day?
                    "mon" => self.days.mon = val,
                    "tue" => self.days.tue = val,
                    "wed" => self.days.wed = val,
                    "thu" => self.days.thu = val,
                    "fri" => self.days.fri = val,
                    "sat" => self.days.sat = val,
                    "sun" => self.days.sun = val,

                    // Or a tweet?
                    TWEET => {
                        // Tweeting is parsed in his own block.
                        let mut tweet = Tweet::new();
                        let mut key_value: HashMap<String, Vec<String>> = HashMap::new();
                        let mut last_key = TWEET.to_owned();

                        for line in block.split('\n') {
                            // Tweets can be multiline.
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

                        self.tweets.push(tweet);
                    }

                    _ => continue,
                }
            }
        }
    }

    pub fn append_new(&mut self, image: &str) {
        match self
            .tweets
            .iter_mut()
            .find(|ref p| image.trim() == p.image.trim())
        {
            // This place is useful to update if exists.
            Some(_) => {}

            // New entry!
            None => {
                let mut tweet = Tweet::new();
                tweet.image = image.to_owned();

                self.tweets.push(tweet);
            }
        }
    }

    pub fn update_state(&mut self, image: String, state: String) {
        match self
            .tweets
            .iter_mut()
            .find(|ref p| image.trim() == p.image.trim())
        {
            // Update if exists.
            Some(tweet) => tweet.state = state,

            // New entry!
            None => {}
        }
    }

    pub fn to_text(&self, name_as_text: bool, unsent_ready: bool, include_help: bool) -> String {
        // Default schedule if doesn't exist.
        let mon = match self.days.mon.len() > 0 {
            true => self.days.mon.to_owned(),
            false => "".to_owned(),
        };

        let tue = match self.days.tue.len() > 0 {
            true => self.days.tue.to_owned(),
            false => "".to_owned(),
        };

        let wed = match self.days.wed.len() > 0 {
            true => self.days.wed.to_owned(),
            false => "".to_owned(),
        };

        let thu = match self.days.thu.len() > 0 {
            true => self.days.thu.to_owned(),
            false => "".to_owned(),
        };

        let fri = match self.days.fri.len() > 0 {
            true => self.days.fri.to_owned(),
            false => "".to_owned(),
        };

        let sat = match self.days.sat.len() > 0 {
            true => self.days.sat.to_owned(),
            false => "".to_owned(),
        };

        let sun = match self.days.sun.len() > 0 {
            true => self.days.sun.to_owned(),
            false => "".to_owned(),
        };

        let last = match self.last.len() > 0 {
            true => self.last.to_owned(),
            false => "".to_owned(),
        };

        let mut content = String::new();

        content.push_str("schedule]\n\n");

        if include_help {
            content.push_str("# Times using 24-hour clock and daily tags, as much as you like.\n");
            content.push_str("# sun] 10 13 16 #sundaytag\n\n");
        }

        content.push_str(format!("last] {}\n\n", last).as_str());

        content.push_str(format!("mon] {}\n", mon).as_str());
        content.push_str(format!("tue] {}\n", tue).as_str());
        content.push_str(format!("wed] {}\n", wed).as_str());
        content.push_str(format!("thu] {}\n", thu).as_str());
        content.push_str(format!("fri] {}\n", fri).as_str());
        content.push_str(format!("sat] {}\n", sat).as_str());
        content.push_str(format!("sun] {}\n\n\n", sun).as_str());

        content.push_str("tweets]\n\n");

        if include_help {
            content.push_str("# tweet] Message to tweet! #cool\n");
            content.push_str("# image] Image-to.tweet\n");
            content.push_str("# state] pending | sent | error <- Handled by the application.\n\n");

            content.push_str("# All fields are optional.\n");
            content.push_str("# If you want you can send a single tweet] or a single image].\n\n");
        }

        // For each tweet.
        for file in &self.tweets {
            let mut text = file.text.to_owned();
            let image = &file.image;

            let state = if file.state.len() > 0 {
                let file_state = file.state.to_owned().trim().to_lowercase();

                let try_date = match DateTime::parse_from_rfc2822(&file_state) {
                    Ok(_) => file_state.to_owned(),
                    Err(_) => ERROR.to_owned(),
                };

                let state = match file_state.as_str() {
                    READY => READY,
                    ERROR => ERROR,
                    _ => try_date.as_str(),
                };

                state.to_owned()
            } else {
                if unsent_ready {
                    READY.to_owned()
                } else {
                    "".to_owned()
                }
            };

            if name_as_text && text.len() < 1 {
                let image_path = Path::new(&image);
                let name = image_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                text = name.replace('-', " ").replace('_', " ").replace('.', " ");
            }

            content.push_str(&format!("{}] {}\n", TWEET, text,));
            content.push_str(&format!("{}] {}\n", IMAGE, image));
            content.push_str(&format!("{}] {}\n", STATE, state));
            content.push_str(&format!("\n"));
        }

        content
    }
}

fn remove_whitespace(s: &str) -> String {
    let sc: Vec<&str> = s.split_whitespace().collect();
    sc.join(" ")
}

// fn remove_whitespace_inplace(s: &mut String) {
//     s.retain(|c| !c.is_whitespace());
// }
