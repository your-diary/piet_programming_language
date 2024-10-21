use std::collections::VecDeque;

use num::Integer;

use super::codel::Codel;
use super::interpreter::Interpreter;

/// Piet Commands (Push, Mod, Roll, etc.)
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
    InNumber,
    InChar,
    OutNumber,
    OutChar,
}

impl Command {
    /**
    Creates a new command from two codels before movement and after movement resp.

    [The spec](https://www.dangermouse.net/esoteric/piet.html) says

    > Commands are defined by the transition of colour from one colour block to the next as the interpreter travels through the program.
    > The number of steps along the Hue Cycle and Lightness Cycle in each transition determine the command executed, as shown in the table at right.
    > If the transition between colour blocks occurs via a slide across a white block, no command is executed.

    This constructor always returns a valid command to be executed. In other words,

    > If the transition between colour blocks occurs via a slide across a white block, no command is executed.

    above is out of the scope of this function.
    */
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
            (4, 2) => Command::InNumber,

            (5, 0) => Command::InChar,
            (5, 1) => Command::OutNumber,
            (5, 2) => Command::OutChar,

            _ => unreachable!(),
        }
    }

    /**
    Executes the command represented by `self`.

    `block_size` shall be the size of the block before movement and it is only used in `Push` command. See also [`Block::size`](super::block::Block::size).

    As [the spec](https://www.dangermouse.net/esoteric/piet.html) says,

    >  Any operations which cannot be performed (such as popping values when not enough are on the stack) are simply ignored, and processing continues with the next command.
    */
    pub fn execute(&self, ip: &mut Interpreter, block_size: usize) {
        assert!(block_size > 0);
        let block_size = block_size as isize;
        let stack = &mut ip.stack;
        match self {
            //spec: Pushes the value of the colour block just exited on to the stack.
            Command::Push => {
                stack.push(block_size);
            }

            //spec: Pops the top value off the stack and discards it.
            Command::Pop => {
                stack.pop();
            }

            //spec: Pops the top two values off the stack, adds them, and pushes the result back on the stack.
            Command::Add => {
                if stack.len() >= 2 {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(x + y);
                }
            }

            //spec: Pops the top two values off the stack, calculates the second top value minus the top value, and pushes the result back on the stack.
            Command::Subtract => {
                if stack.len() >= 2 {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(y - x);
                }
            }

            //spec: Pops the top two values off the stack, multiplies them, and pushes the result back on the stack.
            Command::Multiply => {
                if stack.len() >= 2 {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(x * y);
                }
            }

            //[spec]
            //Pops the top two values off the stack, calculates the integer division of the second top value by the top value,
            //and pushes the result back on the stack.
            //If a divide by zero occurs, it is handled as an implementation-dependent error,
            //though simply ignoring the command is recommended.
            Command::Divide => {
                if stack.len() >= 2 {
                    if *stack.last().unwrap() == 0 {
                        return; //zero-division
                    }
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(y / x);
                }
            }

            //[spec]
            //Pops the top two values off the stack, calculates the second top value modulo the top value, and pushes the result back on the stack.
            //The result has the same sign as the divisor (the top value).
            //If the top value is zero, this is a divide by zero error, which is handled as an implementation-dependent error,
            //though simply ignoring the command is recommended. (See note below.)
            // (snip)
            //The mod command is thus identical to floored division
            Command::Mod => {
                if stack.len() >= 2 {
                    if *stack.last().unwrap() == 0 {
                        return; //zero-division
                    }
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    #[allow(unstable_name_collisions)]
                    stack.push(y - (y.div_floor(&x) * x)); //Python-style mod
                }
            }

            //spec: Replaces the top value of the stack with 0 if it is non-zero, and 1 if it is zero.
            Command::Not => {
                if !stack.is_empty() {
                    let x = stack.pop().unwrap();
                    if x == 0 {
                        stack.push(1);
                    } else {
                        stack.push(0);
                    }
                }
            }

            //spec: Pops the top two values off the stack, and pushes 1 on to the stack if the second top value is greater than the top value, and pushes 0 if it is not greater.
            Command::Greater => {
                if stack.len() >= 2 {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    if y > x {
                        stack.push(1);
                    } else {
                        stack.push(0);
                    }
                }
            }

            //spec: Pops the top value off the stack and rotates the DP clockwise that many steps (anticlockwise if negative).
            Command::Pointer => {
                if !stack.is_empty() {
                    let x = stack.pop().unwrap();
                    ip.dp = ip.dp.rotate_clockwise_by(x);
                }
            }

            //spec: Pops the top value off the stack and toggles the CC that many times (the absolute value of that many times if negative).
            Command::Switch => {
                if !stack.is_empty() {
                    let x = stack.pop().unwrap();
                    if x.abs() % 2 == 1 {
                        ip.cc = ip.cc.flip();
                    }
                }
            }

            //spec: Pushes a copy of the top value on the stack on to the stack.
            Command::Duplicate => {
                if !stack.is_empty() {
                    stack.push(*stack.last().unwrap());
                }
            }

            //visualization: https://github.com/your-diary/piet_programming_language/blob/master/readme_assets/spec.png
            //
            //[spec]
            //Pops the top two values off the stack and "rolls" the remaining stack entries to a depth equal to the second value popped,
            //by a number of rolls equal to the first value popped.
            //A single roll to depth n is defined as burying the top value on the stack n deep and bringing all values above it up by 1 place.
            //A negative number of rolls rolls in the opposite direction.
            //A negative depth is an error and the command is ignored.
            //If a roll is greater than an implementation-dependent maximum stack depth,
            //it is handled as an implementation-dependent error, though simply ignoring the command is recommended.
            Command::Roll => {
                if stack.len() < 2 {
                    return;
                }

                let num_roll = stack[stack.len() - 1];
                let depth = stack[stack.len() - 2];
                if (depth < 0) || (stack.len() - 2 < depth as usize) {
                    return;
                }
                for _ in 0..2 {
                    stack.pop().unwrap();
                }
                //if operation can be done but virtually nothing happens
                if (depth <= 1) || (num_roll == 0) {
                    return;
                }

                let mut buf = VecDeque::with_capacity(depth as usize);
                for _ in 0..depth {
                    buf.push_front(stack.pop().unwrap());
                }
                if num_roll > 0 {
                    buf.rotate_right((num_roll % depth) as usize);
                } else {
                    buf.rotate_left((num_roll.abs() % depth) as usize);
                }
                for e in buf {
                    stack.push(e);
                }
            }

            //[spec]
            //Reads a value from STDIN as either a number or character,
            //depending on the particular incarnation of this command and pushes it on to the stack.
            //If no input is waiting on STDIN, this is an error and the command is ignored.
            //If an integer read does not receive an integer value, this is an error and the command is ignored.
            Command::InNumber => {
                if let Some(n) = ip.stdin.read_integer() {
                    stack.push(n);
                }
            }

            //[spec]
            //Reads a value from STDIN as either a number or character,
            //depending on the particular incarnation of this command and pushes it on to the stack.
            //If no input is waiting on STDIN, this is an error and the command is ignored.
            //If an integer read does not receive an integer value, this is an error and the command is ignored.
            Command::InChar => {
                if let Some(c) = ip.stdin.read_char() {
                    stack.push(c as isize);
                }
            }

            //[spec]
            //Pops the top value off the stack and prints it to STDOUT as either a number or character,
            //depending on the particular incarnation of this command.
            Command::OutNumber => {
                if !stack.is_empty() {
                    let x = stack.pop().unwrap();
                    ip.output(&format!("{}\n", x));
                }
            }

            //[spec]
            //Pops the top value off the stack and prints it to STDOUT as either a number or character,
            //depending on the particular incarnation of this command.
            Command::OutChar => {
                if !stack.is_empty() {
                    let x = *stack.last().unwrap();
                    if (0 <= x) && (x <= char::MAX as isize) {
                        stack.pop().unwrap();
                        ip.output(&format!("{}", char::from_u32(x as u32).unwrap()));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::super::cc::CC;
    use super::super::dp::DP;
    use super::*;

    #[test]
    fn test_push() {
        let command = Command::Push;
        let mut ip = Interpreter::new();
        ip.stack = vec![1, 2];
        command.execute(&mut ip, 3);
        assert_eq!(vec![1, 2, 3], ip.stack);
    }

    #[test]
    fn test_pop() {
        let command = Command::Pop;

        let mut ip = Interpreter::new();
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1, 2];
        command.execute(&mut ip, 1);
        assert_eq!(ip.stack, vec![1]);
    }

    #[test]
    fn test_add() {
        let command = Command::Add;

        let mut ip = Interpreter::new();
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        command.execute(&mut ip, 1);
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![1, 2];
        command.execute(&mut ip, 1);
        assert_eq!(vec![3], ip.stack);
    }

    #[test]
    fn test_subtract() {
        let command = Command::Subtract;

        let mut ip = Interpreter::new();
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        command.execute(&mut ip, 1);
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![1, 2];
        command.execute(&mut ip, 1);
        assert_eq!(vec![-1], ip.stack);
    }

    #[test]
    fn test_multiply() {
        let command = Command::Multiply;

        let mut ip = Interpreter::new();
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        command.execute(&mut ip, 1);
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![2, 3];
        command.execute(&mut ip, 1);
        assert_eq!(vec![6], ip.stack);
    }

    #[test]
    fn test_divide() {
        let command = Command::Divide;

        let mut ip = Interpreter::new();
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        command.execute(&mut ip, 1);
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![7, 3];
        command.execute(&mut ip, 1);
        assert_eq!(vec![2], ip.stack);

        //zero-division
        let mut ip = Interpreter::new();
        ip.stack = vec![2, 7, 0];
        command.execute(&mut ip, 1);
        assert_eq!(vec![2, 7, 0], ip.stack);
    }

    #[test]
    fn test_mod() {
        let command = Command::Mod;

        let mut ip = Interpreter::new();
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        command.execute(&mut ip, 1);
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![5, 3];
        command.execute(&mut ip, 1);
        assert_eq!(vec![2], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![2, 3];
        command.execute(&mut ip, 1);
        assert_eq!(vec![2], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![-1, 3];
        command.execute(&mut ip, 1);
        assert_eq!(vec![2], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![-5, 3];
        command.execute(&mut ip, 1);
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![-5, -3];
        command.execute(&mut ip, 1);
        assert_eq!(vec![-2], ip.stack);

        //zero-division
        let mut ip = Interpreter::new();
        ip.stack = vec![2, 7, 0];
        command.execute(&mut ip, 1);
        assert_eq!(vec![2, 7, 0], ip.stack);
    }

    #[test]
    fn test_not() {
        let command = Command::Not;

        let mut ip = Interpreter::new();
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![0];
        command.execute(&mut ip, 1);
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        command.execute(&mut ip, 1);
        assert_eq!(vec![0], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![2];
        command.execute(&mut ip, 1);
        assert_eq!(vec![0], ip.stack);
    }

    #[test]
    fn test_greater() {
        let command = Command::Greater;

        let mut ip = Interpreter::new();
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![0];
        command.execute(&mut ip, 1);
        assert_eq!(vec![0], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![1, 0];
        command.execute(&mut ip, 1);
        assert_eq!(vec![1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![1, 1];
        command.execute(&mut ip, 1);
        assert_eq!(vec![0], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![1, 2];
        command.execute(&mut ip, 1);
        assert_eq!(vec![0], ip.stack);
    }

    #[test]
    fn test_pointer() {
        let command = Command::Pointer;

        let mut ip = Interpreter::new();
        command.execute(&mut ip, 1);
        assert_eq!(DP::Right, ip.dp);

        let mut ip = Interpreter::new();
        ip.stack = vec![0];
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());
        assert_eq!(DP::Right, ip.dp);

        let mut ip = Interpreter::new();
        ip.stack = vec![2];
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());
        assert_eq!(DP::Left, ip.dp);

        let mut ip = Interpreter::new();
        ip.stack = vec![-1];
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());
        assert_eq!(DP::Up, ip.dp);
    }

    #[test]
    fn test_switch() {
        let command = Command::Switch;

        let mut ip = Interpreter::new();
        command.execute(&mut ip, 1);
        assert_eq!(CC::Left, ip.cc);

        let mut ip = Interpreter::new();
        ip.stack = vec![0];
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());
        assert_eq!(CC::Left, ip.cc);

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());
        assert_eq!(CC::Right, ip.cc);

        let mut ip = Interpreter::new();
        ip.stack = vec![2];
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());
        assert_eq!(CC::Left, ip.cc);

        let mut ip = Interpreter::new();
        ip.stack = vec![3];
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());
        assert_eq!(CC::Right, ip.cc);

        let mut ip = Interpreter::new();
        ip.stack = vec![-1];
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());
        assert_eq!(CC::Right, ip.cc);
    }

    #[test]
    fn test_duplicate() {
        let command = Command::Duplicate;

        let mut ip = Interpreter::new();
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![1];
        command.execute(&mut ip, 1);
        assert_eq!(vec![1, 1], ip.stack);
    }

    //cases in which nothing happens
    #[test]
    fn test_roll_01() {
        let command = Command::Roll;

        //negative depth
        let mut ip = Interpreter::new();
        ip.stack = vec![9, 8, 7, 1, 2, 3, 4, -2, 5];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 8, 7, 1, 2, 3, 4, -2, 5], ip.stack);

        //zero depth
        let mut ip = Interpreter::new();
        ip.stack = vec![9, 8, 7, 1, 2, 3, 4, 0, 5];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 8, 7, 1, 2, 3, 4], ip.stack);

        //one depth
        let mut ip = Interpreter::new();
        ip.stack = vec![9, 8, 7, 1, 2, 3, 4, 1, 5];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 8, 7, 1, 2, 3, 4], ip.stack);

        //depth is too large
        let mut ip = Interpreter::new();
        ip.stack = vec![9, 8, 7, 1, 2, 3, 4, 8, 5];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 8, 7, 1, 2, 3, 4, 8, 5], ip.stack);

        //zero number of rotations
        let mut ip = Interpreter::new();
        ip.stack = vec![9, 8, 7, 1, 2, 3, 4, 4, 0];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 8, 7, 1, 2, 3, 4], ip.stack);
    }

    //positive number of rolls
    #[test]
    fn test_roll_02() {
        let command = Command::Roll;

        let mut ip = Interpreter::new();
        ip.stack = vec![9, 1, 2, 3, 4, 4, 1];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 4, 1, 2, 3], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![9, 1, 2, 3, 4, 4, 2];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 3, 4, 1, 2], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![9, 1, 2, 3, 4, 4, 3];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 2, 3, 4, 1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![9, 1, 2, 3, 4, 4, 4];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 1, 2, 3, 4], ip.stack);

        //expects the complexity is independent of `num_roll`
        let mut ip = Interpreter::new();
        ip.stack = vec![9, 1, 2, 3, 4, 4, 4 * 10isize.pow(8) + 1];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 4, 1, 2, 3], ip.stack);
    }

    //negative number of rolls
    #[test]
    fn test_roll_03() {
        let command = Command::Roll;

        let mut ip = Interpreter::new();
        ip.stack = vec![9, 1, 2, 3, 4, 4, -1];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 2, 3, 4, 1], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![9, 1, 2, 3, 4, 4, -2];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 3, 4, 1, 2], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![9, 1, 2, 3, 4, 4, -3];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 4, 1, 2, 3], ip.stack);

        let mut ip = Interpreter::new();
        ip.stack = vec![9, 1, 2, 3, 4, 4, -4];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 1, 2, 3, 4], ip.stack);

        //expects the complexity is independent of `num_roll`
        let mut ip = Interpreter::new();
        ip.stack = vec![9, 1, 2, 3, 4, 4, -4 * 10isize.pow(8) - 1];
        command.execute(&mut ip, 1);
        assert_eq!(vec![9, 2, 3, 4, 1], ip.stack);
    }

    #[test]
    fn test_read_number() {
        let command = Command::InNumber;
        let mut ip = Interpreter::new_with_stdin(" -100 abc 🍄🌷 100 ");

        command.execute(&mut ip, 1);
        assert_eq!(vec![-100], ip.stack);

        command.execute(&mut ip, 1);
        assert_eq!(vec![-100], ip.stack);

        command.execute(&mut ip, 1);
        assert_eq!(vec![-100], ip.stack);

        command.execute(&mut ip, 1);
        assert_eq!(vec![-100, 100], ip.stack);

        for _ in 0..2 {
            command.execute(&mut ip, 1);
            assert_eq!(vec![-100, 100], ip.stack);
            command.execute(&mut ip, 1);
            assert_eq!(vec![-100, 100], ip.stack);
        }
    }

    #[test]
    fn test_read_char() {
        let command = Command::InChar;
        let mut ip = Interpreter::new_with_stdin(" -1 a 🌷🍄 a🍄 🍄a ");

        let f = |v: Vec<char>| -> Vec<isize> { v.into_iter().map(|c| c as isize).collect_vec() };

        command.execute(&mut ip, 1);
        assert_eq!(f(vec!['-']), ip.stack);

        command.execute(&mut ip, 1);
        assert_eq!(f(vec!['-', '1']), ip.stack);

        command.execute(&mut ip, 1);
        assert_eq!(f(vec!['-', '1', 'a']), ip.stack);

        command.execute(&mut ip, 1);
        assert_eq!(f(vec!['-', '1', 'a', '🌷']), ip.stack);

        command.execute(&mut ip, 1);
        assert_eq!(f(vec!['-', '1', 'a', '🌷', '🍄']), ip.stack);

        command.execute(&mut ip, 1);
        assert_eq!(f(vec!['-', '1', 'a', '🌷', '🍄', 'a']), ip.stack);

        command.execute(&mut ip, 1);
        assert_eq!(f(vec!['-', '1', 'a', '🌷', '🍄', 'a', '🍄']), ip.stack);

        command.execute(&mut ip, 1);
        assert_eq!(
            f(vec!['-', '1', 'a', '🌷', '🍄', 'a', '🍄', '🍄']),
            ip.stack
        );

        command.execute(&mut ip, 1);
        assert_eq!(
            f(vec!['-', '1', 'a', '🌷', '🍄', 'a', '🍄', '🍄', 'a']),
            ip.stack
        );

        for _ in 0..2 {
            command.execute(&mut ip, 1);
            assert_eq!(
                f(vec!['-', '1', 'a', '🌷', '🍄', 'a', '🍄', '🍄', 'a']),
                ip.stack
            );
            command.execute(&mut ip, 1);
            assert_eq!(
                f(vec!['-', '1', 'a', '🌷', '🍄', 'a', '🍄', '🍄', 'a']),
                ip.stack
            );
        }
    }

    #[test]
    fn test_write_number() {
        let command = Command::OutNumber;

        let mut ip = Interpreter::new_with_stdin("");
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new_with_stdin("");
        ip.stack = vec![1];
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());
        assert_eq!("1\n".as_bytes(), &ip.output_buf);

        let mut ip = Interpreter::new_with_stdin("");
        ip.stack = vec![-1];
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());
        assert_eq!("-1\n".as_bytes(), &ip.output_buf);
    }

    #[test]
    fn test_write_char() {
        let command = Command::OutChar;

        let mut ip = Interpreter::new();
        command.execute(&mut ip, 1);
        assert!(ip.stack.is_empty());

        let mut ip = Interpreter::new();
        ip.stack = vec![char::MAX as isize + 1, -1, 'a' as isize, '🍄' as isize];

        command.execute(&mut ip, 1);
        assert_eq!(vec![char::MAX as isize + 1, -1, 'a' as isize], ip.stack);
        assert_eq!("🍄".as_bytes(), &ip.output_buf);

        command.execute(&mut ip, 1);
        assert_eq!(vec![char::MAX as isize + 1, -1], ip.stack);
        assert_eq!("🍄a".as_bytes(), &ip.output_buf);

        command.execute(&mut ip, 1);
        assert_eq!(vec![char::MAX as isize + 1, -1], ip.stack);
        assert_eq!("🍄a".as_bytes(), &ip.output_buf);

        ip.stack.pop().unwrap();
        command.execute(&mut ip, 1);
        assert_eq!(vec![char::MAX as isize + 1], ip.stack);
        assert_eq!("🍄a".as_bytes(), &ip.output_buf);
    }
}
