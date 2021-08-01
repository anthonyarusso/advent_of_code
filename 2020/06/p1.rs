use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut current_group: Vec<char> = Vec::new();
    let mut group_count = 0;
    let mut yes_count = 0;

    for (line_count, line) in f.lines().enumerate() {
       let line = line.unwrap().to_string();
       // line is empty, start new group
       if line.trim_end() == "" {
           group_count += 1;
           current_group.sort_unstable();
           current_group.dedup();

           // Increase the count by the number of unique questions
           // answered "yes" by the current group.
           yes_count += current_group.len();

           /* DEBUG OUPUT
           let group_string: String = current_group.iter().collect();
           println!("group No.{}, {}", group_count, group_string);
           */

           current_group = Vec::new();
       } else {
           current_group.extend(line.trim_end().chars());
       }
    }

    // Increment the count by the last group
   group_count += 1;
   current_group.sort_unstable();
   current_group.dedup();
   yes_count += current_group.len();
   current_group = Vec::new();

    println!("yes_count: {}", yes_count);

    Ok(())
}
