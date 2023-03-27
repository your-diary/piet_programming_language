use std::collections::HashSet;
use std::error::Error;

use piet_programming_language::cc::CC;
use piet_programming_language::dp::DP;
use piet_programming_language::image::Image;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = format!(
        "{}/Downloads/power2_big.png",
        std::env::var("HOME").unwrap()
    );
    let img = Image::new(&filename)?;
    println!("{}", img);

    let mut cur = (0, 0);
    let mut dp = DP::Right;
    let mut cc = CC::Left;

    let mut stack: Vec<isize> = vec![];

    loop {
        println!("{:?}", cur);
        let cur_codel = img.get_codel(cur);
        assert!(!cur_codel.is_black());
        if (!cur_codel.is_white()) {
            for i in 0..8 {
                let next_index = img.get_next_codel_index(cur, &dp, &cc);
                println!("  {:?}", next_index);
                if (next_index.is_none()) {
                    if (i % 2 == 0) {
                        cc = cc.flip();
                    } else {
                        dp = dp.next();
                    }
                    if (i == 7) {
                        return Ok(());
                    }
                    continue;
                }
                let next_codel = img.get_codel(next_index.unwrap());
                if (next_codel.is_black()) {
                    if (i % 2 == 0) {
                        cc = cc.flip();
                    } else {
                        dp = dp.next();
                    }
                    if (i == 7) {
                        return Ok(());
                    }
                    continue;
                }
                cur = next_index.unwrap();
                break;
            }
        } else {
            let mut visited = HashSet::new();
            loop {
                let cur_codel = img.get_codel(cur);
                if (visited.contains(&(cur_codel, dp))) {
                    return Ok(());
                }
                visited.insert((cur_codel, dp));

                let next_index = img.get_next_codel_index_white(cur, &dp);
                if (next_index.is_none()) {
                    cc = cc.flip();
                    dp = dp.next();
                    continue;
                }
                let next_codel = img.get_codel(next_index.unwrap());
                if (next_codel.is_black()) {
                    cc = cc.flip();
                    dp = dp.next();
                    continue;
                }
                cur = next_index.unwrap();
                break;
            }
        }
    }
}
