use std::{collections::HashMap, path::Path};

pub const TWEET: &str = "tweet";
pub const IMAGE: &str = "image";
pub const STATE: &str = "state";

pub const PENDING: &str = "pending";
pub const SENT: &str = "sent";
pub const ERROR: &str = "error";

#[derive(Debug)]
pub struct Days {
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Tweets {
    pub queue: Vec<Tweet>,
}

impl Tweets {
    pub fn new() -> Tweets {
        let queue = Vec::new();

        Tweets { queue }
    }
}

pub struct River {
    pub days: Days,
    pub tweets: Tweets,
}

impl River {
    pub fn new() -> River {
        let days = Days::new();
        let tweets = Tweets::new();

        River { days, tweets }
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

                let v = remove_whitespace(v);

                match k.to_lowercase().as_str() {
                    // It's a day?
                    "mon" => self.days.mon.push(v),
                    "tue" => self.days.tue.push(v),
                    "wed" => self.days.wed.push(v),
                    "thu" => self.days.thu.push(v),
                    "fri" => self.days.fri.push(v),
                    "sat" => self.days.sat.push(v),
                    "sun" => self.days.sun.push(v),

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

                        self.tweets.queue.push(tweet);
                    }

                    _ => continue,
                }
            }
        }
    }

    pub fn append_new(&mut self, image: String) {
        match self
            .tweets
            .queue
            .iter_mut()
            .find(|ref p| image.trim() == p.image.trim())
        {
            // This place is useful to update because it exists.
            Some(_) => {}

            // New entry!
            None => {
                let mut tweet = Tweet::new();
                tweet.image = image;

                self.tweets.queue.push(tweet);
            }
        }
    }

    pub fn to_content(&self, with_name_as_text: bool) -> String {
        let mut content = String::new();

        content.push_str("schedule]\n\n");

        content.push_str("# Custom times and tags per day, as much as you like.\n\n");

        // @todo Schedule from the file or default.

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

        for file in &self.tweets.queue {
            let mut text = file.text.to_owned();
            let image = &file.image;

            let state = if file.state.len() > 0 {
                let state = match file.state.to_lowercase().as_str() {
                    PENDING => "Pending",
                    SENT => "Sent",
                    ERROR => "Error",
                    _ => PENDING,
                };

                state.to_string()
            } else {
                PENDING.to_owned()
            };

            if with_name_as_text && text.len() < 1 {
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
