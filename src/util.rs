use std::io;

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

/// This function clear the console and put the cursor at the top right.
pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_title(str: &str) {
    println!("{0} {1} {0}", SEPARATOR, str)
}