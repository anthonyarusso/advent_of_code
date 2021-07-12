use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

const FRONT_CHAR: char = 'F';
const BACK_CHAR: char = 'B';
const LEFT_CHAR: char = 'L';
const RIGHT_CHAR: char = 'R';
// The number of characters that represent a row or column
const ROW_COUNT: usize = 7;
const COL_COUNT: usize = 3;
// The number by which the row number is multiplied when
// generating the seat ID.
const ROW_MULTIPLE: u32 = 8;

fn read_pass(line: &str) -> Result<(u32, u32), ()> {
    let mut row = 0;
    let mut col = 0;

    // Input is not ten characters long
    if line.len() != ROW_COUNT + COL_COUNT {
        return Err(())
    }

       for (i, letter) in line.chars().enumerate() {
           match letter {
               // Match row
               FRONT_CHAR | BACK_CHAR => {
                   // Found outside the first seven characters
                   if i >= ROW_COUNT {
                       return Err(());
                   }
                   if letter == BACK_CHAR {
                       // Increase row number by half the possible rows.
                       // E.g. if it is in the back half of the entire plane
                       // then it is at least 128/2 or 63 (minus 1 for 0 index).
                       row += 2_u32.pow((ROW_COUNT - 1 - i) as u32);
                   }
               },
               // Match column
               RIGHT_CHAR | LEFT_CHAR => {
                   // Found outside the last three characters
                   if i < ROW_COUNT {
                       return Err(());
                   }
                   if letter == RIGHT_CHAR {
                       // See above
                       col += 2_u32.pow((COL_COUNT - 1 - (i - ROW_COUNT)) as u32);
                   }
               },
               _  => {
                   return Err(());
               },
           }
       }

    Ok((row, col))
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut highest = 0;
    let mut high_row = 0;
    let mut high_col = 0;

    for (line_count, line) in f.lines().enumerate() {
       let line = line.unwrap().to_string();
       let (row, col) = read_pass(&line).unwrap_or_else(|_| {
           println!("invalid input on line {}", line_count);
           (0, 0)
       });
       let seat_id = row * ROW_MULTIPLE + col;
       if seat_id > highest {
           highest = seat_id;
           high_row = row;
           high_col = col;
       }
    }
    println!("highest: {}, row: {}, col: {}", highest, high_row, high_col);

    Ok(())
}
