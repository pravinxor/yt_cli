mod data_display;
mod data_parse;
mod tools;

use crate::data_display::display;
use crate::data_parse::{find_json, get_video_vec};
use crate::tools::exit_error;
use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event};
use crossterm::style::{Attribute, Color, SetAttribute, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::ExecutableCommand;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut stdout = stdout();
    let mut query = String::new();
    print!("Enter a search query >>> ");
    stdout.flush().unwrap();
    stdout
        .execute(SetAttribute(Attribute::Italic))
        .unwrap()
        .execute(SetForegroundColor(Color::Cyan))
        .unwrap();
    stdin().read_line(&mut query).unwrap();
    stdout.execute(SetAttribute(Attribute::Reset)).unwrap();
    let data = match data_parse::get_source(query.trim_end_matches(&['\r', '\n'][..])) {
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
    enable_raw_mode().unwrap();
    stdout.execute(Clear(ClearType::All)).unwrap();
    display(video_data);
    loop {
        if let Event::Resize(_width, _height) = read().unwrap() {
            stdout.execute(Clear(ClearType::All)).unwrap();
            display(video_data);
        }
    }
}
