use crate::data_parse::{get_owner, get_title};

use serde_json::Value;

use crossterm::style::Print;
use crossterm::terminal::size;
use crossterm::ExecutableCommand;
use std::io::stdout;

pub(crate) fn display(json: &[Value]) {
    let window_width = size().unwrap().0;
    for element in json {
        format_out(element, window_width);
    }
}

pub(crate) fn format_out(source: &Value, width: u16) -> Option<()> {
    let owner = get_owner(source)?;
    let video_title = get_title(source)?;
    let mut out = format!("{} | {}", video_title, owner);
    if out.len() > width as usize {
        if width > 3 {
            out.truncate((width - 3) as usize);
            out.push_str("...");
        } else {
            out.truncate(width as usize)
        }
    }
    out.push('\n');
    stdout().execute(Print(out)).unwrap();
    Some(())
}
