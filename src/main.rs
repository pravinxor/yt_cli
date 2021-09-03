mod data_parse;
mod tools;

use crate::data_parse::find_json;
use crate::tools::exit_error;

fn main() {
    let query = "take on me";
    let data = match data_parse::get_source(query) {
        Ok(data) => data,
        Err(error) => exit_error(&error.to_string()),
    };

    let json = match find_json(&data) {
        Ok(json) => json,
        Err(error) => exit_error(error.as_str()),
    };
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
