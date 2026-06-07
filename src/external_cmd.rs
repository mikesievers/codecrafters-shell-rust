use crate::util::find_executable_in_path;
use std::io::{self, Error, Write};
use std::process::Command;

#[derive(Debug, PartialEq)]
pub struct ExternalCmd {
    pub name: String,
    pub parameters: Vec<String>,
}

impl ExternalCmd {
    pub fn run(&self) -> io::Result<i32> {
        match find_executable_in_path(&self.name) {
            Some(full_path) => match Command::new(&self.name).args(&self.parameters).output() {
                Ok(output) => {
                    io::stdout().write_all(&output.stdout)?;
                    io::stderr().write_all(&output.stderr)?;
                    return Err(Error::from_raw_os_error(output.status.code().unwrap_or(1)));
                }
                Err(_) => {
                    println!("Failed to run command {}", full_path);
                    return Err(Error::from_raw_os_error(1));
                }
            },
            None => {
                println!("{}: command not found", self.name);
                return Err(Error::from_raw_os_error(1));
            }
        }
    }
}
