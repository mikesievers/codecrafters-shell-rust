use itertools::Itertools;
#[allow(unused_imports)]
use std::io::{self, Write, stdin};
use std::process::exit;

#[cfg(test)]
use std::assert_matches;

#[derive(Debug, PartialEq)]
enum Builtin {
    Exit,
    Echo(Vec<String>),
}

#[derive(Debug, PartialEq)]
struct Executable {
    name: String,
    parameters: Vec<String>,
}

#[derive(Debug, PartialEq)]
enum Command {
    BuiltinType(Builtin),
    ExecutableType(Executable),
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

fn execute_cmd_line(cmd_line: Option<Command>) {
    match cmd_line {
        Some(Command::BuiltinType(Builtin::Exit)) => {
            exit(0);
        }
        Some(Command::BuiltinType(Builtin::Echo(params))) => {
            println!("{}", params.join(" "));
        }
        Some(Command::ExecutableType(Executable {
            name: cmd,
            parameters: _,
        })) => {
            println!("{}: command not found", cmd);
        }
        _ => {}
    }
}

fn read_input(buffer: &mut String) {
    stdin().read_line(buffer).expect("Could not read input");
}

fn parse_input(buffer: &String) -> Option<Command> {
    let line_parts = buffer.trim().split(" ").collect_vec();

    match line_parts.split_first() {
        None => return None,
        Some((&command, params)) => match command {
            "exit" => Some(Command::BuiltinType(Builtin::Exit)),
            "echo" => Some(Command::BuiltinType(Builtin::Echo(
                params.into_iter().map(|s| s.to_string()).collect_vec(),
            ))),
            _ => Some(Command::ExecutableType(Executable {
                name: command.to_string(),
                parameters: vec![],
            })),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_builtins() {
        assert_eq!(
            parse_input(&"exit".to_string()).unwrap(),
            Command::BuiltinType(Builtin::Exit)
        );
        assert_matches!(
            parse_input(&"echo some thing".to_string()),
            Some(Command::BuiltinType(Builtin::Echo(_)))
        );
    }

    #[test]
    fn test_parse_executable() {
        let _exe_name = "myexe".to_string();
        assert_matches!(
            parse_input(&"myexe somefile".to_string()).unwrap(),
            Command::ExecutableType(Executable {
                name: _exe_name,
                parameters: _
            })
        )
    }
}
