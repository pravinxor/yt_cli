use regex::Regex;
use reqwest::blocking::Client;
use serde_json::Value;
use std::process::exit;

fn main() {
    let prepend = "https://www.youtube.com/results?search_query=";
    let query = "shrek memes";
    let matcher = Regex::new("var ytInitialData = .*}}}}}}").unwrap();
    let client = Client::new();
    let data = &client
        .get(format!("{}{}", prepend, query))
        .send()
        .unwrap()
        .text()
        .unwrap();

    let json = match matcher.find(data) {
        None => {
            exit(-1);
        }
        Some(json) => (json.as_str().to_owned() + "}"),
    };
    let json = json.trim_start_matches("var ytInitialData = ");
    let json: Value = serde_json::from_str(json).unwrap();
    dbg!(json);
}
