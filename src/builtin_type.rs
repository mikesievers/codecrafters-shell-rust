use super::BuiltinId;
use std::io::{self, Error};

#[derive(Debug, PartialEq)]
pub struct Type {
    pub args: Vec<String>,
}

impl Type {
    pub fn run(&self) -> io::Result<i32> {
        let line = self.args.join(" ");
        println!("{}", line);
        Err(Error::from_raw_os_error(0))
    }
}
