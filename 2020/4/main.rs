use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

const NUM_OF_FIELDS: usize = 8;

/*
struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: u32,
    hcl: String,
    ecl: String,
    pid: u32,
    cid: u32
}
*/

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    // let mut passports: Vec<Passport> = Vec::new();

    let mut counter = 0;
    let mut field_count = 0;
    let mut contains_cid = false;

    let f_lines = f.lines();

    for line in f_lines {
       let line = line.unwrap().to_string();

       if line.trim_end().is_empty() {
           // end of entry, determine whether it's valid
           if field_count == NUM_OF_FIELDS ||
                (field_count == (NUM_OF_FIELDS - 1) && !contains_cid) {
               counter += 1;
           }
           // reset variables for next entry
           field_count = 0;
           contains_cid = false;
       } else {
           let sundae: Vec<&str> = line.trim_end().split(&[' ', ':'][..]).collect();
           if sundae.contains(&"cid") {
               contains_cid = true;
           }
           let pairs: Vec<(&str, &str)> = sundae.chunks(2).map(|c| (c[0], c[1])).collect();
           // increment field_count by the number of fields
           // found on this line (which is the number key-value
           // pairs).
           field_count += pairs.len();
       }
    }
   // end of file, perform one last check on the final line
   if field_count == NUM_OF_FIELDS ||
        (field_count == (NUM_OF_FIELDS - 1) && !contains_cid) {
       counter += 1;
   }
    println!("counter: {}", counter);

    Ok(())
}
