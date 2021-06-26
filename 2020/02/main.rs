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
       let min: usize = sundae.unwrap().0.parse().unwrap();
       let sundae = sundae.unwrap().1.split_once(' ');
       let max: usize = sundae.unwrap().0.parse().unwrap();
       let sundae = sundae.unwrap().1.split_once(':');
       let letter: char = sundae.unwrap().0.parse().unwrap();
       let password = sundae.unwrap().1.trim_start().to_string();

       let letter_count = password.matches(letter).count();
       if letter_count >= min && letter_count <= max {
           counter += 1;
       }
    }
    println!("counter: {}", counter);

    Ok(())
}
