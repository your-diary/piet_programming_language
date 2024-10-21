use std::fmt::{self, Display};

use super::image::Pixel;

/// Codel (LightRed, Green, White, etc.)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Codel {
    LightRed,     //#FFC0C0
    LightYellow,  //#FFFFC0
    LightGreen,   //#C0FFC0
    LightCyan,    //#C0FFFF
    LightBlue,    //#C0C0FF
    LightMagenta, //#FFC0FF

    Red,     //#FF0000
    Yellow,  //#FFFF00
    Green,   //#00FF00
    Cyan,    //#00FFFF
    Blue,    //#0000FF
    Magenta, //#FF00FF

    DarkRed,     //#C00000
    DarkYellow,  //#C0C000
    DarkGreen,   //#00C000
    DarkCyan,    //#00C0C0
    DarkBlue,    //#0000C0
    DarkMagenta, //#C000C0

    White, //#FFFFFF
    Black, //#000000
}

impl Display for Codel {
    /// Prints a colored square (full-width space with a background color).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = match self {
            Codel::LightRed => (255, 192, 192),
            Codel::LightYellow => (255, 255, 192),
            Codel::LightGreen => (192, 255, 192),
            Codel::LightCyan => (192, 255, 255),
            Codel::LightBlue => (192, 192, 255),
            Codel::LightMagenta => (255, 192, 255),

            Codel::Red => (255, 0, 0),
            Codel::Yellow => (255, 255, 0),
            Codel::Green => (0, 255, 0),
            Codel::Cyan => (0, 255, 255),
            Codel::Blue => (0, 0, 255),
            Codel::Magenta => (255, 0, 255),

            Codel::DarkRed => (192, 0, 0),
            Codel::DarkYellow => (192, 192, 0),
            Codel::DarkGreen => (0, 192, 0),
            Codel::DarkCyan => (0, 192, 192),
            Codel::DarkBlue => (0, 0, 192),
            Codel::DarkMagenta => (192, 0, 192),

            Codel::White => (255, 255, 255),
            Codel::Black => (0, 0, 0),
        };
        //by the way, `38;2` can be used to change the foreground color
        write!(f, "\u{001B}[48;2;{};{};{}m　\u{001B}[0m", r, g, b)
    }
}

impl Codel {
    pub fn new(p: &Pixel) -> Option<Self> {
        match p {
            #[rustfmt::skip]
            Pixel { r: 255, g: 192, b: 192 } => Some(Codel::LightRed),
            #[rustfmt::skip]
            Pixel { r: 255, g: 255, b: 192 } => Some(Codel::LightYellow),
            #[rustfmt::skip]
            Pixel { r: 192, g: 255, b: 192 } => Some(Codel::LightGreen),
            #[rustfmt::skip]
            Pixel { r: 192, g: 255, b: 255 } => Some(Codel::LightCyan),
            #[rustfmt::skip]
            Pixel { r: 192, g: 192, b: 255 } => Some(Codel::LightBlue),
            #[rustfmt::skip]
            Pixel { r: 255, g: 192, b: 255 } => Some(Codel::LightMagenta),

            #[rustfmt::skip]
            Pixel { r: 255, g: 0, b: 0 } => Some(Codel::Red),
            #[rustfmt::skip]
            Pixel { r: 255, g: 255, b: 0 } => Some(Codel::Yellow),
            #[rustfmt::skip]
            Pixel { r: 0, g: 255, b: 0 } => Some(Codel::Green),
            #[rustfmt::skip]
            Pixel { r: 0, g: 255, b: 255 } => Some(Codel::Cyan),
            #[rustfmt::skip]
            Pixel { r: 0, g: 0, b: 255 } => Some(Codel::Blue),
            #[rustfmt::skip]
            Pixel { r: 255, g: 0, b: 255 } => Some(Codel::Magenta),

            #[rustfmt::skip]
            Pixel { r: 192, g: 0, b: 0 } => Some(Codel::DarkRed),
            #[rustfmt::skip]
            Pixel { r: 192, g: 192, b: 0 } => Some(Codel::DarkYellow),
            #[rustfmt::skip]
            Pixel { r: 0, g: 192, b: 0 } => Some(Codel::DarkGreen),
            #[rustfmt::skip]
            Pixel { r: 0, g: 192, b: 192 } => Some(Codel::DarkCyan),
            #[rustfmt::skip]
            Pixel { r: 0, g: 0, b: 192 } => Some(Codel::DarkBlue),
            #[rustfmt::skip]
            Pixel { r: 192, g: 0, b: 192 } => Some(Codel::DarkMagenta),

            #[rustfmt::skip]
            Pixel { r: 255, g: 255, b: 255 } => Some(Codel::White),
            #[rustfmt::skip]
            Pixel { r: 0, g: 0, b: 0 } => Some(Codel::Black),

            _ => None,
        }
    }

    pub fn is_black(&self) -> bool {
        self == &Codel::Black
    }

    pub fn is_white(&self) -> bool {
        self == &Codel::White
    }

    //ref: https://www.dangermouse.net/esoteric/piet.html
    fn get_hue(&self) -> usize {
        match self {
            Codel::LightRed | Codel::Red | Codel::DarkRed => 0,
            Codel::LightYellow | Codel::Yellow | Codel::DarkYellow => 1,
            Codel::LightGreen | Codel::Green | Codel::DarkGreen => 2,
            Codel::LightCyan | Codel::Cyan | Codel::DarkCyan => 3,
            Codel::LightBlue | Codel::Blue | Codel::DarkBlue => 4,
            Codel::LightMagenta | Codel::Magenta | Codel::DarkMagenta => 5,
            _ => unreachable!(),
        }
    }

    //ref: https://www.dangermouse.net/esoteric/piet.html
    fn get_lightness(&self) -> usize {
        match self {
            Codel::LightRed
            | Codel::LightYellow
            | Codel::LightGreen
            | Codel::LightCyan
            | Codel::LightBlue
            | Codel::LightMagenta => 0,
            Codel::Red
            | Codel::Yellow
            | Codel::Green
            | Codel::Cyan
            | Codel::Blue
            | Codel::Magenta => 1,
            Codel::DarkRed
            | Codel::DarkYellow
            | Codel::DarkGreen
            | Codel::DarkCyan
            | Codel::DarkBlue
            | Codel::DarkMagenta => 2,
            _ => unreachable!(),
        }
    }

    pub fn get_hue_difference(from: &Codel, to: &Codel) -> usize {
        let from = from.get_hue();
        let to = to.get_hue();
        (to + 6 - from) % 6
    }

    pub fn get_lightness_difference(from: &Codel, to: &Codel) -> usize {
        let from = from.get_lightness();
        let to = to.get_lightness();
        (to + 3 - from) % 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test01() {
        assert_eq!(0, Codel::get_hue_difference(&Codel::Red, &Codel::LightRed));
        assert_eq!(1, Codel::get_hue_difference(&Codel::Red, &Codel::Yellow));
        assert_eq!(4, Codel::get_hue_difference(&Codel::Blue, &Codel::Green));
    }

    #[test]
    // #[ignore]
    fn test02() {
        assert_eq!(
            0,
            Codel::get_lightness_difference(&Codel::LightRed, &Codel::LightYellow)
        );
        assert_eq!(
            1,
            Codel::get_lightness_difference(&Codel::LightRed, &Codel::Yellow)
        );
        assert_eq!(
            2,
            Codel::get_lightness_difference(&Codel::Red, &Codel::LightGreen)
        );
    }
}
