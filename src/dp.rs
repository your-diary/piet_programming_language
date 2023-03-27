#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum DP {
    Right,
    Down,
    Left,
    Up,
}

impl DP {
    pub fn next(&self) -> Self {
        match (self) {
            DP::Right => DP::Down,
            DP::Down => DP::Left,
            DP::Left => DP::Up,
            DP::Up => DP::Right,
        }
    }

    pub fn get_displacement(&self) -> (isize, isize) {
        match (self) {
            DP::Right => (0, 1),
            DP::Down => (1, 0),
            DP::Left => (0, -1),
            DP::Up => (-1, 0),
        }
    }
}
