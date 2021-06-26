use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Block {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}
impl Block {
    fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Block { x, y, z, w }
    }
    fn is_neighbors_with(&self, other: &Block) -> bool {
        // all coordinates are within 1 unit of each other
        (self.x - other.x).abs() <= 1 &&
            (self.y - other.y).abs() <= 1 &&
            (self.z - other.z).abs() <= 1 &&
            (self.w - other.w).abs() <= 1 &&
            // but is not the same block
            !(self == other)
    }
    fn check_status(&self, active_blocks: &Vec<Block>) -> bool {
        let mut neighbor_count = 0;
        // does the block start this frame active?
        let known_active = active_blocks.iter().any(|b| *b == *self);

        for block in active_blocks.iter() {
            if self.is_neighbors_with(&block) {
                neighbor_count += 1;
            }
        }

        (known_active && (neighbor_count == 2 || neighbor_count == 3)) ||
            (!known_active && neighbor_count == 3)
    }
}

fn main() {
    let mut active_blocks: Vec<Block> = Vec::new();
    let mut next_active_blocks: Vec<Block> = Vec::new();
    let mut checked_blocks: Vec<Block> = Vec::new();
    let mut cycle = 0;

    // increment y for every line, and x for every character
    let (mut coord_x, mut coord_y): (i32, i32) = (0, 0);

    let file = File::open("17_input.txt").unwrap();
    let mut reader = io::BufReader::new(file);
    let mut buf = String::new();
    println!("input file reads: ");
    while reader.read_line(&mut buf).unwrap() > 0 {
        let line = buf.trim_end();
        println!("{}", line);
        for c in buf.chars() {
            if c == '#' {
                active_blocks.push(Block::new(coord_x, coord_y, 0, 0));
            }
            coord_x += 1;
        }
        coord_y += 1;
        coord_x = 0;
        buf.clear();
    }

    for _ in 0..6 {
    println!("cycle: {} | active_blocks.len(): {}", cycle,  active_blocks.len());

    for block in active_blocks.iter() {
        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    for m in -1..2 {
                        let current_block = Block::new(
                            block.x + i,
                            block.y + j,
                            block.z + k,
                            block.w + m);

                        if !checked_blocks.iter().any(|x| *x == current_block) {
                            if current_block.check_status(&active_blocks) {
                                next_active_blocks.push(current_block.clone());
                            }
                            checked_blocks.push(current_block);
                        }
                    }
                }
            }
        }
    } // end for active_blocks

    active_blocks = next_active_blocks.clone();
    next_active_blocks.clear();
    checked_blocks.clear();
    cycle += 1;
    } // end of cycle
    println!("cycle: {} | active_blocks.len(): {}", cycle,  active_blocks.len());
}
