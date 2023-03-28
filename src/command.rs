use std::error::Error;

use num::Integer;

use super::codel::Codel;
use super::interpreter::Interpreter;

#[derive(Debug)]
pub enum Command {
    Push,
    Pop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Not,
    Greater,
    Pointer,
    Switch,
    Duplicate,
    Roll,
    ReadNumber,
    ReadChar,
    WriteNumber,
    WriteChar,
}

impl Command {
    pub fn new(from: &Codel, to: &Codel) -> Self {
        let hue_difference = Codel::get_hue_difference(from, to);
        let lightness_difference = Codel::get_lightness_difference(from, to);
        match (hue_difference, lightness_difference) {
            (0, 1) => Command::Push,
            (0, 2) => Command::Pop,

            (1, 0) => Command::Add,
            (1, 1) => Command::Subtract,
            (1, 2) => Command::Multiply,

            (2, 0) => Command::Divide,
            (2, 1) => Command::Mod,
            (2, 2) => Command::Not,

            (3, 0) => Command::Greater,
            (3, 1) => Command::Pointer,
            (3, 2) => Command::Switch,

            (4, 0) => Command::Duplicate,
            (4, 1) => Command::Roll,
            (4, 2) => Command::ReadNumber,

            (5, 0) => Command::ReadChar,
            (5, 1) => Command::WriteNumber,
            (5, 2) => Command::WriteChar,

            _ => unreachable!(),
        }
    }

    pub fn apply(&self, ip: &mut Interpreter, value: isize) -> Result<(), Box<dyn Error>> {
        assert!(value > 0);
        let stack = &mut ip.stack;
        match (self) {
            Command::Push => {
                stack.push(value);
            }
            Command::Pop => {
                stack.pop();
            }
            Command::Add => {
                if (stack.len() >= 2) {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(x + y);
                }
            }
            Command::Subtract => {
                if (stack.len() >= 2) {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(y - x);
                }
            }
            Command::Multiply => {
                if (stack.len() >= 2) {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(x * y);
                }
            }
            Command::Divide => {
                if (stack.len() >= 2) {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    if (x == 0) {
                        return Err(format!("zero-division at {:?}", value).into());
                    }
                    stack.push(y / x);
                }
            }
            Command::Mod => {
                if (stack.len() >= 2) {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    if (x == 0) {
                        return Err(format!("zero-division at {:?}", value).into());
                    }
                    #[allow(unstable_name_collisions)]
                    stack.push(y - (y.div_floor(&x) * x)); //Python-style mod
                }
            }
            Command::Not => {
                if (!stack.is_empty()) {
                    let x = stack.pop().unwrap();
                    if (x == 0) {
                        stack.push(1);
                    } else {
                        stack.push(0);
                    }
                }
            }
            Command::Greater => {
                if (stack.len() >= 2) {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    if (y > x) {
                        stack.push(1);
                    } else {
                        stack.push(0);
                    }
                }
            }
            Command::Pointer => {
                if (!stack.is_empty()) {
                    let x = stack.pop().unwrap();
                    ip.dp = ip.dp.rotate_by(x);
                }
            }
            Command::Switch => {
                if (!stack.is_empty()) {
                    let x = stack.pop().unwrap();
                    if (x.abs() % 2 == 1) {
                        ip.cc = ip.cc.flip();
                    }
                }
            }
            Command::Duplicate => {
                if (!stack.is_empty()) {
                    stack.push(*stack.last().unwrap());
                }
            }
            Command::Roll => {
                if (stack.len() >= 2) {
                    let num_roll = stack.pop().unwrap();
                    let depth = stack.pop().unwrap();
                    todo!();
                }
            }
            Command::ReadNumber => {
                let n = ip.stdin.read_integer();
                if (n.is_none()) {
                    return Ok(());
                }
                stack.push(n.unwrap());
            }
            Command::ReadChar => {
                let c = ip.stdin.read_char();
                if (c.is_none()) {
                    return Ok(());
                }
                stack.push(c.unwrap() as isize);
            }
            Command::WriteNumber => {
                if (!stack.is_empty()) {
                    let x = stack.pop().unwrap();
                    ip.output(&x.to_string());
                }
            }
            Command::WriteChar => {
                if (!stack.is_empty()) {
                    let x = stack.pop().unwrap();
                    ip.output(&(x as u8 as char).to_string());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::cc::CC;
    use super::super::dp::DP;
    use super::*;

    #[test]
    fn test_push() {
        let command = Command::Push;
        let mut ip = Interpreter::new();
        ip.stack = vec![1, 2];
        assert!(command.apply(&mut ip, 3).is_ok());
        assert_eq!(vec![1, 2, 3], ip.stack);
    }

    #[test]
    fn test_pop() {
        let command = Command::Pop;

        let mut ip = Interpreter::new();
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1, 2];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(ip.stack, vec![1]);
    }

    #[test]
    fn test_add() {
        let command = Command::Add;

        let mut ip = Interpreter::new();
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![1, 2];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![3], ip.stack);
    }

    #[test]
    fn test_subtract() {
        let command = Command::Subtract;

        let mut ip = Interpreter::new();
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![1, 2];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![-1], ip.stack);
    }

    #[test]
    fn test_multiply() {
        let command = Command::Multiply;

        let mut ip = Interpreter::new();
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![2, 3];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![6], ip.stack);
    }

    #[test]
    fn test_divide() {
        let command = Command::Divide;

        let mut ip = Interpreter::new();
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![7, 3];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![2], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![2, 7, 0];
        assert!(command.apply(&mut ip, 1).is_err());
        assert_eq!(vec![2], ip.stack);
    }

    #[test]
    fn test_mod() {
        let command = Command::Mod;

        let mut ip = Interpreter::new();
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![5, 3];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![2], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![2, 3];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![2], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![-1, 3];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![2], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![-5, 3];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![-5, -3];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![-2], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![2, 7, 0];
        assert!(command.apply(&mut ip, 1).is_err());
        assert_eq!(vec![2], ip.stack);
    }

    #[test]
    fn test_not() {
        let command = Command::Not;

        let mut ip = Interpreter::new();
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![0];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![0], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![2];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![0], ip.stack);
    }

    #[test]
    fn test_greater() {
        let command = Command::Greater;

        let mut ip = Interpreter::new();
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![0];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![0], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![1, 0];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![1, 1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![0], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![1, 2];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![0], ip.stack);
    }

    #[test]
    fn test_pointer() {
        let command = Command::Pointer;

        let mut ip = Interpreter::new();
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(DP::Right, ip.dp);

        let mut ip = Interpreter::new();
        ip.stack = vec![0];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());
        assert_eq!(DP::Right, ip.dp);

        let mut ip = Interpreter::new();
        ip.stack = vec![2];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());
        assert_eq!(DP::Left, ip.dp);

        let mut ip = Interpreter::new();
        ip.stack = vec![-1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());
        assert_eq!(DP::Up, ip.dp);
    }

    #[test]
    fn test_switch() {
        let command = Command::Switch;

        let mut ip = Interpreter::new();
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(CC::Left, ip.cc);

        let mut ip = Interpreter::new();
        ip.stack = vec![0];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());
        assert_eq!(CC::Left, ip.cc);

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());
        assert_eq!(CC::Right, ip.cc);

        let mut ip = Interpreter::new();
        ip.stack = vec![2];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());
        assert_eq!(CC::Left, ip.cc);

        let mut ip = Interpreter::new();
        ip.stack = vec![3];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());
        assert_eq!(CC::Right, ip.cc);

        let mut ip = Interpreter::new();
        ip.stack = vec![-1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());
        assert_eq!(CC::Right, ip.cc);
    }

    #[test]
    fn test_duplicate() {
        let command = Command::Duplicate;

        let mut ip = Interpreter::new();
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![1, 1], ip.stack);
    }

    #[test]
    fn test_roll() {
        //TODO
    }

    #[test]
    fn test_read_number() {
        let command = Command::ReadNumber;
        let mut ip = Interpreter::new_with_stdin(" -100 abc 100 ");

        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![-100], ip.stack);

        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![-100], ip.stack);

        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![-100, 100], ip.stack);

        for _ in 0..2 {
            assert!(command.apply(&mut ip, 1).is_ok());
            assert_eq!(vec![-100, 100], ip.stack);
            assert!(command.apply(&mut ip, 1).is_ok());
            assert_eq!(vec![-100, 100], ip.stack);
        }
    }

    #[test]
    fn test_read_char() {
        let command = Command::ReadChar;
        let mut ip = Interpreter::new_with_stdin(" -1 a ");

        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![b'-' as isize], ip.stack);

        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![b'-' as isize, b'1' as isize], ip.stack);

        assert!(command.apply(&mut ip, 1).is_ok());
        assert_eq!(vec![b'-' as isize, b'1' as isize, b'a' as isize], ip.stack);

        for _ in 0..2 {
            assert!(command.apply(&mut ip, 1).is_ok());
            assert_eq!(vec![b'-' as isize, b'1' as isize, b'a' as isize], ip.stack);
            assert!(command.apply(&mut ip, 1).is_ok());
            assert_eq!(vec![b'-' as isize, b'1' as isize, b'a' as isize], ip.stack);
        }
    }

    #[test]
    fn test_write_number() {
        let command = Command::WriteNumber;

        let mut ip = Interpreter::new_with_stdin("");
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new_with_stdin("");
        ip.stack = vec![1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());
        assert_eq!("1\n".as_bytes(), &ip.output_buf);

        let mut ip = Interpreter::new_with_stdin("");
        ip.stack = vec![-1];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());
        assert_eq!("-1\n".as_bytes(), &ip.output_buf);
    }

    #[test]
    fn test_write_char() {
        let command = Command::WriteChar;

        let mut ip = Interpreter::new();
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![b'a' as isize];
        assert!(command.apply(&mut ip, 1).is_ok());
        assert!(ip.stack.is_empty());
        assert_eq!("a\n".as_bytes(), &ip.output_buf);
    }
}
