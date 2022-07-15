use std::io::{self, Write};
use std::fs;

pub fn get_input(question: &str) -> String {
    print!("{}", question);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn print_in_file(filename: &str, line: &str, append: bool) {
    let mut file = fs::OpenOptions::new()
      .write(true)
      .append(append)
      .open(filename)
      .unwrap();
      
    file.write_all(line.as_bytes()).unwrap();
}

pub fn clear_file(filename: &str) {
    let mut file = fs::OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(filename)
      .unwrap();
    file.write(b"").unwrap();
}