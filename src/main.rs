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
    Type(Vec<String>),
}

impl Builtin {
    fn execute(&self) {
        match self {
            Builtin::Exit => exit(0),
            Builtin::Echo(parameters) => {
                println!("{}", parameters.join(" "));
            }
            Builtin::Type(parameters) => {
                let cmd = parse_input(&parameters.join(" ")).unwrap();
                match cmd {
                    Command::BuiltinType(Builtin::Exit)
                    | Command::BuiltinType(Builtin::Echo(_))
                    | Command::BuiltinType(Builtin::Type(_)) => {
                        println!("{} is a shell builtin", parameters[0])
                    }
                    Command::ExecutableType(_) => {
                        println!("{}: command not found", parameters[0])
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Executable {
    name: String,
    parameters: Vec<String>,
}

impl Executable {
    fn execute(&self) {
        println!("{}: command not found", self.name);
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    BuiltinType(Builtin),
    ExecutableType(Executable),
}

impl Command {
    // NOTE: The parameters fn could be implemented by Builtin and Executable individually
    fn parameters(&self) -> Option<&Vec<String>> {
        match self {
            Command::BuiltinType(Builtin::Echo(parameters)) => Some(parameters),
            Command::BuiltinType(Builtin::Type(parameters)) => Some(parameters),
            _ => None,
        }
    }
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
        Some(Command::BuiltinType(builtin)) => builtin.execute(),
        Some(Command::ExecutableType(executable)) => executable.execute(),
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
            "type" => Some(Command::BuiltinType(Builtin::Type(
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
    fn test_parse_builtin_exit() {
        assert_eq!(
            parse_input(&"exit".to_string()).unwrap(),
            Command::BuiltinType(Builtin::Exit)
        );
    }

    #[test]
    fn test_parse_builtin_echo() {
        assert_matches!(
            parse_input(&"echo some thing".to_string()),
            Some(Command::BuiltinType(Builtin::Echo(_)))
        );
    }

    #[test]
    fn test_parse_builtin_type() {
        let cmd = parse_input(&"type exit".to_string()).unwrap();
        assert_eq!(cmd.parameters().unwrap(), &vec!["exit".to_string()]);
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
