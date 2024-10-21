use itertools::Itertools;
use rustc_hash::FxHashSet;

use super::cc::CC;
use super::dp::DP;

/**
Colour block.

In more general terms, a block is a connected component of a graph.

Related [spec](https://www.dangermouse.net/esoteric/piet.html):

> A colour block is a contiguous block of any number of codels of one colour, bounded by blocks of other colours or by the edge of the program graphic.
> Blocks of colour adjacent only diagonally are not considered contiguous.

*/
#[derive(Debug, Default, Clone)]
pub struct Block {
    /**
    Number of codels in the block.

    This is used as an integer literal as [the spec](https://www.dangermouse.net/esoteric/piet.html) says

    > Each non-black, non-white colour block in a Piet program represents an integer equal to the number of codels in that block.
    > Note that non-positive integers cannot be represented, although they can be constructed with operators.
    > When the interpreter encounters a number, it does not necessarily do anything with it.
    > In particular, it is not automatically pushed on to the stack - there is an explicit command for that (see below).
    */
    pub size: usize,

    //indices of the 8 corners
    //The naming convention is `<dp>_<cc>` (see `DP` struct and `CC` struct).
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
    /// Creates a new block from the list of the codels in the same connected component.
    pub fn new(s: &FxHashSet<(usize, usize)>) -> Self {
        let i_min = s.iter().min_by_key(|(i, _)| i).unwrap().0;
        let i_max = s.iter().max_by_key(|(i, _)| i).unwrap().0;
        let j_min = s.iter().min_by_key(|(_, j)| j).unwrap().1;
        let j_max = s.iter().max_by_key(|(_, j)| j).unwrap().1;
        Self {
            size: s.len(),
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

    pub fn get_corner_index(&self, dp: &DP, cc: &CC) -> (usize, usize) {
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
        let s = FxHashSet::from_iter(l);
        let block = Block::new(&s);
        assert_eq!(block.size, 19);
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
