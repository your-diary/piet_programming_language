use num::FromPrimitive;

/**
Direction Pointer (DP)

The default value is `DP::Right` as [the spec](https://www.dangermouse.net/esoteric/piet.html) says

> The interpreter maintains a Direction Pointer (DP), initially pointing to the right.

*/
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum DP {
    #[default]
    Right,
    Down,
    Left,
    Up,
}

impl DP {
    pub fn turn_right(&self) -> Self {
        self.rotate_clockwise_by(1)
    }

    pub fn rotate_clockwise_by(&self, i: isize) -> Self {
        let i = (*self as isize) + i;
        if i >= 0 {
            Self::from_isize(i % 4).unwrap()
        } else {
            Self::from_isize((i % 4 + 4) % 4).unwrap()
        }
    }

    /// Returns the index displacement when you go straight one step in the direction of DP.
    pub fn get_displacement(&self) -> (isize, isize) {
        match self {
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Up => (-1, 0),
        }
    }
}

impl FromPrimitive for DP {
    fn from_i64(i: i64) -> Option<Self> {
        if i < 0 {
            None
        } else {
            Self::from_u64(i as u64)
        }
    }

    fn from_u64(i: u64) -> Option<Self> {
        match i {
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
    fn test01() {
        assert_eq!(DP::Right.turn_right(), DP::Down);
        assert_eq!(DP::Down.turn_right(), DP::Left);
        assert_eq!(DP::Left.turn_right(), DP::Up);
        assert_eq!(DP::Up.turn_right(), DP::Right);
    }

    #[test]
    fn test02() {
        assert_eq!(DP::Right.rotate_clockwise_by(0), DP::Right);

        assert_eq!(DP::Right.rotate_clockwise_by(1), DP::Down);
        assert_eq!(DP::Right.rotate_clockwise_by(2), DP::Left);
        assert_eq!(DP::Right.rotate_clockwise_by(3), DP::Up);
        assert_eq!(DP::Right.rotate_clockwise_by(4), DP::Right);
        assert_eq!(DP::Right.rotate_clockwise_by(5), DP::Down);
        assert_eq!(DP::Right.rotate_clockwise_by(6), DP::Left);
        assert_eq!(DP::Right.rotate_clockwise_by(7), DP::Up);
        assert_eq!(DP::Right.rotate_clockwise_by(8), DP::Right);

        assert_eq!(DP::Right.rotate_clockwise_by(-1), DP::Up);
        assert_eq!(DP::Right.rotate_clockwise_by(-2), DP::Left);
        assert_eq!(DP::Right.rotate_clockwise_by(-3), DP::Down);
        assert_eq!(DP::Right.rotate_clockwise_by(-4), DP::Right);
        assert_eq!(DP::Right.rotate_clockwise_by(-5), DP::Up);
        assert_eq!(DP::Right.rotate_clockwise_by(-6), DP::Left);
        assert_eq!(DP::Right.rotate_clockwise_by(-7), DP::Down);
        assert_eq!(DP::Right.rotate_clockwise_by(-8), DP::Right);
    }

    #[test]
    fn test03() {
        assert_eq!((0, 1), DP::Right.get_displacement());
        assert_eq!((1, 0), DP::Down.get_displacement());
        assert_eq!((0, -1), DP::Left.get_displacement());
        assert_eq!((-1, 0), DP::Up.get_displacement());
    }
}
