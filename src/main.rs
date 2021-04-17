#[allow(dead_code)]
mod block;

#[allow(dead_code)]
mod map;

use block::*;
use map::*;
use std::collections::BTreeSet;

fn solve(m: Map, blocks: BTreeSet<Block>) -> Option<Map> {
    if m.complete() {
        return Some(m);
    }
    if blocks.is_empty() {
        return None;
    }

    for block in blocks.iter() {
        for painted_map in m.paint(block.clone()){
            let mut next = blocks.clone();
            next.remove(block);
            if let Some(answer) = solve(painted_map, next) {
                return Some(answer);
            }
        }
    }

    None
}

fn main() {
    let mut map = Map::default();

    for i in 0..13 {
        *map.get_cell_raw(i).unwrap() = i as i32;
    }

    println!("{}", map);
}
