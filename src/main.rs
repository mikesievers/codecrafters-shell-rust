#[allow(unused_imports)]
use std::io::{self, Write, stdin};

fn main() {
    // Main REPL loop
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        stdin()
            .read_line(&mut command)
            .expect("Could not read input");

        println!("{}: command not found", command.trim());
    }
}
