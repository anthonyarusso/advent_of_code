use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut current_group: Vec<char> = Vec::new();
    let mut yes_count = 0;
    // Boolean to denote if this line is the first member in a group.
    let mut first_member = true;

    for line in f.lines() {
       let line = line.unwrap().to_string();

       // line is empty, start new group
       if line.trim_end() == "" {
           first_member = true;

           // Increase the count by the number of questions answered
           // "yes" by everyone in the current group.
           yes_count += current_group.len();
           current_group = Vec::new();

       } else if first_member {
           first_member = false;
           // First line in the new group. Add all responses.
           current_group.extend(line.trim_end().chars());

       } else {
           // Filter out any responses that the next person in the
           // group did not answer "yes" to.
           current_group.retain(|c| {
               line.contains(|l| l == *c)
           });
       }
    }

    // In case input does not end in a newline. Run again.
    yes_count += current_group.len();
    println!("yes_count: {}", yes_count);

    Ok(())
}
