use dialoguer::Input;
use std::io::{stdout, Write};
use termion::{clear, cursor};

pub fn prompt(prompt: &str) -> String {
    print!("{}{}", clear::All, cursor::Goto(1, 1));

    stdout().flush().unwrap();

    Input::<String>::new()
        .with_prompt(prompt)
        .interact()
        .unwrap()
}

pub fn print_pos(x: u16, y: u16, msg: String) {
    print!("{}{}{}", cursor::Goto(x, y), clear::CurrentLine, msg);
    stdout().flush().unwrap();
}
