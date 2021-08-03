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
    let mut bags_map: HashMap<String, Bag> = HashMap::new();

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

        // Parse the 'parent' bag's color name.
        let parent = sundae.0.trim_end_matches(" bags").trim_end_matches(" bag");

        // Parse the 'child' bags' color names.
        let children: Vec<String> = sundae.1
            .trim_end_matches('.')
            .split(", ")
            .map(|s| s
                 .trim_start_matches(char::is_numeric)
                 .trim_start()
                 .trim_end_matches(" bags")
                 .trim_end_matches(" bag")
                 .to_string())
            .collect();

        // Split the 'children string' along spaces, then only
        // keep substrings that are entirely numerics.
        let mut amounts: Vec<&str> = sundae.1
            .split(' ')
            .collect();
        amounts.retain(|s| s.chars().all(char::is_numeric));
        let amounts: Vec<u32> = amounts
            .iter()
            .map(|s| {
                if s.parse::<u32>().is_err() {
                    panic!("Failed to parse numerical string as integer!");
                }
                s.parse::<u32>().unwrap()
            })
            .collect();

        // Insert 'child' bags with only their color name in an
        // owned String. This allows the 'parent' bag to borrow
        // from these Strings with proper lifetimes (compared to
        // `line`'s relatively short lifetime in this for loop.
        for child in children.iter() {
            bags_map.insert(child.to_string(), Bag::new(child));
        }
        let mut parent_bag = Bag::new(parent);

        for (i, child) in children.iter().enumerate() {
            if bags_map.get(child).is_none() {
                panic!("Hashmap does not contain key {}!", child);
            }
            // Bag contains no other bags. Skip inserting children.
            if child.contains("no other") {
                break;
            }
            if amounts.get(i).is_none() {
                panic!("Bag color - Bag Amount mismatch!");
            }
            parent_bag.push(&bags_map.get(child).unwrap().color, *amounts.get(i).unwrap());
        }
        // map inserts using a reference to the String as a key
        // if the bag doesnt exist in the map then the color field
        // of the Bag struct permanently stores the color's name.
        // If the bag does exist in map then update the Bag's
        // contents field and/or set other Bags' contents to
        // borrow from the Bag's color field.
    }

    Ok(())
}
