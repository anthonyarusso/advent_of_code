use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

// subtract one to adjust for 0 index
const ROW_COUNT: usize = 31;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let f_lines: Vec<_> = f.lines().map(|line| line.unwrap_or_default()).collect();

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product: u128 =  1;

    for (right, down) in slopes.iter() {
        let mut counter = 0;
        let mut x = 0;

        for line in f_lines.clone().iter().skip(*down).step_by(*down) {
           let chs: Vec<char> = line.chars().collect();
           x = (x + right) % ROW_COUNT;

           if chs[x] == '#' {
               counter += 1;
           }
        }
        println!("========= counter: {} =========", counter);
        product *= counter as u128;
    }
    println!("===== product:  {} =====", product);

    Ok(())
}
