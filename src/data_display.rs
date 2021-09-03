use crate::data_parse::{get_owner, get_title};
use serde_json::Value;

pub(crate) fn format_out(source: &Value, width: usize) -> Option<()> {
    let owner = get_owner(source)?;
    let video_title = get_title(source)?;
    let mut out = format!("{} | {}", video_title, owner);
    if out.len() > width {
        if width > 3 {
            out.truncate(width - 3);
            out.push_str("...");
        } else {
            out.truncate(width)
        }
    }
    println!("{}", out);
    Some(())
}
