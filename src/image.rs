use image::{self, io::Reader, DynamicImage};
use std::{
    error::Error,
    fmt::{self, Display},
};

/*-------------------------------------*/

/* Codel */

#[derive(Clone, Copy, Debug)]
enum Codel {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = match (self) {
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
        write!(f, "\u{001B}[48;2;{};{};{}m{}\u{001B}[0m", r, g, b, "　")
    }
}

impl Codel {
    fn new(p: &Pixel) -> Option<Self> {
        match (p) {
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
}

/*-------------------------------------*/

/* Pixel */

#[derive(PartialEq, Clone, Copy, Debug)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/*-------------------------------------*/

/* Image */

pub struct Image {
    m: Vec<Vec<Codel>>,
}

impl Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for i in 0..self.m.len() {
            for j in 0..self.m[0].len() {
                s += &format!("{}", self.m[i][j]);
            }
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

impl Image {
    pub fn new(file: &str) -> Result<Self, Box<dyn Error>> {
        let mut pixels = vec![];
        match Reader::open(file)?.decode()? {
            DynamicImage::ImageRgb8(img) => {
                let height = img.height();
                let width = img.width();
                for i in 0..height {
                    pixels.push(vec![]);
                    for j in 0..width {
                        let pixel = img.get_pixel(j, i);
                        pixels
                            .last_mut()
                            .unwrap()
                            .push(Pixel::new(pixel[0], pixel[1], pixel[2]));
                    }
                }
            }
            _ => return Err("unsupported file format".into()),
        }

        let codel_size = detect_codel_size(&pixels).ok_or("failed to detect the codel size")?;

        let height = pixels.len() / codel_size;
        let width = pixels[0].len() / codel_size;
        let mut pixels_scaled = vec![vec![]; height];
        for i in 0..height {
            for j in 0..width {
                pixels_scaled[i].push(pixels[i * codel_size][j * codel_size]);
            }
        }

        let mut m = vec![vec![]; pixels_scaled.len()];
        for i in 0..pixels_scaled.len() {
            for j in 0..pixels_scaled[0].len() {
                let codel = Codel::new(&pixels_scaled[i][j])
                    .ok_or(format!("invalid color at ({}, {})", i, j))?;
                m[i].push(codel);
            }
        }

        Ok(Self { m: m })
    }
}

fn detect_codel_size(m: &Vec<Vec<Pixel>>) -> Option<usize> {
    let height = m.len();
    let width = m[0].len();
    //tries all of the common divisors of `height` and `width` in descending order
    'a: for codel_size in (1..=(height.min(width))).rev() {
        if !((width % codel_size == 0) && (height % codel_size == 0)) {
            continue;
        }
        let h = height / codel_size;
        let w = width / codel_size;
        for i in 0..h {
            for j in 0..w {
                let origin_x = j * codel_size;
                let origin_y = i * codel_size;
                let p = &m[origin_y][origin_x];
                for x in 0..codel_size {
                    for y in 0..codel_size {
                        if (&m[origin_y + y][origin_x + x] != p) {
                            continue 'a;
                        }
                    }
                }
            }
        }
        return Some(codel_size);
    }
    None
}

/*-------------------------------------*/
