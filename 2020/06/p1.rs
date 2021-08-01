use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut current_group: Vec<char> = Vec::new();
    // FOR DEBUG  let mut group_count = 0;
    let mut yes_count = 0;

    for line in f.lines() {
       let line = line.unwrap().to_string();
       // line is empty, start new group
       if line.trim_end() == "" {
           // group_count += 1;
           current_group.sort_unstable();
           current_group.dedup();

           // Increase the count by the number of unique questions
           // answered "yes" by the current group.
           yes_count += current_group.len();
           current_group = Vec::new();

           /* DEBUG OUPUT
           let group_string: String = current_group.iter().collect();
           println!("group No.{}, {}", group_count, group_string);
           */
       } else {
           current_group.extend(line.trim_end().chars());
       }
    }
   current_group.sort_unstable();
   current_group.dedup();
   yes_count += current_group.len();

    println!("yes_count: {}", yes_count);

    Ok(())
}
