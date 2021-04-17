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
        for painted_map in m.paint(block.clone()) {
            let mut next = blocks.clone();
            next.remove(block);
            if let Some(answer) = solve(painted_map, next) {
                return Some(answer);
            }
        }
    }

    None
}

fn numer_1() {
    let map = Map::new_with_fill(6, 4, vec![(0, 0), (1, 0), (5, 0), (5, 1)]);
    println!("{}", map);

    if let Some(answer) = solve(
        map,
        vec![
            Block::new(init::B4),
            Block::new(init::B2),
            Block::new(init::B6),
            Block::new(init::B8),
        ]
        .into_iter()
        .collect(),
    ) {
        println!("{}", answer);
    }
}

fn number_62() {
    let map = Map::new_with_fill(
        6,
        10,
        vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ],
    );
    println!("{}", map);

    let mut blocks = Block::get_all();
    blocks.remove(&Block::new(init::B3));
    blocks.remove(&Block::new(init::B7));

    if let Some(answer) = solve(map, blocks) {
        println!("{}", answer);
    }
}

fn main() {
    number_62();
}
