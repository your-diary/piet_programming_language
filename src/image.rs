use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display},
    rc::Rc,
};

use image::{self, io::Reader, DynamicImage};
use itertools::Itertools;

/*-------------------------------------*/

/* Codel */

#[derive(Clone, Copy, Debug, PartialEq)]
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
        write!(f, "\u{001B}[48;2;{};{};{}m　\u{001B}[0m", r, g, b)
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

/* Block */

#[derive(Debug, Default, Clone)]
struct Block {
    num_codal: usize,

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
    fn new(s: &HashSet<(usize, usize)>) -> Self {
        let i_min = s.iter().min_by_key(|(i, _)| i).unwrap().0;
        let i_max = s.iter().max_by_key(|(i, _)| i).unwrap().0;
        let j_min = s.iter().min_by_key(|(_, j)| j).unwrap().1;
        let j_max = s.iter().max_by_key(|(_, j)| j).unwrap().1;
        Self {
            num_codal: s.len(),
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
        let s = HashSet::from_iter(l);
        let block = Block::new(&s);
        assert_eq!(block.num_codal, 19);
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

/*-------------------------------------*/

/* Image */

pub struct Image {
    m: Vec<Vec<Codel>>,
    block_map: Vec<Vec<Rc<Block>>>,
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

        let codel_size =
            Self::detect_codel_size(&pixels).ok_or("failed to detect the codel size")?;

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

        let block_map = Self::create_block_map(&m);

        Ok(Self { m, block_map })
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

    fn create_block_map(m: &Vec<Vec<Codel>>) -> Vec<Vec<Rc<Block>>> {
        let mut connected_components = vec![];
        let mut visited = HashSet::new();
        for i in 0..m.len() {
            for j in 0..m[0].len() {
                if (visited.contains(&(i, j))) {
                    continue;
                }
                let mut visited_local = HashSet::new();
                Self::dfs((i, j), &m[i][j], m, &mut visited_local);
                visited_local.iter().for_each(|e| {
                    visited.insert(*e);
                });
                connected_components.push(visited_local);
            }
        }

        let mut block_map = vec![vec![Rc::new(Block::default()); m[0].len()]; m.len()];
        connected_components.iter().for_each(|s| {
            let block = Rc::new(Block::new(s));
            s.iter().for_each(|(i, j)| {
                block_map[*i][*j] = block.clone();
            });
        });

        block_map
    }

    fn four_adjacents((i, j): (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        if (i != 0) {
            ret.push((i - 1, j));
        }
        if (i != height - 1) {
            ret.push((i + 1, j));
        }
        if (j != 0) {
            ret.push((i, j - 1));
        }
        if (j != width - 1) {
            ret.push((i, j + 1));
        }
        ret
    }

    fn dfs(
        cur: (usize, usize),
        color: &Codel,
        m: &Vec<Vec<Codel>>,
        visited: &mut HashSet<(usize, usize)>,
    ) {
        visited.insert(cur);
        let height = m.len();
        let width = m[0].len();
        let adjacents = Self::four_adjacents(cur, height, width)
            .into_iter()
            .filter(|e| !visited.contains(e))
            .filter(|(i, j)| &m[*i][*j] == color)
            .collect_vec();
        for &adj in &adjacents {
            Self::dfs(adj, color, m, visited);
        }
    }
}

/*-------------------------------------*/
