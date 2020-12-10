use std::io;

pub fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(..) => {
            input.truncate(input.len() - 1);
            return input;
        }
        Err(error) => println!("Error during read_line: {}", error),
    }
    read_line()
}