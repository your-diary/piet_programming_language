use std::{
    error::Error,
    fmt::{self, Display},
    path::Path,
    rc::Rc,
};

use image::{self, DynamicImage, ImageReader};
use itertools::Itertools;
use rustc_hash::FxHashSet;

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
    /// Prints the input image as an ASCII art.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //column numbers
        let mut s = format!(
            "   {}\n  {}\n",
            (0..self.width)
                .map(|e| format!(
                    "{:2}",
                    if e % 10 == 0 {
                        (e / 10).to_string()
                    } else {
                        " ".to_string()
                    }
                ))
                .join(""),
            (0..self.width).map(|e| format!("{:2}", e % 10)).join(""),
        );
        for i in 0..self.height {
            s += &format!("{:2} ", i); //row number
            for j in 0..self.width {
                s += &format!("{}", self.m[i][j]); //see `impl Display for Codel`
            }
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

impl Image {
    pub fn new(
        file: impl AsRef<Path>,
        codel_size: Option<usize>,
        default_color: Option<Codel>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut pixel_map = vec![];
        if !file.as_ref().exists() {
            return Err("file not found".into());
        }
        match ImageReader::open(file)?.decode()? {
            DynamicImage::ImageRgb8(img) => {
                let height = img.height();
                let width = img.width();
                for i in 0..height {
                    let mut row = Vec::with_capacity(width as usize);
                    for j in 0..width {
                        let pixel = img.get_pixel(j, i);
                        row.push(Pixel::new(pixel[0], pixel[1], pixel[2]));
                    }
                    pixel_map.push(row);
                }
            }
            DynamicImage::ImageRgba8(img) => {
                let height = img.height();
                let width = img.width();
                for i in 0..height {
                    let mut row = Vec::with_capacity(width as usize);
                    for j in 0..width {
                        let pixel = img.get_pixel(j, i);
                        row.push(Pixel::new(pixel[0], pixel[1], pixel[2]));
                    }
                    pixel_map.push(row);
                }
            }
            _ => return Err("unsupported file format".into()),
        }

        //[spec]
        //Piet code takes the form of graphics made up of the recognised colours.
        //Individual pixels of colour are significant in the language,
        //so it is common for programs to be enlarged for viewing so that the details are easily visible.
        //In such enlarged programs, the term "codel" is used to mean a block of colour equivalent to a single pixel of code,
        //to avoid confusion with the actual pixels of the enlarged graphic, of which many may make up one codel.
        let codel_size = if let Some(codel_size) = codel_size {
            if !Self::check_if_codel_size_is_valid(&pixel_map, codel_size) {
                return Err("incorrect codel size specified".into());
            }
            codel_size
        } else {
            Self::detect_codel_size(&pixel_map).ok_or("failed to detect the codel size")?
        };

        let height = pixel_map.len() / codel_size;
        let width = pixel_map[0].len() / codel_size;
        let mut m = vec![Vec::with_capacity(width); height];
        for i in 0..height {
            for j in 0..width {
                let pixel = pixel_map[i * codel_size][j * codel_size];
                let codel = Codel::new(&pixel)
                    .or(default_color)
                    .ok_or(format!("invalid color at ({}, {})", i, j))?;
                m[i].push(codel);
            }
        }

        let block_map = Self::create_block_map(&m);

        Ok(Self {
            m,
            height,
            width,
            block_map,
        })
    }

    fn check_if_codel_size_is_valid(pixel_map: &[Vec<Pixel>], codel_size: usize) -> bool {
        let height = pixel_map.len();
        let width = pixel_map[0].len();
        if (height % codel_size != 0) || (width % codel_size != 0) {
            return false;
        }
        let h = height / codel_size;
        let w = width / codel_size;
        for i in 0..h {
            for j in 0..w {
                let origin_i = i * codel_size;
                let origin_j = j * codel_size;
                let p = &pixel_map[origin_i][origin_j];
                for i in 0..codel_size {
                    for j in 0..codel_size {
                        if &pixel_map[origin_i + j][origin_j + i] != p {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    /// Automatically detects the codel size.
    /// As codel size is generally not unique, we return the largest possible codel size (if any).
    fn detect_codel_size(pixel_map: &[Vec<Pixel>]) -> Option<usize> {
        let height = pixel_map.len();
        let width = pixel_map[0].len();
        //tries all of the common divisors of `height` and `width` in descending order
        for codel_size in (1..=(height.min(width))).rev() {
            if !((width % codel_size == 0) && (height % codel_size == 0)) {
                continue;
            }
            if Self::check_if_codel_size_is_valid(pixel_map, codel_size) {
                return Some(codel_size);
            }
        }
        None
    }

    /// Splits the graph into blocks (i.e. connected components) by repeating DFS.
    /// `returned_value[i][j]` represents the block to which the codel at `(i, j)` belongs.
    /// As generally multiple pairs of `(i, j)` belong to the same block, we use `Rc`.
    ///
    /// Related [spec](https://www.dangermouse.net/esoteric/piet.html):
    ///
    /// > A colour block is a contiguous block of any number of codels of one colour, bounded by blocks of other colours or by the edge of the program graphic.
    /// > Blocks of colour adjacent only diagonally are not considered contiguous.
    ///
    fn create_block_map(m: &[Vec<Codel>]) -> Vec<Vec<Rc<Block>>> {
        let mut connected_components = vec![];
        let mut visited = FxHashSet::default();
        for i in 0..m.len() {
            for j in 0..m[0].len() {
                if visited.contains(&(i, j)) {
                    continue;
                }
                let visited_local = Self::dfs((i, j), &m[i][j], m);
                visited_local.iter().for_each(|e| {
                    visited.insert(*e);
                });
                connected_components.push(visited_local);
            }
        }

        let mut block_map = vec![vec![Rc::new(Block::default()); m[0].len()]; m.len()];
        connected_components.into_iter().for_each(|s| {
            let block = Rc::new(Block::new(&s));
            s.into_iter().for_each(|(i, j)| {
                block_map[i][j] = block.clone();
            });
        });

        block_map
    }

    /// Returns the four adjacent codels to the codel at `(i, j)`.
    fn four_adjacents((i, j): (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        if i != 0 {
            ret.push((i - 1, j));
        }
        if i != height - 1 {
            ret.push((i + 1, j));
        }
        if j != 0 {
            ret.push((i, j - 1));
        }
        if j != width - 1 {
            ret.push((i, j + 1));
        }
        ret
    }

    fn dfs(start: (usize, usize), color: &Codel, m: &[Vec<Codel>]) -> FxHashSet<(usize, usize)> {
        let mut visited = FxHashSet::default();

        let height = m.len();
        let width = m[0].len();

        let mut q = vec![start];
        while let Some(cur) = q.pop() {
            visited.insert(cur);
            Self::four_adjacents(cur, height, width)
                .into_iter()
                .filter(|e| !visited.contains(e))
                .filter(|(i, j)| &m[*i][*j] == color)
                .for_each(|e| {
                    q.push(e);
                });
        }

        visited
    }

    pub fn get_codel_at(&self, (i, j): (usize, usize)) -> &Codel {
        &self.m[i][j]
    }

    pub fn get_block_size_at(&self, (i, j): (usize, usize)) -> usize {
        self.block_map[i][j].size
    }

    /// Returns the index of the "next" codel as you traverse the blocks of the input image.
    /// See these links for more details:
    /// - <https://www.dangermouse.net/esoteric/piet.html> ("Program Execution" section)
    /// - <https://raw.githubusercontent.com/your-diary/piet_programming_language/refs/heads/master/readme_assets/spec.png>
    pub fn get_next_codel_index(
        &self,
        (i, j): (usize, usize),
        dp: &DP,
        cc: &CC,
    ) -> Option<(usize, usize)> {
        let corner = self.block_map[i][j].get_corner_index(dp, cc);
        self.get_next_codel_index_in_dp_direction(corner, dp)
    }

    /// Returns the index of the codel when you move straight one step in the direction of `dp`.
    /// `None` is returned iff the next codel is out of bounds.
    ///
    /// Related [spec](https://www.dangermouse.net/esoteric/piet.html):
    ///
    /// > Sliding across white blocks takes the interpreter in a straight line until it hits a coloured pixel or edge.
    /// > It does not use the procedure described above for determining where the interpreter emerges from non-white coloured blocks.
    pub fn get_next_codel_index_in_dp_direction(
        &self,
        (i, j): (usize, usize),
        dp: &DP,
    ) -> Option<(usize, usize)> {
        let (delta_i, delta_j) = dp.get_displacement();
        let next = (i as isize + delta_i, j as isize + delta_j);
        if (0 <= next.0)
            && (next.0 < self.height as isize)
            && (0 <= next.1)
            && (next.1 < self.width as isize)
        {
            Some((next.0 as usize, next.1 as usize))
        } else {
            None
        }
    }
}

/*-------------------------------------*/
