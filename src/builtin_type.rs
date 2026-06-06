use super::BuiltinId;
use std::env;
use std::fs;
use std::io::{self, Error};
use std::str::FromStr;
use is_executable::IsExecutable;

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

fn find_executable_in_path(cmd_name: &String) -> Option<String> {
    // Iterate over all paths in os env variable PATH
    // and check whether an executable of that name is in the path
    // Return the full path if found, None if not

    match env::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let full_path = path.join(cmd_name);
                if let Ok(file_exists) = fs::exists(&full_path) {
                    if file_exists && full_path.is_executable() {
                        return Some(full_path.to_string_lossy().into_owned());
                    }
                }
            }
        }
        None => {
            println!("OS environment variable PATH is not set.");
        }
    }

    // No matching path has been found
    None
}
