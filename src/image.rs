use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display},
    rc::Rc,
};

use image::{self, io::Reader, DynamicImage};
use itertools::Itertools;

use super::block::Block;
use super::cc::CC;
use super::codel::Codel;
use super::dp::DP;

/*-------------------------------------*/

/* Pixel */

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
    height: usize,
    width: usize,
    block_map: Vec<Vec<Rc<Block>>>,
}

impl Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!(
            "   {}\n  {}\n",
            (0..self.width)
                .map(|e| format!(
                    "{:2}",
                    if (e % 10 == 0) {
                        (e / 10).to_string()
                    } else {
                        " ".to_string()
                    }
                ))
                .join(""),
            (0..self.width).map(|e| format!("{:2}", e % 10)).join(""),
        );
        for i in 0..self.height {
            s += &format!("{:2} ", i);
            for j in 0..self.width {
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

        let height = m.len();
        let width = m[0].len();

        Ok(Self {
            m,
            height,
            width,
            block_map,
        })
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

    pub fn get_codel(&self, (i, j): (usize, usize)) -> &Codel {
        &self.m[i][j]
    }

    pub fn get_number(&self, (i, j): (usize, usize)) -> isize {
        self.block_map[i][j].num_codel
    }

    pub fn get_next_codel_index(
        &self,
        (i, j): (usize, usize),
        dp: &DP,
        cc: &CC,
    ) -> Option<(usize, usize)> {
        let corner = self.block_map[i][j].get_corner(dp, cc);
        self.get_next_codel_index_white(corner, dp)
    }

    pub fn get_next_codel_index_white(
        &self,
        origin: (usize, usize),
        dp: &DP,
    ) -> Option<(usize, usize)> {
        let displacement = dp.get_displacement();
        let next = (
            origin.0 as isize + displacement.0,
            origin.1 as isize + displacement.1,
        );
        if ((0 <= next.0)
            && (next.0 < self.height as isize)
            && (0 <= next.1)
            && (next.1 < self.width as isize))
        {
            Some((next.0 as usize, next.1 as usize))
        } else {
            None
        }
    }
}

/*-------------------------------------*/
