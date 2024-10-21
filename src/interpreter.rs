use std::fmt::{self, Display, Formatter};
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

impl Display for Interpreter {
    //The implementation is a bit dirty but see https://github.com/rust-lang/rust/issues/55584 .
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{:12} DP:{:5} CC:{:?}",
            format!("{:?}", self.cur),
            format!("{:?}", self.dp),
            self.cc
        )
    }
}

impl Interpreter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            //spec: The Piet language interpreter begins executing a program in the colour block which includes the upper left codel of the program.
            cur: (0, 0),

            //spec: Piet uses a stack for storage of all data values. Data values exist only as integers
            stack: vec![],

            dp: DP::default(),
            cc: CC::default(),
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
