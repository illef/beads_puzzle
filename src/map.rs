//content 가 0 이면 해당 칸에 아무것도 없는것이다
use std::fmt::Display;
use std::fmt::Formatter;

use ansi_term::Colour;

pub struct Map {
    width: i32,
    height: i32,
    content: Vec<i32>,
}

impl Map {
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
