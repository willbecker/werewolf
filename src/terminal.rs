use dialoguer::{Input, Confirmation};
use std::io::{stdout, Write};
use termion::{clear, cursor, color, terminal_size};

pub fn prompt(prompt: &str) -> String {
    clear_all();

    Input::<String>::new()
        .with_prompt(prompt)
        .interact()
        .unwrap()
}

pub fn confirm(message: &str) {
    print!("{}{}", cursor::Right(1), color::Fg(color::Green));
    stdout().flush().unwrap();

    Confirmation::new()
        .with_text(message)
        .default(true)
        .show_default(false)
        .interact();

    print!("{}", color::Fg(color::Reset));
}

pub fn print_pos(x: u16, y: u16, msg: String) {
    print!("{}{}{}", cursor::Goto(x, y), clear::CurrentLine, msg);
    stdout().flush().unwrap();
}

pub fn clear_all() {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    stdout().flush().unwrap();
}
