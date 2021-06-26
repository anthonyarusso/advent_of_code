use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let file = File::open("1_input.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut buf = String::new();

    let mut smalls: Vec<i32> = Vec::with_capacity(30);
    let mut bigs: Vec<i32> = Vec::with_capacity(200);

    let mut line_number = 1;

    while reader.read_line(&mut buf)? > 0 {
        let line = buf.trim_end();

        if let Ok(x) = line.parse::<i32>() {
            if x <= 673 {
                smalls.push(x);
            } else {
                bigs.push(x);
            }
        } else {
            println!("error parsing value on line {}", line_number);
        }

        buf.clear();
        line_number += 1;
    } // end while lines

    let mut solution: (i32, i32, i32) = (0, 0, 0);

    for (i, x) in smalls.iter().enumerate() {
        // first, try 2 small + 1 big
        for y in smalls.get((i + 1)..).unwrap().iter() {
            for z in bigs.iter() {
                if *z == 2020 - (x + y) {
                    solution = (*x, *y, *z);    
                    break;
                }
            }

            if solution != (0, 0, 0) {
                break;
            }
        } // end for y

        // if still not solved, try 1 small + 2 big
        if solution == (0, 0, 0) {
            for (i, y) in bigs.iter().enumerate() {
                for z in bigs.get((i + 1)..).unwrap().iter() {
                    if *z == 2020 - (x + y) {
                        solution = (*x, *y, *z);
                        break;
                    }
                }

                if solution != (0, 0, 0,) {
                    break;
                }
            }
        }
    } // end for x

    println!("solution set: {:?}, result: {}", solution, solution.0 * solution.1 * solution.2);
    println!("smalls.len(): {}", smalls.len());
    println!("bigs.len(): {}", bigs.len());
    Ok(())
}
