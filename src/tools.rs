use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::{io::stdout, process::exit};

pub(crate) fn error(message: &str) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        Print(format!("\n{}", message)),
        ResetColor
    )
    .unwrap()
}
pub(crate) fn exit_error(message: &str) -> ! {
    error(message);
    exit(-1)
}
