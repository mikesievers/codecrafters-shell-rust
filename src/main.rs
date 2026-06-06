use itertools::Itertools;
use std::any::Any;
use std::io::{self, Error, Write, stdin};
use std::process::exit;
use std::str::FromStr;
use strum_macros::{Display, EnumString};


// Builtins
// Every built in command must be able to run()
trait BuiltinCmd: std::fmt::Debug {
    fn run(&self) -> io::Result<i32>;
    fn as_any(&self) -> &dyn Any;
}

// These are the known builtins
#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
enum BuiltinId {
    Exit,
}

#[derive(Debug, PartialEq)]
struct Exit {
    code: Option<i32>,
}

impl BuiltinCmd for Exit {
    fn run(&self) -> io::Result<i32> {
        exit(self.code.unwrap_or(0));
    }

    fn as_any(&self) -> &dyn Any {
        self
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
    Builtin(Box<dyn BuiltinCmd>),
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

        let _exit_code = execute_cmd_line(cmd_line);
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
                Some(Command::Builtin(Box::new(Exit { code })))
            }
            Err(_) => None,
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
    use std::str::FromStr;
    use std::assert_matches;

    #[test]
    fn test_builtinid() {
        assert_eq!(BuiltinId::from_str("exit").unwrap(), BuiltinId::Exit);
        assert_eq!(format!("{}", BuiltinId::Exit), "exit");
    }

    #[test]
    fn test_parse_builtin_exit() {
        let result = parse_input(&"exit".to_string());

        match result {
            Some(Command::Builtin(cmd)) => {
                let exit_cmd = cmd
                    .as_any()
                    .downcast_ref::<Exit>()
                    .expect("Expected to find an Exit struct");
                assert_eq!(exit_cmd.code, None);
            }
            _ => panic!("Expected an Exit command."),
        }
    }

    // #[test]
    // fn test_parse_builtin_echo() {
    //     assert_matches!(
    //         parse_input(&"echo some thing".to_string()),
    //         Some(OldCommand::BuiltinType(Builtin::Echo(_)))
    //     );
    // }

    // #[test]
    // fn test_parse_builtin_type() {
    //     let cmd = parse_input(&"type exit".to_string()).unwrap();
    //     assert_eq!(cmd.parameters().unwrap(), &vec!["exit".to_string()]);
    // }

    // #[test]
    // fn test_parse_executable() {
    //     let _exe_name = "myexe".to_string();
    //     assert_matches!(
    //         parse_input(&"myexe somefile".to_string()).unwrap(),
    //         OldCommand::ExecutableType(ExecutableCmd {
    //             name: _exe_name,
    //             parameters: _
    //         })
    //     )
    // }
}
