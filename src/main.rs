#[allow(unused_imports)]
use std::io::{self, Write};
mod command;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    while let Ok(_len) = stdin.read_line(&mut input) {
        //println!("{input}");
        match command::parse_command(&input) {
            Ok(p) => print!("{p}"),
            Err(e) => println!("{e}"),
        }
        input.clear();
        // TODO: clean up
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
