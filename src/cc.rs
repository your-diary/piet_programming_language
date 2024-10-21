/**
Codel Chooser (CC)

The default value is `CC::Left` as [the spec](https://www.dangermouse.net/esoteric/piet.html) says

> The interpreter also maintains a Codel Chooser (CC), initially pointing left.

*/
#[derive(Debug, PartialEq, Default)]
pub enum CC {
    #[default]
    Left,
    Right,
}

impl CC {
    /// Returns the flipped direction.
    pub fn flip(&self) -> Self {
        match self {
            CC::Left => CC::Right,
            CC::Right => CC::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(CC::Right, CC::Left.flip());
        assert_eq!(CC::Left, CC::Right.flip());
    }
}
