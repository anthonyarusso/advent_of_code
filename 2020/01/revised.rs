use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;

fn find_addends(sum: u32, parts: u32, values: &Vec<u32>) -> Vec<u32> {
    let mut smalls: Vec<u32> = Vec::with_capacity(30);
    let mut bigs: Vec<u32> = Vec::with_capacity(200);
    let mut perfects = 0;
    let portion: f32 = sum as f32 / parts as f32;

    // sort values
    for x in values.iter() {
        // first we test equality which will only be true if portion
        // is a whole number and x and 'parts' are factors of sum.
        if (*x as f32) == portion {
            // we need to consider cases such as p + b + s
            perfects += 1;
            
            if perfects == parts {
                // 'values' contained 'parts' amount of perfect portions
                // e.g. sum is 100 and values contained four '25's.
                return vec![portion as u32, parts]; 
            }
        } else if (*x as f32) < portion {
            smalls.push(*x);
        } else {
            bigs.push(*x);
        }
    }

    if perfects > 0 {
        // if there are perfects recurse without perfects to account for
        // "p + b + s" type scenarios. Effectively, we save this state as
        // "p + find_addends()" which ought to return the remaining 
        // "b + s" sum. values does not need perfects removed as they
        // cannot be counted twice.

        // if perfects exist then portion is a whole number and can be
        // safely cast as u32
        let mut imperfect_values =
            find_addends(sum - perfects * portion as u32, parts - perfects, values);

        // if "p + b + s" == sum
        if imperfect_values.iter().sum::<u32>() + (perfects * portion as u32) == sum {
            let mut answer = vec![portion as u32, perfects];
            answer.append(&mut imperfect_values);
            return answer;
        }
    } else {
        let mut small_count = 1;
        // We know either b or s is at most (parts - 1). Otherwise, if b or s
        // was equal to parts then they would be perfect. Call (parts - 1) "n".
        // Therefore the addends are between nb + 1s to 1b to ns.

        while small_count < parts {

            small_count += 1;
        }
        
        // then test less bigs, more smalls, e.g. 1b + 3s
    }
    Vec::new()
}

fn main() -> io::Result<()> {
    // command-line arguments
    let file_name: String = env::args().nth(1).expect("Please provide a text file.");
    let arg2 = env::args().nth(2).expect("Please provide the desired sum.");
    let arg3 = env::args().nth(3).expect("Please provide the number of addends.");
    let sum: u32 = arg2.parse().expect("Please provide a whole number sum.");
    let parts: usize = arg3.parse().expect("Please provide a whole number of addends.");

    // file io
    let file = File::open(&file_name)?;
    let mut reader = io::BufReader::new(file);
    let mut buf = String::new();
    let mut contents: Vec<u32> = Vec::with_capacity(200);
    let mut line_number = 1;

    // setup
    let mut solution: Vec<u32> = Vec::with_capacity(parts);

    while reader.read_line(&mut buf)? > 0 {
        let line = buf.trim_end();

        if let Ok(x) = line.parse::<u32>() {
            contents.push(x);
        } else {
            println!("Error parsing value on line {}", line_number);
        };

        buf.clear();
        line_number += 1;
    } // end while read_line

    println!("solution set: {:?}, result: {}", solution, 0);

    Ok(())
}
