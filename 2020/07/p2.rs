use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

#[derive(Default)]
struct Bag<'a> {
    color: String,
    // Vector of tuples. (Pointer, Amount)
    contents: Vec<(&'a str, u32)>,
}
impl<'a> Bag<'a> {
    pub fn new(color: &str) -> Self {
        Self {
            color: color.to_string(),
            ..Default::default()
        }
    }
    pub fn get_color(&self) -> &str {
        &self.color
    }
    pub fn push(&mut self, color: &'a str, amount: u32) {
        self.contents.push((color, amount));
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    // <Key, Value> : <Bag Color, List of Contents>
    let mut bags_map: HashMap<&str, Bag> = HashMap::new();

    for line in f.lines() {
        // line creates a String containing the bag names
        if line.is_err() {
            panic!("Error reading line!");
        }
        let line = line.unwrap();

        if line.split_once(" contain ").is_none() {
            panic!("Error: Cannot locate substring!");
        }
        let sundae = line.split_once(" contain ").unwrap();
        let parent = sundae.0.trim_end_matches("bags").trim_end_matches("bag");
        let children: Vec<&str> = sundae.1
            .trim_end_matches('.')
            .split(", ")
            .map(|s| s
                 .trim_start_matches(char::is_numeric)
                 .trim_start()
                 .trim_end_matches(" bags")
                 .trim_end_matches(" bag"))
            .collect();


        print!("{} --- ", parent);
        for child in children.iter() {
            print!("{};", child);
        }
        print!("\n");

        // map inserts using a reference to the String as a key
        // if the bag doesnt exist in the map then the color field
        // of the Bag struct permanently stores the color's name.
        // If the bag does exist in map then update the Bag's
        // contents field and/or set other Bags' contents to
        // borrow from the Bag's color field.
    }

    Ok(())
}
