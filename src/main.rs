mod data_parse;
mod tools;

use crate::data_parse::{find_json, get_length, get_owner, get_video_vec, get_view_count};
use crate::tools::exit_error;

fn main() {
    let query = "shrek meme";
    let data = match data_parse::get_source(query) {
        Ok(data) => data,
        Err(error) => exit_error(&error.to_string()),
    };

    let json = match find_json(&data) {
        Ok(json) => json,
        Err(error) => exit_error(error.as_str()),
    };
    println!("{}", json);
    let video_data = match get_video_vec(&json) {
        None => exit_error("Could not find/parse json"),
        Some(vec) => vec,
    };
    for video_datum in video_data {
        println!("{}", get_view(video_datum).unwrap_or_default());
    }
}
