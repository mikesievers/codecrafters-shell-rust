use std::io::{self, Error};

#[derive(Debug, PartialEq)]
pub struct ExternalCmd {
    pub name: String,
    pub parameters: Vec<String>,
}

impl ExternalCmd {
    pub fn run(&self) -> io::Result<i32> {
        println!("{}: command not found", self.name);
        Err(Error::from_raw_os_error(1))
    }
}
