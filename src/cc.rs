pub enum CC {
    Left,
    Right,
}

impl CC {
    pub fn flip(&self) -> Self {
        match (self) {
            CC::Left => CC::Right,
            CC::Right => CC::Left,
        }
    }
}
