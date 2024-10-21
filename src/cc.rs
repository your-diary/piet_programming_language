/// Codel Chooser (CC)
#[derive(Debug, PartialEq)]
pub enum CC {
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
