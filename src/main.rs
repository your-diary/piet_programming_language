use std::collections::HashSet;
use std::error::Error;

use piet_programming_language::command::Command;
use piet_programming_language::image::Image;
use piet_programming_language::interpreter::Interpreter;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = format!(
        "{}/Downloads/power2_big.png",
        std::env::var("HOME").unwrap()
    );

    let img = Image::new(&filename)?;
    println!("{}", img);

    let mut ip = Interpreter::new();

    loop {
        println!("{:?}", ip.cur);
        let cur_codel = img.get_codel(ip.cur);
        assert!(!cur_codel.is_black());
        if (!cur_codel.is_white()) {
            let iter_max = 7; //changes `dp` or `cc` at most 7 times
            for i in 0..=iter_max {
                let next_index = img.get_next_codel_index(ip.cur, &ip.dp, &ip.cc);
                // println!("  {:?}", next_index);
                if (next_index.is_none()) {
                    if (i % 2 == 0) {
                        ip.cc = ip.cc.flip();
                    } else {
                        ip.dp = ip.dp.next();
                    }
                    if (i == iter_max) {
                        return Ok(());
                    }
                    continue;
                }
                let next_codel = img.get_codel(next_index.unwrap());
                if (next_codel.is_black()) {
                    if (i % 2 == 0) {
                        ip.cc = ip.cc.flip();
                    } else {
                        ip.dp = ip.dp.next();
                    }
                    if (i == iter_max) {
                        return Ok(());
                    }
                    continue;
                }

                if (next_codel.is_white()) {
                    ip.cur = next_index.unwrap();
                    break;
                }

                let command = Command::new(cur_codel, next_codel);
                println!("{:?}", command);
                let value = img.get_number(ip.cur);
                command.apply(&mut ip, value)?;

                ip.cur = next_index.unwrap();
                break;
            }
        } else {
            let mut visited = HashSet::new();
            loop {
                let cur_codel = img.get_codel(ip.cur);
                if (visited.contains(&(cur_codel, ip.dp))) {
                    return Ok(());
                }
                visited.insert((cur_codel, ip.dp));

                let next_index = img.get_next_codel_index_white(ip.cur, &ip.dp);
                if (next_index.is_none()) {
                    ip.cc = ip.cc.flip();
                    ip.dp = ip.dp.next();
                    continue;
                }
                let next_codel = img.get_codel(next_index.unwrap());
                if (next_codel.is_black()) {
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
