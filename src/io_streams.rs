use std::io::{self, Write};
use std::fs::File;

pub fn get_input(question: &str) -> String {
    print!("{}", question);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn print_in_file(file_name: &str, line: &str) {
    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(error) => {
            println!("Could not create file: {}", error);
            return;
        }
    };
    match file.write_all(line.as_bytes()) {
        Ok(_) => (),
        Err(error) => {
            println!("Could not write to file: {}", error);
            return;
        }
    }
}