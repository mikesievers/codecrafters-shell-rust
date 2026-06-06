use itertools::Itertools;
use std::io::{self, Error, Write, stdin};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

mod builtin_echo;
mod builtin_exit;
mod builtin_type;

use builtin_echo::Echo;
use builtin_exit::Exit;
use builtin_type::Type;

// Builtins
// List of known builtins to be used to match them in strings
#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
enum BuiltinId {
    Exit,
    Echo,
    Type,
}

#[derive(Debug)]
enum Builtin {
    Exit(Exit),
    Echo(Echo),
    Type(Type),
}

impl Builtin {
    // Dispatcher method
    fn run(&self) -> io::Result<i32> {
        match self {
            Builtin::Exit(cmd) => cmd.run(),
            Builtin::Echo(cmd) => cmd.run(),
            Builtin::Type(cmd) => cmd.run(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct ExecutableCmd {
    name: String,
    parameters: Vec<String>,
}

impl ExecutableCmd {
    fn run(&self) -> io::Result<i32> {
        println!("{}: command not found", self.name);
        Err(Error::from_raw_os_error(1))
    }
}

#[derive(Debug)]
enum Command {
    //Builtin(Box<dyn BuiltinCmd>),
    Builtin(Builtin),
    External(ExecutableCmd),
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

        let _result = execute_cmd_line(cmd_line);
    }
}

fn execute_cmd_line(cmd_line: Option<Command>) -> io::Result<i32> {
    match cmd_line {
        Some(Command::Builtin(builtin_cmd)) => builtin_cmd.run(),
        Some(Command::External(external_cmd)) => external_cmd.run(),
        _ => Err(Error::from_raw_os_error(1)),
    }
}

fn read_input(buffer: &mut String) {
    stdin().read_line(buffer).expect("Could not read input");
}

fn parse_input(buffer: &String) -> Option<Command> {
    let line_parts = buffer.trim().split(" ").collect_vec();

    match line_parts.split_first() {
        None => return None,
        Some((&command, params)) => match BuiltinId::from_str(command) {
            Ok(BuiltinId::Exit) => {
                let code = params.first().and_then(|s| s.parse::<i32>().ok());
                Some(Command::Builtin(Builtin::Exit(Exit { code })))
            }
            Ok(BuiltinId::Echo) => Some(Command::Builtin(Builtin::Echo(Echo {
                args: params.iter().map(|s| s.to_string()).collect_vec(),
            }))),
            Ok(BuiltinId::Type) => Some(Command::Builtin(Builtin::Type(Type {
                args: params.iter().map(|s| s.to_string()).collect_vec(),
            }))),
            // If matching a builtin fails, treat as executable command
            Err(_) => Some(Command::External(ExecutableCmd {
                name: command.to_string(),
                parameters: params.iter().map(|s| s.to_string()).collect_vec(),
            })),
        },
    }
}

// fn parse_input(buffer: &String) -> Option<OldCommand> {
//     let line_parts = buffer.trim().split(" ").collect_vec();

//     match line_parts.split_first() {
//         None => return None,
//         Some((&command, params)) => match command {
//             "exit" => Some(OldCommand::BuiltinType(Builtin::Exit)),
//             "echo" => Some(OldCommand::BuiltinType(Builtin::Echo(
//                 params.into_iter().map(|s| s.to_string()).collect_vec(),
//             ))),
//             "type" => Some(OldCommand::BuiltinType(Builtin::Type(
//                 params.into_iter().map(|s| s.to_string()).collect_vec(),
//             ))),
//             _ => Some(OldCommand::ExecutableType(ExecutableCmd {
//                 name: command.to_string(),
//                 parameters: vec![],
//             })),
//         },
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    use std::assert_matches;
    use std::str::FromStr;

    #[test]
    fn test_builtinid() {
        assert_eq!(BuiltinId::from_str("exit").unwrap(), BuiltinId::Exit);
        assert_eq!(format!("{}", BuiltinId::Exit), "exit");
    }

    #[test]
    fn test_parse_builtin_exit() {
        assert_matches!(
            parse_input(&"exit".to_string()).unwrap(),
            Command::Builtin(Builtin::Exit(Exit { code: None }))
        );
    }

    #[test]
    fn test_parse_builtin_echo() {
        assert_matches!(
            parse_input(&"echo some thing".to_string()).unwrap(),
            Command::Builtin(Builtin::Echo(Echo { args: _ }))
        );
    }

    #[test]
    fn test_parse_builtin_type() {
        let cmd = parse_input(&"type exit".to_string()).unwrap();
        assert_matches!(cmd, Command::Builtin(Builtin::Type(Type { args: _ })));
    }

    #[test]
    fn test_parse_executable() {
        let _exe_name = "myexe".to_string();
        assert_matches!(
            parse_input(&"myexe somefile".to_string()).unwrap(),
            Command::External(ExecutableCmd {
                name: _,
                parameters: _
            })
        );
    }
}
