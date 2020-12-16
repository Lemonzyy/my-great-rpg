use std::{
    io::{self, Write},
};

const SEPARATOR: &str = "====";

/// This function use recursion for getting the user console input.
pub fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(..) => {
            return input.trim_end().to_string();
        }
        Err(error) => println!("Error during read_line: {}", error),
    }
    read_line()
}

/// This function clear the console and put the cursor at the top left.
pub fn clear_console() {
    println!("\x1B[2J\x1B[1;1H");
}

pub fn print_title(str: &str) {
    println!("{0} {1} {0}", SEPARATOR, str)
}

pub fn ask<A, C, F>(action: A, condition: C, fallback: F) -> Result<String, String>
    where A: Fn(),
          C: Fn(&String) -> bool,
          F: Fn(&String) {
    loop {
        action();

        io::stdout().flush().expect("Error while stdout flushing");
        let value = read_line();

        if !value.is_empty() && condition(&value) {
            break Ok(value);
        } else {
            fallback(&value)
        }
    }
}