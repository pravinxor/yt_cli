use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::get;
use serde_json::{from_str, Value};

lazy_static! {
    static ref MATCHER: Regex = Regex::new("var ytInitialData = .*}}}}}}").unwrap();
}

const PREPEND: &str = "https://www.youtube.com/results?search_query=";
pub(crate) fn get_source(query: &str) -> reqwest::Result<String> {
    get(format!("{}{}", PREPEND, query))?.text()
}

pub(crate) fn find_json(source: &str) -> core::result::Result<Value, String> {
    let json_string = match matcher.find(source) {
        None => return Err(format!("matcher could not find JSON in -> {}", source)),
        Some(json) => json,
    }
    .as_str()
    .to_owned()
        + "}";
    let json_str = json_string.trim_start_matches("var ytInitialData = ");
    return match from_str(json_str) {
        Ok(serde_json) => Ok(serde_json),
        Err(error) => Err(error.to_string()),
    };
}
