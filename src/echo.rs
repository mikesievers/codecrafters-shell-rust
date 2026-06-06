use std::io::{self, Error};

#[derive(Debug, PartialEq)]
pub struct Echo {
    pub args: Vec<String>,
}

impl Echo {
    pub fn run(&self) -> io::Result<i32> {
        let line = self.args.join(" ");
        println!("{}", line);
        Err(Error::from_raw_os_error(0))
    }
}
