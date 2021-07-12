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
// 2^(ROW_COUNT + COL_COUNT)
const TOTAL_SEATS: usize = 1024;
// The number of actual seats in a column (2^COL_COUNT)
const COL_SEATS: usize = 8;

fn read_pass(line: &str) -> (u32, u32) {
    let mut row = 0;
    let mut col = 0;

    // Input is not ten characters long
    if line.len() != ROW_COUNT + COL_COUNT {
        panic!("Invalid input!");
    }

       for (i, letter) in line.chars().enumerate() {
           match letter {
               // Match row
               FRONT_CHAR | BACK_CHAR => {
                   // Found outside the first seven characters
                   if i >= ROW_COUNT {
                        panic!("Invalid input!");
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
                        panic!("Invalid input!");
                   }
                   if letter == RIGHT_CHAR {
                       // See above
                       col += 2_u32.pow((COL_COUNT - 1 - (i - ROW_COUNT)) as u32);
                   }
               },
               _  => {
                    panic!("Invalid input!");
               },
           }
       }

    (row, col)
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut taken_seats = [false; TOTAL_SEATS];
    let mut your_seat_id = 0;

    for line in f.lines() {
       let line = line.unwrap().to_string();
       let (row, col) = read_pass(&line);
       let seat_id = row * COL_SEATS as u32 + col;

       if seat_id >= TOTAL_SEATS as u32 {
           panic!("seat_id out of bounds!");
       }

        taken_seats[seat_id as usize] = true;
    } // END `for line`

    // Check every empty seat to see if its neighbors are taken
    for (i, is_taken) in taken_seats.iter().enumerate() {
        if !is_taken {
            println!("seat {} is not taken!", i);
            if i != 0 && taken_seats[i - 1] && taken_seats[i + 1] {
                your_seat_id = i;
            }
        }
    }

    println!("seat info:\nid: {}\nrow: {}\n col: {}",
             your_seat_id,
             your_seat_id / 8,
             your_seat_id % 8);

    Ok(())
}
