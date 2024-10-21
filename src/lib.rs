pub mod args;
pub mod block;
pub mod cc;
pub mod codel;
pub mod command;
pub mod dp;
pub mod image;
pub mod interpreter;
pub mod stdin;

use std::error::Error;

use rustc_hash::FxHashSet;

use crate::args::Args;
use crate::command::Command;
use crate::image::Image;
use crate::interpreter::Interpreter;

/// Prints the given string to stderr if `is_verbose_mode` is `true`.
fn debug_print(is_verbose_mode: bool, s: &str) {
    if is_verbose_mode {
        eprintln!("{}", s);
    }
}

/// Runs a Piet program.
//This functions is tested in integration tests.
pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let img = Image::new(&args.image_file, args.codel_size)?;
    debug_print(args.verbose, &format!("{}", img));

    if img.get_codel_at((0, 0)).is_black() {
        return Err("the top-left codel shall not be black".into());
    }

    let mut ip = Interpreter::new();

    let mut num_iter = 0;
    loop {
        let cur_codel = img.get_codel_at(ip.cur);
        assert!(!cur_codel.is_black());
        if !cur_codel.is_white() {
            if num_iter == args.max_iter.unwrap_or(usize::MAX) {
                println!("Program terminated by `max-iter`.");
                return Ok(());
            }
            num_iter += 1;

            debug_print(args.verbose, &ip.to_string());

            let iter_max = 8; //changes `dp` and `cc` at most 7 times
            for i in 0..iter_max {
                //[spec]
                // Black colour blocks and the edges of the program restrict program flow.
                // If the Piet interpreter attempts to move into a black block or off an edge,
                // it is stopped and the CC is toggled.
                // The interpreter then attempts to move from its current block again.
                // If it fails a second time, the DP is moved clockwise one step.
                // These attempts are repeated, with the CC and DP being changed between alternate attempts.
                // If after eight attempts the interpreter cannot leave its current colour block,
                // there is no way out and the program terminates.
                let next_index = img.get_next_codel_index(ip.cur, &ip.dp, &ip.cc);
                if next_index.is_none() {
                    if i % 2 == 0 {
                        ip.cc = ip.cc.flip();
                    } else {
                        ip.dp = ip.dp.turn_right();
                    }
                    if i == iter_max - 1 {
                        return Ok(());
                    }
                    continue;
                }
                let next_codel = img.get_codel_at(next_index.unwrap());
                if next_codel.is_black() {
                    if i % 2 == 0 {
                        ip.cc = ip.cc.flip();
                    } else {
                        ip.dp = ip.dp.turn_right();
                    }
                    if i == iter_max - 1 {
                        return Ok(());
                    }
                    continue;
                }

                if next_codel.is_white() {
                    ip.cur = next_index.unwrap();
                    break;
                }

                let command = Command::new(cur_codel, next_codel);
                debug_print(args.verbose, &format!("    {:?}", command));
                let block_size = img.get_block_size_at(ip.cur);
                command.execute(&mut ip, block_size);

                ip.cur = next_index.unwrap();
                break;
            }
        } else {
            //See `White Blocks` section in the spec: https://www.dangermouse.net/esoteric/piet.html

            let mut visited = FxHashSet::default();

            //FIXME: Currently, the average number of iterations needed to find a non-white codel or wall is the size of the current white block.
            //       Ideally it should be O(1) (like `Block::get_corner_index()`).
            loop {
                if num_iter == args.max_iter.unwrap_or(usize::MAX) {
                    println!("Program terminated by `max-iter`.");
                    return Ok(());
                }
                num_iter += 1;

                debug_print(args.verbose, &ip.to_string());

                if visited.contains(&(ip.cur, ip.dp)) {
                    return Ok(());
                }
                visited.insert((ip.cur, ip.dp));

                let next_index = img.get_next_codel_index_in_dp_direction(ip.cur, &ip.dp);
                if next_index.is_none() {
                    ip.cc = ip.cc.flip();
                    ip.dp = ip.dp.turn_right();
                    continue;
                }
                let next_codel = img.get_codel_at(next_index.unwrap());
                if next_codel.is_black() {
                    ip.cc = ip.cc.flip();
                    ip.dp = ip.dp.turn_right();
                    continue;
                }

                ip.cur = next_index.unwrap();

                if next_codel.is_white() {
                    continue;
                }

                //spec: If the transition between colour blocks occurs via a slide across a white block, no command is executed.

                break;
            }
        }
    }
}
