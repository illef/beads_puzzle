//content 가 0 이면 해당 칸에 아무것도 없는것이다
use std::collections::BTreeSet;
use std::fmt::Display;
use std::fmt::Formatter;

use ansi_term::Colour;

use super::*;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Map {
    width: i32,
    height: i32,
    content: Vec<i32>,
    current_line: usize,
}

impl Map {
    fn paint_impl(&self, b: &Block) -> Option<Map> {
        // 첫번째 빈칸을 찾는다
        let idx = self.content.iter().enumerate().find(|(_, v)| **v == 0);
        if idx.is_none() {
            return None;
        }
        let (idx, _) = idx.unwrap();
        let row_num = (idx / self.width as usize) as i32;

        // 해당 블럭을 모두 색칠할 수 있는지 확인한다
        let paint_block = |w: i32| -> Option<Map> {
            let mut m = self.clone();
            for y in 0..5 {
                for x in 0..5 {
                    let v = b.cell(x, y).unwrap();
                    if v == 0 {
                        continue;
                    }
                    if let Some(dest) = m.get_cell((w + x, y + row_num)) {
                        if *dest == 0 {
                            *dest = v;
                        } else {
                            return None;
                        }
                    } else {
                        // dest가 없으므로
                        return None;
                    }
                }
            }
            Some(m)
        };

        for w in 0..self.width {
            if let Some(m) = paint_block(w) {
                return Some(m);
            }
        }

        None
    }

    pub fn paint(&self, mut b: Block) -> BTreeSet<Map> {
        let mut set = BTreeSet::default();
        for _ in 0..4 {
            b.rotate();
            b.nomalize();
            if let Some(m) = self.paint_impl(&b) {
                set.insert(m);
            }
        }
        b.re_init();
        b.reverse();
        for _ in 0..4 {
            b.rotate();
            b.nomalize();
            if let Some(m) = self.paint_impl(&b) {
                set.insert(m);
            }
        }
        set
    }

    pub fn new(width: i32, height: i32, first_line: &[i32]) -> Map {
        let mut m = Map {
            width,
            height,
            content: std::iter::repeat(0)
                .take(width as usize * height as usize)
                .collect(),
            current_line: 0,
        };

        for (src, dest) in first_line.iter().zip(m.content.iter_mut()) {
            *dest = *src
        }

        m
    }

    pub fn is_valid(&self) -> bool {
        self.width * self.height == self.content.len() as i32
    }

    pub fn get_cell_raw(&mut self, idx: usize) -> Option<&mut i32> {
        self.content.get_mut(idx)
    }

    pub fn get_cell(&mut self, (x, y): (i32, i32)) -> Option<&mut i32> {
        if x < 0 || self.width <= x {
            None
        } else if y < 0 || self.height <= y {
            None
        } else {
            Some(&mut self.content[(y * self.width + x) as usize])
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map {
            width: 6,
            height: 10,
            content: std::iter::repeat(0).take(6 * 10).collect(),
            current_line: 0,
        }
    }
}

fn get_color_idx(color_code: i32, color_count: i32) -> Colour {
    let range = 240 / color_count;
    let color = range * color_code;

    Colour::Fixed(color as u8)
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for i in 0..self.content.len() {
            match self.content[i] {
                0 => write!(f, "{}", Colour::RGB(44, 50, 60).paint("●"))?,
                v => write!(f, "{}", get_color_idx(v, 12).paint("●"))?,
            }

            if (i + 1) % self.width as usize == 0 {
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_map_test() {
        let m = Map::new(3, 3, &[1, 2, 3, 4]);
        assert_eq!(m.content, vec![1, 2, 3, 4, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn paint_test() {
        {
            let m = Map::new(6, 2, &[]);
            let set = m.paint(Block::new(init::B0));
            assert_eq!(set.len(), 1);
            assert_eq!(
                set.into_iter().next().unwrap().content,
                vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]
            );
        }
        {
            let m = Map::new(6, 2, &[2]);
            let set = m.paint(Block::new(init::B0));
            assert_eq!(set.len(), 1);
            assert_eq!(
                set.into_iter().next().unwrap().content,
                vec![2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]
            );
        }
        {
            let m = Map::new(6, 4, &[3, 3]);
            let set = m.paint(Block::new(init::B1));
            assert_eq!(8, set.len())
        }

        {
            let m = Map::new(6, 4, &[3, 3]);
            let set = m.paint(Block::new(init::B7));
            assert_eq!(1, set.len())
        }
    }
}
