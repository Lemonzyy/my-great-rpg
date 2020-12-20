use std::{
    io::{stdin, stdout, Write},
};

use crossterm::{
    cursor,
    QueueableCommand,
    terminal::{self, ClearType},
};

const SEPARATOR: &str = "====";

/// This function use recursion for getting the user console input.
pub fn read_line() -> String {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(..) => {
            return input.trim_end().to_string();
        }
        Err(error) => println!("Error during read_line: {}", error),
    }
    read_line()
}

/// This function clear the console and put the cursor at the top left.
pub fn clear_console() {
    stdout()
        .queue(terminal::Clear(ClearType::All)).expect("Error while clearing the terminal")
        .queue(cursor::MoveTo(0, 0)).expect("Error while moving the terminal cursor to 0,0")
        .flush().unwrap()
}

pub fn print_title(str: &str) {
    println!("{0} {1} {0}", SEPARATOR, str);
}

pub fn ask<A, C, F>(action: A, condition: C, fallback: F) -> Result<String, String>
    where A: Fn(),
          C: Fn(&String) -> bool,
          F: Fn(&String) {
    loop {
        action();

        stdout().flush().expect("Error while stdout flushing");
        let value = read_line();

        if !value.is_empty() && condition(&value) {
            break Ok(value);
        } else {
            fallback(&value)
        }
    }
}