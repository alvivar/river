// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.

// If a copy of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code)] // print_tweet is not mandatory, so.

use egg_mode;
use std::{
    io::{Read, Write},
    process::exit,
};
use yansi::Paint;

const TWITTER_FILE: &str = "river.twitter.auth.txt";

pub struct Config {
    pub token: egg_mode::Token,
    pub user_id: u64,
    pub username: String,
}

impl Config {
    pub async fn load() -> Self {
        if let Some(config) = Config::load_inner().await {
            return config;
        }

        if let Some(config) = Config::load_inner().await {
            return config;
        } else {
            println!("You may need to learn to copy paste stuff.");
            println!("Good luck on your life.");

            std::io::stdin().read(&mut [0]).unwrap();
            exit(0);
        }
    }

    async fn load_inner() -> Option<Self> {
        // Include these two files in the same path as the main.rs.
        let consumer_key = include_str!("consumer_key").trim();
        let consumer_secret = include_str!("consumer_secret").trim();

        let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);

        let mut config = String::new();
        let user_id: u64;
        let username: String;
        let token: egg_mode::Token;

        if let Ok(mut file) = std::fs::File::open(TWITTER_FILE) {
            file.read_to_string(&mut config).unwrap();
            let mut lines = config.split('\n');

            user_id = u64::from_str_radix(&lines.next().unwrap(), 10).unwrap();

            username = lines.next().unwrap().to_string();

            let access_token = egg_mode::KeyPair::new(
                lines.next().unwrap().to_string(),
                lines.next().unwrap().to_string(),
            );

            token = egg_mode::Token::Access {
                consumer: con_token,
                access: access_token,
            };

            if let Err(_) = egg_mode::auth::verify_tokens(&token).await {
                println!("We've hit an error using your old tokens\n");
                std::fs::remove_file(TWITTER_FILE).unwrap();
            } else {
                println!("Welcome back @{}!\n", username);
            }
        } else {
            let request_token = egg_mode::auth::request_token(&con_token, "oob")
                .await
                .unwrap();

            println!("We need to authenticate with twitter");
            println!("Go to the following url, sign in, and come back with the #pin\n");

            println!(
                "{}\n\nPaste the #pin here and [hit] enter:",
                egg_mode::auth::authorize_url(&request_token)
            );

            let mut pin = String::new();
            std::io::stdin().read_line(&mut pin).unwrap();
            println!();

            let auth = match egg_mode::auth::access_token(con_token, &request_token, pin).await {
                Ok(tok) => tok,
                Err(_) => {
                    println!("Hm, that #pin didn't work\n");
                    return None;
                }
            };

            token = auth.0;
            user_id = auth.1;
            username = auth.2;

            match token {
                egg_mode::Token::Access {
                    access: ref access_token,
                    ..
                } => {
                    config.push_str(&format!("{}", user_id));
                    config.push('\n');
                    config.push_str(&username);
                    config.push('\n');
                    config.push_str(&access_token.key);
                    config.push('\n');
                    config.push_str(&access_token.secret);
                }

                _ => unreachable!(),
            }

            let mut file = std::fs::File::create(TWITTER_FILE).unwrap();
            file.write_all(config.as_bytes()).unwrap();

            println!("Welcome @{}!\n", username);
        }

        // @todo Is there a better way to query whether a file exists?
        if std::fs::metadata(TWITTER_FILE).is_ok() {
            Some(Config {
                token,
                user_id,
                username,
            })
        } else {
            None
        }
    }
}

pub fn print_tweet(tweet: &egg_mode::tweet::Tweet) {
    if let Some(ref user) = tweet.user {
        println!(
            "{} (@{}) posted at {}",
            Paint::blue(&user.name),
            Paint::bold(Paint::blue(&user.screen_name)),
            tweet.created_at.with_timezone(&chrono::Local)
        );
    }

    if let Some(ref screen_name) = tweet.in_reply_to_screen_name {
        println!("➜ in reply to @{}", Paint::blue(screen_name));
    }

    if let Some(ref status) = tweet.retweeted_status {
        println!("{}", Paint::red("Retweet ➜"));
        print_tweet(status);
        return;
    } else {
        println!("{}", Paint::green(&tweet.text));
    }

    if let Some(source) = &tweet.source {
        println!("➜ via {} ({})", source.name, source.url);
    }

    if let Some(ref place) = tweet.place {
        println!("➜ from: {}", place.full_name);
    }

    if let Some(ref status) = tweet.quoted_status {
        println!("{}", Paint::red("➜ Quoting the following status:"));
        print_tweet(status);
    }

    if !tweet.entities.hashtags.is_empty() {
        println!("➜ Hashtags contained in the tweet:");
        for tag in &tweet.entities.hashtags {
            println!("  {}", tag.text);
        }
    }

    if !tweet.entities.symbols.is_empty() {
        println!("➜ Symbols contained in the tweet:");
        for tag in &tweet.entities.symbols {
            println!("  {}", tag.text);
        }
    }

    if !tweet.entities.urls.is_empty() {
        println!("➜ URLs contained in the tweet:");
        for url in &tweet.entities.urls {
            if let Some(expanded_url) = &url.expanded_url {
                println!("  {}", expanded_url);
            }
        }
    }

    if !tweet.entities.user_mentions.is_empty() {
        println!("➜ Users mentioned in the tweet:");
        for user in &tweet.entities.user_mentions {
            println!("  {}", Paint::bold(Paint::blue(&user.screen_name)));
        }
    }

    if let Some(ref media) = tweet.extended_entities {
        println!("➜ Media attached to the tweet:");
        for info in &media.media {
            println!("  A {:?}", info.media_type);
        }
    }
}
