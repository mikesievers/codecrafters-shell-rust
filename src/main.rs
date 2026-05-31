#[allow(unused_imports)]
use std::io::{self, Write, stdin};

fn main() {
    // Main REPL loop
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();

        // TODO: Add error handling rather than panicking in read_command
        read_input(&mut buffer);

        let cmd = parse_input(&buffer);

        match cmd {
            "exit" => {
                break;
            }
            _ => {
                println!("{}: command not found", cmd);
            }
        }
    }
}

fn read_input(buffer: &mut String) {
    stdin().read_line(buffer).expect("Could not read input");
}

fn parse_input(buffer: &String) -> &str {
    buffer.trim()
}
