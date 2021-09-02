use regex::Regex;
use reqwest::blocking::Client;
use serde_json::Value;
use std::process::exit;

fn main() {
    let prepend = "https://www.youtube.com/results?search_query=";
    let query = "take on me";
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
    let video_data = json
        .get("contents")
        .unwrap()
        .get("twoColumnSearchResultsRenderer")
        .unwrap()
        .get("primaryContents")
        .unwrap()
        .get("sectionListRenderer")
        .unwrap()
        .get("contents")
        .unwrap()
        .get(0)
        .unwrap()
        .get("itemSectionRenderer")
        .unwrap()
        .get("contents")
        .unwrap();
    for video_datum in video_data.as_array().unwrap() {
        if let Some(data) = video_datum.get("videoRenderer") {
            let title = data
                .get("title")
                .unwrap()
                .get("runs")
                .unwrap()
                .get(0)
                .unwrap()
                .get("text")
                .unwrap();
            dbg!(title);
        }
    }
}
