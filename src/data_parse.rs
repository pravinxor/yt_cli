use crate::tools::clean_quotes;
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
    let json_string = match MATCHER.find(source) {
        None => return Err(format!("matcher could not find JSON in -> {}", source)),
        Some(json) => json,
    }
    .as_str()
    .to_owned()
        + "}";
    let json_str = json_string.trim_start_matches("var ytInitialData = ");
    match from_str(json_str) {
        Ok(serde_json) => Ok(serde_json),
        Err(error) => Err(error.to_string()),
    }
}
pub(crate) fn get_video_vec(source: &Value) -> Option<&Vec<Value>> {
    source
        .get("contents")?
        .get("twoColumnSearchResultsRenderer")?
        .get("primaryContents")?
        .get("sectionListRenderer")?
        .get("contents")?
        .get(0)?
        .get("itemSectionRenderer")?
        .get("contents")?
        .as_array()
}

pub(crate) fn get_title(source: &Value) -> Option<String> {
    let title = source
        .get("videoRenderer")?
        .get("title")?
        .get("runs")?
        .get(0)?
        .get("text")?
        .to_string();
    Some(clean_quotes(&title))
}
pub(crate) fn get_view_count(source: &Value) -> Option<String> {
    let view_count = source
        .get("videoRenderer")?
        .get("viewCountText")?
        .get("simpleText")?
        .to_string();
    Some(clean_quotes(&view_count))
}
pub(crate) fn get_length(source: &Value) -> Option<String> {
    let length = source
        .get("videoRenderer")?
        .get("lengthText")?
        .get("simpleText")?
        .to_string();
    Some(clean_quotes(&length))
}
pub(crate) fn get_owner(source: &Value) -> Option<String> {
    let owner = source
        .get("videoRenderer")?
        .get("ownerText")?
        .get("runs")?
        .get(0)?
        .get("text")?
        .to_string();
    Some(clean_quotes(&owner))
}
pub(crate) fn get_views(source: &Value) -> Option<String> {
    let owner = source
        .get("videoRenderer")?
        .get("viewCountText")?
        .get("simpleText")?
        .to_string();
    Some(clean_quotes(&owner))
}
pub(crate) fn get_published_time(source: &Value) -> Option<String> {
    let owner = source
        .get("videoRenderer")?
        .get("publishedTimeText")?
        .get("simpleText")?
        .to_string();
    Some(clean_quotes(&owner))
}
