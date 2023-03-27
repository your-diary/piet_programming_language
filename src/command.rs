use super::codel::Codel;

#[derive(Debug)]
pub enum Command {
    Push,
    Pop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Not,
    Greater,
    Pointer,
    Switch,
    Duplicate,
    Roll,
    ReadNumber,
    ReadChar,
    WriteNumber,
    WriteChar,
}

impl Command {
    pub fn new(from: &Codel, to: &Codel) -> Self {
        let hue_difference = Codel::get_hue_difference(from, to);
        let lightness_difference = Codel::get_lightness_difference(from, to);
        match ((hue_difference, lightness_difference)) {
            (0, 1) => Command::Push,
            (0, 2) => Command::Pop,

            (1, 0) => Command::Add,
            (1, 1) => Command::Subtract,
            (1, 2) => Command::Multiply,

            (2, 0) => Command::Divide,
            (2, 1) => Command::Mod,
            (2, 2) => Command::Not,

            (3, 0) => Command::Greater,
            (3, 1) => Command::Pointer,
            (3, 2) => Command::Switch,

            (4, 0) => Command::Duplicate,
            (4, 1) => Command::Roll,
            (4, 2) => Command::ReadNumber,

            (5, 0) => Command::ReadChar,
            (5, 1) => Command::WriteNumber,
            (5, 2) => Command::WriteChar,

            _ => unreachable!(),
        }
    }
}
