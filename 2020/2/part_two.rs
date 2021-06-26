use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut counter = 0;

    for line in f.lines() {
       let line = line.unwrap().to_string();

       let sundae = line.split_once('-');
       let first: usize = sundae.unwrap().0.parse().unwrap();
       let first = first - 1;
       let sundae = sundae.unwrap().1.split_once(' ');
       let second: usize = sundae.unwrap().0.parse().unwrap();
       let second = second - 1;
       let sundae = sundae.unwrap().1.split_once(':');
       let letter: char = sundae.unwrap().0.parse().unwrap();
       let password = sundae.unwrap().1.trim_start().to_string();
       let pw_chs: Vec<char> = password.chars().collect();
       
       let is_first = pw_chs[first] == letter;
       let is_second = pw_chs[second] == letter;
       
       if (is_first || is_second) && !(is_first && is_second) {
           counter += 1;
       }
    }
    println!("counter: {}", counter);

    Ok(())
}
