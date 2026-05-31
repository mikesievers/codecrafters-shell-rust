use itertools::Itertools;
#[allow(unused_imports)]
use std::io::{self, Write, stdin};
use std::process::exit;

enum Builtin {
    Exit,
    Echo(Vec<String>),
    // TODO: Refactor Notbuiltin to be a command
    // This will need the parse function to return an enum of Builtin/Command
    Notbuiltin(String),
}

fn main() {
    // Main REPL loop
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();

        // TODO: Add error handling rather than panicking in read_command
        read_input(&mut buffer);

        let cmd_line = parse_input(&buffer);

        execute_cmd_line(cmd_line);
    }
}

fn execute_cmd_line(cmd_line: Option<Builtin>) {
    match cmd_line {
        Some(Builtin::Exit) => {
            exit(0);
        }
        Some(Builtin::Echo(params)) => {
            println!("{}", params.join(" "));
        }
        Some(Builtin::Notbuiltin(cmd)) => {
            println!("{}: command not found", cmd);
        }
        _ => {}
    }
}

fn read_input(buffer: &mut String) {
    stdin().read_line(buffer).expect("Could not read input");
}

fn parse_input(buffer: &String) -> Option<Builtin> {
    let line_parts = buffer.trim().split(" ").collect_vec();

    match line_parts.split_first() {
        None => return None,
        Some((&command, params)) => match command {
            "exit" => Some(Builtin::Exit),
            "echo" => Some(Builtin::Echo(
                params.into_iter().map(|s| s.to_string()).collect_vec(),
            )),
            _ => Some(Builtin::Notbuiltin(command.to_string())),
        },
    }
}
