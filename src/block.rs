use std::collections::HashSet;

use itertools::Itertools;

use super::cc::CC;
use super::dp::DP;

#[derive(Debug, Default, Clone)]
pub struct Block {
    num_codal: usize,

    right_left: (usize, usize),
    right_right: (usize, usize),
    down_left: (usize, usize),
    down_right: (usize, usize),
    left_left: (usize, usize),
    left_right: (usize, usize),
    up_left: (usize, usize),
    up_right: (usize, usize),
}

impl Block {
    pub fn new(s: &HashSet<(usize, usize)>) -> Self {
        let i_min = s.iter().min_by_key(|(i, _)| i).unwrap().0;
        let i_max = s.iter().max_by_key(|(i, _)| i).unwrap().0;
        let j_min = s.iter().min_by_key(|(_, j)| j).unwrap().1;
        let j_max = s.iter().max_by_key(|(_, j)| j).unwrap().1;
        Self {
            num_codal: s.len(),
            #[rustfmt::skip]
            right_left: *s.iter().filter(|(_, j)| *j == j_max).sorted().next().unwrap(),
            #[rustfmt::skip]
            right_right: *s.iter().filter(|(_, j)| *j == j_max).sorted().last().unwrap(),
            #[rustfmt::skip]
            down_left: *s.iter().filter(|(i, _)| *i == i_max).sorted().last().unwrap(),
            #[rustfmt::skip]
            down_right: *s.iter().filter(|(i, _)| *i == i_max).sorted().next().unwrap(),
            #[rustfmt::skip]
            left_left: *s.iter().filter(|(_, j)| *j == j_min).sorted().last().unwrap(),
            #[rustfmt::skip]
            left_right: *s.iter().filter(|(_, j)| *j == j_min).sorted().next().unwrap(),
            #[rustfmt::skip]
            up_left: *s.iter().filter(|(i, _)| *i == i_min).sorted().next().unwrap(),
            #[rustfmt::skip]
            up_right: *s.iter().filter(|(i, _)| *i == i_min).sorted().last().unwrap(),
        }
    }

    pub fn get_corner(&self, dp: &DP, cc: &CC) -> (usize, usize) {
        match (dp, cc) {
            (DP::Right, CC::Left) => self.right_left,
            (DP::Right, CC::Right) => self.right_right,
            (DP::Down, CC::Left) => self.down_left,
            (DP::Down, CC::Right) => self.down_right,
            (DP::Left, CC::Left) => self.left_left,
            (DP::Left, CC::Right) => self.left_right,
            (DP::Up, CC::Left) => self.up_left,
            (DP::Up, CC::Right) => self.up_right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //   ■   ■
    // ■ ■ ■ ■ ■ ■
    //   ■ ■ ■
    // ■ ■ ■ ■ ■ ■
    //   ■   ■
    #[test]
    fn test01() {
        let l = vec![
            (0, 1),
            (0, 3),
            (1, 0),
            (1, 1),
            (1, 2),
            (1, 3),
            (1, 4),
            (1, 5),
            (2, 1),
            (2, 2),
            (2, 3),
            (3, 0),
            (3, 1),
            (3, 2),
            (3, 3),
            (3, 4),
            (3, 5),
            (4, 1),
            (4, 3),
        ];
        let s = HashSet::from_iter(l);
        let block = Block::new(&s);
        assert_eq!(block.num_codal, 19);
        assert_eq!(block.right_left, (1, 5));
        assert_eq!(block.right_right, (3, 5));
        assert_eq!(block.down_left, (4, 3));
        assert_eq!(block.down_right, (4, 1));
        assert_eq!(block.left_left, (3, 0));
        assert_eq!(block.left_right, (1, 0));
        assert_eq!(block.up_left, (0, 1));
        assert_eq!(block.up_right, (0, 3));
    }
}
