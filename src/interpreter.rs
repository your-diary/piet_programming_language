use std::io::{self, Write};

use super::cc::CC;
use super::dp::DP;
use super::stdin::Stdin;

pub struct Interpreter {
    pub cur: (usize, usize),
    pub stack: Vec<isize>,
    pub dp: DP,
    pub cc: CC,
    pub stdin: Stdin,

    #[cfg(test)]
    pub output_buf: Vec<u8>,
}

impl Interpreter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            cur: (0, 0),
            stack: vec![],
            dp: DP::Right,
            cc: CC::Left,
            stdin: Stdin::new(),

            #[cfg(test)]
            output_buf: vec![],
        }
    }

    //for dependency injection
    pub fn new_with_stdin(s: &str) -> Self {
        Self {
            cur: (0, 0),
            stack: vec![],
            dp: DP::Right,
            cc: CC::Left,
            stdin: Stdin::new_with_string(s),

            #[cfg(test)]
            output_buf: vec![],
        }
    }

    pub fn output(&mut self, s: &str) {
        io::stdout().write_all(s.as_bytes()).unwrap();
        io::stdout().flush().unwrap();

        #[cfg(test)]
        {
            self.output_buf.write_all(s.as_bytes()).unwrap();
        }
    }
}
