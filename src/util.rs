use std::io;

pub fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(..) => return input,
        Err(error) => println!("Error: {}", error),
    }
    read_line()
}