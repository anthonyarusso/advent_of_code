use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

const NUM_OF_FIELDS: usize = 8;

fn is_valid_height(input: &str) -> bool {
    let mut height = input.to_string();
    let mut unit: String = String::new();
    unit.push(height.pop().unwrap());
    unit.push(height.pop().unwrap());
    unit = unit.chars().rev().collect();
    let mut unit = unit.as_str();

    // run this twice in case height is parsed
    // incorrectly the first time
    for _ in 0..2 {
       if unit == "cm" {
         let height: u32 = height.parse()
             .expect("Unable to parse height!");

         return height >= 150 || height <= 193;
       } else if unit == "in" {
         let height: u32 = height.parse()
             .expect("Unable to parse height!");

         return height >= 59 || height <= 76;
       } else if unit.parse::<u32>().is_ok() {
           // no units (e.g. 'cm' / 'in') in input
           // this means we grabbed the last two
           // digits, replace them and assume
           // centimeters.
           height.push_str(unit);
           unit = "cm";
        } else {
           // Last two chars are neither digits nor
           // units. Panic.
           panic!("Invalid units provided for height!");
        }
    }
    false
}

fn is_hex(input: &str) -> bool {
    for (i, letter) in input.chars().enumerate() {
        match letter {
            '#' => {
                // is valid hexadecimal only if
                // '#' appears on the first digit
                if i != 0 {
                    return false;
                }
            },
            '0'..='9' |
            'A'..='F' |
            'a'..='f' => {
                if i == 0 {
                    return false;
                }
            },
            _ => {
                return false;
            }
        }
    }
    true
}

fn validate_data(entry: String) -> bool {
    // skip first item since the format!({} {}) method seems to produce an extra ' '
   let sundae: Vec<&str> = entry.trim_end().split(&[' ', ':'][..]).skip(1).collect();
   let pairs: Vec<(&str, &str)> = sundae.chunks(2).map(|c| (c[0], c[1])).collect();
   // index aligns to field in the match statement below. E.g. fields_present[0]
   // coincides with 'byr' which is the first entry below.
   let mut fields_present = [false; 7];

   for (i, pair) in pairs.iter().enumerate() {
       match pair.0 {
           "byr" => {
               let birth_year: u32 =
                   pair.1.parse()
                   .expect("Unable to parse birth year!");
               if pair.1.len() == 4 && (birth_year < 1920 || birth_year > 2002) {
                   return false;
               }
               if fields_present[0] {
                   // field is already present (meaning a duplicate
                   // field was found on the same entry, return false
                   return false;
               } else {
                   fields_present[0] = true;
               }
           },
           "iyr" => {
               let issue_year: u32 =
                   pair.1.parse()
                   .expect("Unable to parse issue year!");
               if pair.1.len() == 4 && (issue_year < 2010 || issue_year > 2020) {
                   return false;
               }
               if fields_present[1] {
                   return false;
               } else {
                   fields_present[1] = true;
               }
           },
           "eyr" => {
               let expire_year: u32 =
                   pair.1.parse()
                   .expect("Unable to parse expiration year!");
               if pair.1.len() == 4 && (expire_year < 2020 || expire_year > 2030) {
                   return false;
               }
               if fields_present[2] {
                   return false;
               } else {
                   fields_present[2] = true;
               }
           },
           "hgt" => {
               if !is_valid_height(pair.1) {
                   return false;
               }
               if fields_present[3] {
                   return false;
               } else {
                   fields_present[3] = true;
               }
           },
           "hcl" => {
               if !is_hex(pair.1) {
                 return false; 
               }
               if fields_present[4] {
                   return false;
               } else {
                   fields_present[4] = true;
               }
           },
           "ecl" => {
               match pair.1 {
                   "amb" |
                   "blu" |
                   "brn" |
                   "gry" |
                   "grn" |
                   "hzl" |
                   "oth" => {
                       // nothing to do
                   },
                   _ => {
                       return false;
                   }
               }
               if fields_present[5] {
                   return false;
               } else {
                   fields_present[5] = true;
               }
           },
           "pid" => {
               if pair.1.parse::<u32>().is_err() || pair.1.len() != 9 {
                   return false;
               }
               if fields_present[6] {
                   return false;
               } else {
                   fields_present[6] = true;
               }
           },
           "cid" => {
               // valid key, but we don't have to check
           },
           _ => {
               panic!("Invalid key found in key-value pair!");
           }
       }
   }

    // if any fields are missing (false) then the entry is invalid
   !fields_present.contains(&false)
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut counter = 0;
    let mut entry = String::new();

    for line in f.lines() {
       let line = line.unwrap().to_string();

       if line.trim_end().is_empty() {
           // end of entry, determine whether it's valid
           if validate_data(entry) {
               counter += 1;
           }
           entry = String::new();
       } else {
           entry = format!("{} {}", entry, line);
       }
    }
   // end of file, perform one last check on the final line
   if validate_data(entry) {
       counter += 1;
   }
    println!("counter: {}", counter);

    Ok(())
}
