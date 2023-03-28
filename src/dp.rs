use num::FromPrimitive;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum DP {
    Right,
    Down,
    Left,
    Up,
}

impl DP {
    pub fn next(&self) -> Self {
        self.rotate_by(1)
    }

    pub fn rotate_by(&self, i: isize) -> Self {
        let i = (*self as isize) + i;
        if (i >= 0) {
            Self::from_isize(i % 4).unwrap()
        } else {
            Self::from_isize((i % 4 + 4) % 4).unwrap()
        }
    }

    pub fn get_displacement(&self) -> (isize, isize) {
        match (self) {
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Up => (-1, 0),
        }
    }
}

impl FromPrimitive for DP {
    fn from_i64(i: i64) -> Option<Self> {
        if (i < 0) {
            None
        } else {
            Self::from_u64(i as u64)
        }
    }

    fn from_u64(i: u64) -> Option<Self> {
        match (i) {
            0 => Some(DP::Right),
            1 => Some(DP::Down),
            2 => Some(DP::Left),
            3 => Some(DP::Up),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test01() {
        assert_eq!(DP::Right.next(), DP::Down);
        assert_eq!(DP::Down.next(), DP::Left);
        assert_eq!(DP::Left.next(), DP::Up);
        assert_eq!(DP::Up.next(), DP::Right);
    }

    #[test]
    // #[ignore]
    fn test02() {
        assert_eq!(DP::Right.rotate_by(0), DP::Right);

        assert_eq!(DP::Right.rotate_by(1), DP::Down);
        assert_eq!(DP::Right.rotate_by(2), DP::Left);
        assert_eq!(DP::Right.rotate_by(3), DP::Up);
        assert_eq!(DP::Right.rotate_by(4), DP::Right);
        assert_eq!(DP::Right.rotate_by(5), DP::Down);
        assert_eq!(DP::Right.rotate_by(6), DP::Left);
        assert_eq!(DP::Right.rotate_by(7), DP::Up);
        assert_eq!(DP::Right.rotate_by(8), DP::Right);

        assert_eq!(DP::Right.rotate_by(-1), DP::Up);
        assert_eq!(DP::Right.rotate_by(-2), DP::Left);
        assert_eq!(DP::Right.rotate_by(-3), DP::Down);
        assert_eq!(DP::Right.rotate_by(-4), DP::Right);
        assert_eq!(DP::Right.rotate_by(-5), DP::Up);
        assert_eq!(DP::Right.rotate_by(-6), DP::Left);
        assert_eq!(DP::Right.rotate_by(-7), DP::Down);
        assert_eq!(DP::Right.rotate_by(-8), DP::Right);
    }
}
