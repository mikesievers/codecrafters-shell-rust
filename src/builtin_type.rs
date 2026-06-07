use super::BuiltinId;
use std::io::{self, Error};
use std::str::FromStr;

use crate::util::find_executable_in_path;

#[derive(Debug, PartialEq)]
pub struct Type {
    pub args: Vec<String>,
}

impl Type {
    pub fn run(&self) -> io::Result<i32> {
        let cmd_name = self.args.first().unwrap();

        match BuiltinId::from_str(cmd_name) {
            Ok(_) => {
                println!("{} is a shell builtin", cmd_name)
            }
            Err(_) => {
                // It's not a builtin, check if it's an executable
                match find_executable_in_path(cmd_name) {
                    Some(full_path) => {
                        println!("{} is {}", cmd_name, full_path);
                    }
                    None => {
                        println!("{}: not found", cmd_name)
                    }
                }
            }
        }
        Err(Error::from_raw_os_error(0))
    }
}
