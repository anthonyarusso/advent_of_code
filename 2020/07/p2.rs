use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

type IndexSlice = (usize, usize);

#[derive(Default)]
struct Bag {
    color: IndexSlice,
    // Vector of tuples. (Pointer, Amount)
    contents: Vec<(IndexSlice, u32)>,
}
impl Bag {
    pub fn new(color: IndexSlice) -> Self {
        Self {
            color: color,
            ..Default::default()
        }
    }
    pub fn get_color(&self) -> IndexSlice {
        self.color
    }
    pub fn push(&mut self, color: IndexSlice, amount: u32) {
        self.contents.push((color, amount));
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut store = String::new();
    let mut map: HashMap<IndexSlice, Bag> = HashMap::new();

    for line in f.lines() {
        let line = line.expect("Error reading from file!");
        let sundae = line.split_once(" contain ").expect("Failed to split line!");
        let parent = sundae.0.trim_end_matches(" bags").trim_end_matches(" bag");
        
        let children: Vec<&str> = sundae.1
            .trim_end_matches('.')
            .split(", ")
            .map(|s| s
                 .trim_start_matches(char::is_numeric)
                 .trim_start()
                 .trim_end_matches(" bags")
                 .trim_end_matches(" bag"))
            .collect();

        let mut amounts: Vec<&str> = sundae.1
            .split(' ')
            .collect();
        amounts.retain(|s| s.chars().all(char::is_numeric));
        let amounts: Vec<u32> = amounts
            .iter()
            .map(|s| {s.parse::<u32>().expect("Failed to parse integer!") })
            .collect();

        let p_start = store.len();
        store.push_str(parent);
        let p_end = store.len();
        store.push(';');

        // If the parent already exists in map, get a pointer to the pre-existing
        // value. Otherwise, insert a new Bag an use its pointer.
        let parent_bag = Bag::new((p_start, p_end));
        let parent_bag = map.entry((p_start, p_end)).or_insert(parent_bag);

        // Not every child bag was given an amount. Either the input file
        // contains errors or lists the parent as containing "no other bags".
        let mut no_other_bags = false;
        if children.len() != amounts.len() {
            if sundae.1 == "no other bags." {
                no_other_bags = true;
            } else {
                panic!("Bags-Amounts mismatch!");
            }
        }

        if !no_other_bags {
            for child in children.iter() {
                let start;
                let end;
                if store.contains(child) {
                    start = store.find(child).expect("Cannot find substring!");
                    end = start + child.len();
                } else {
                    start = store.len();
                    store.push_str(child);
                    end = store.len();
                    store.push(';');
                }
                // Preemptively insert (empty) child bags to map.
                // NOTE: I guess the input.txt actually lists every possible
                // type of bag as a "parent" at least once so this step is
                // unnecessary. If it turns out this is not true, find a way
                // to implement this.
                // map.insert((start, end), Bag::new((start, end)));

                // Update the parent bag's map entry with the child.
                parent_bag.push((start, end), 0);
            }
        }
    }

    Ok(())
}
