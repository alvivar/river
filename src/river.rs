use std::collections::HashMap;

pub const TWEET: &str = "tweet";
pub const IMAGE: &str = "image";
pub const STATE: &str = "state";

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
pub struct Tweets {
    queue: Vec<Tweet>,
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

    pub fn parse(&mut self, content: String) {
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
}

fn append_new(tweets: &mut Vec<Tweet>, image: String) {
    match tweets.iter_mut().find(|ref p| image <= p.image) {
        Some(pivot) => {
            // We don't need to update the one we already have.
            return;
        }
        // o/w insert a new leaf at the end
        None => {
            let mut tweet = Tweet::new();
            tweet.image = image;

            tweets.push(tweet);
        }
    }
}

fn remove_whitespace(s: &str) -> String {
    let sc: Vec<&str> = s.split_whitespace().collect();
    sc.join(" ")
}
