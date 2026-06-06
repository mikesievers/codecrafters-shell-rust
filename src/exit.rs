use std::io;
use std::process::exit;

#[derive(Debug, PartialEq)]
pub struct Exit {
    pub code: Option<i32>,
}

impl Exit {
    pub fn run(&self) -> io::Result<i32> {
        exit(self.code.unwrap_or(0));
    }
}
