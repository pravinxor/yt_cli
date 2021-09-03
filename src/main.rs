mod data_display;
mod data_parse;
mod tools;

use crate::data_display::format_out;
use crate::data_parse::{find_json, get_description_snippet, get_video_vec, get_view_count};
use crate::tools::exit_error;
use std::io::stdout;

fn main() {
    let mut stdout = stdout();
    let query = "shrek meme";
    let data = match data_parse::get_source(query) {
        Ok(data) => data,
        Err(error) => exit_error(&error.to_string()),
    };

    let json = match find_json(&data) {
        Ok(json) => json,
        Err(error) => exit_error(error.as_str()),
    };
    let video_data = match get_video_vec(&json) {
        None => exit_error("Could not find/parse json"),
        Some(vec) => vec,
    };
    for video_datum in video_data {
        format_out(video_datum, 100);
    }
}
