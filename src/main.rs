#[allow(dead_code)]
mod block;

#[allow(dead_code)]
mod map;

use block::*;
use map::*;

fn main() {
    let mut map = Map::default();

    for i in 0..13 {
        *map.get_cell_raw(i).unwrap() = i as i32;
    }

    println!("{}", map);
}
