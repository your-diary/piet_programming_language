pub mod args;
pub mod block;
pub mod cc;
pub mod codel;
pub mod command;
pub mod dp;
pub mod image;
pub mod interpreter;
pub mod stdin;

use std::collections::HashSet;
use std::error::Error;

use crate::args::Args;
use crate::command::Command;
use crate::image::Image;
use crate::interpreter::Interpreter;

fn debug_print(is_verbose_mode: bool, s: &str) {
    if is_verbose_mode {
        eprintln!("{}", s);
    }
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let img = Image::new(&args.image_file, args.codel_size)?;
    debug_print(args.verbose, &format!("{}", img));

    let mut ip = Interpreter::new();

    loop {
        debug_print(args.verbose, &format!("{:?}", ip.cur));
        let cur_codel = img.get_codel_at(ip.cur);
        assert!(!cur_codel.is_black());
        if !cur_codel.is_white() {
            let iter_max = 7; //changes `dp` or `cc` at most 7 times
            for i in 0..=iter_max {
                let next_index = img.get_next_codel_index(ip.cur, &ip.dp, &ip.cc);
                // debug_print(args.verbose, &format!("  {:?}", next_index));
                if next_index.is_none() {
                    if i % 2 == 0 {
                        ip.cc = ip.cc.flip();
                    } else {
                        ip.dp = ip.dp.next();
                    }
                    if i == iter_max {
                        return Ok(());
                    }
                    continue;
                }
                let next_codel = img.get_codel_at(next_index.unwrap());
                if next_codel.is_black() {
                    if i % 2 == 0 {
                        ip.cc = ip.cc.flip();
                    } else {
                        ip.dp = ip.dp.next();
                    }
                    if i == iter_max {
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
                let value = img.get_block_size_at(ip.cur) as isize;
                command.apply(&mut ip, value)?;

                ip.cur = next_index.unwrap();
                break;
            }
        } else {
            let mut visited = HashSet::new();
            loop {
                let cur_codel = img.get_codel_at(ip.cur);
                if visited.contains(&(cur_codel, ip.dp)) {
                    return Ok(());
                }
                visited.insert((cur_codel, ip.dp));

                let next_index = img.get_next_codel_index_in_dp_direction(ip.cur, &ip.dp);
                if next_index.is_none() {
                    ip.cc = ip.cc.flip();
                    ip.dp = ip.dp.next();
                    continue;
                }
                let next_codel = img.get_codel_at(next_index.unwrap());
                if next_codel.is_black() {
                    ip.cc = ip.cc.flip();
                    ip.dp = ip.dp.next();
                    continue;
                }
                ip.cur = next_index.unwrap();
                break;
            }
        }
    }
}
