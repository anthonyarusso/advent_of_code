use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashMap, HashSet};

// This approach borrows from a single 'main' String type.
// String slices can be generated by using a start and
// end index and this main string. This only works because
// new data is always added to the string, never removed.

// The bag color whose 'parents' and 'children' will be counted.
const CHOSEN_BAG: &'static str = "shiny gold";
type IndexSlice = (usize, usize);

#[derive(Default)]
struct Bag {
    // Tracks all bags which directly contain this one.
    holders: Vec<IndexSlice>,
    // Tracks all bags directly contained in this one, and
    // how many of each type are required.
    contents: Vec<(IndexSlice, u32)>,
}

fn get_color_from_store(color: &str, store: &str) -> IndexSlice {
    let start = store.find(color).expect("Could not find substring in store!");
    let end = start + color.len();
    (start, end)
}

// Counts how many unique colored bags will eventually contain `color`.
fn count_unique_ancestors(
    color: IndexSlice,
    map: &HashMap<IndexSlice, Bag>,
    tracker: &mut HashSet<IndexSlice>,
    ) -> u32
{
    let mut total = 0;
    let child = map.get(&color).expect("Unable to locate key in map!");
    
    for parent in child.holders.iter() {
        if tracker.insert(*parent) {
            total += 1;
        }
        total += count_unique_ancestors(*parent, map, tracker);
    }
    total
}

// Counts the total number of bags within one `color` bag regardless
// of their color. E.g. 3 red + 10 blue = 13 total descendants.
fn count_total_descendants(color: IndexSlice, map: &HashMap<IndexSlice, Bag>) -> u32 {
    let mut total = 0;
    let parent = map.get(&color).expect("Unable to locate key in map!");
    for (child, amount) in parent.contents.iter() {
        total += amount;
        total += amount * count_total_descendants(*child, map);
    }
    total
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut store = String::new();
    let mut map: HashMap<IndexSlice, Bag> = HashMap::new();

    for line in f.lines() {
        let line = line.expect("Error reading from file!");
        let sundae = line.split_once(" contain ").expect("Failed to split line!");
        // Parse the name of the 'parent' color listed.
        let parent = sundae.0.trim_end_matches(" bags").trim_end_matches(" bag");
        
        // Parse the names of the 'child' colors.
        let children: Vec<&str> = sundae.1
            .trim_end_matches('.')
            .split(", ")
            .map(|s| s
                 .trim_start_matches(char::is_numeric)
                 .trim_start()
                 .trim_end_matches(" bags")
                 .trim_end_matches(" bag"))
            .collect();

        // Parse the amount of each child contained in parent.
        let mut amounts: Vec<&str> = sundae.1
            .split(' ')
            .collect();
        amounts.retain(|s| s.chars().all(char::is_numeric));
        let amounts: Vec<u32> = amounts
            .iter()
            .map(|s| { s.parse::<u32>().expect("Failed to parse integer!") })
            .collect();

        // Insert the 'parent' color to store, or, if it exists,
        // only retreive its indices from store.
        let p_start;
        let p_end;
        if let Some(ps) = store.find(parent) {
            p_start = ps;
            p_end = p_start + parent.len();
        } else {
            p_start = store.len();
            store.push_str(parent);
            p_end = p_start + parent.len();
            store.push(';');
        }

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

        // Push all child colors to store and save their indices
        // to c_contents.
        let mut c_contents: Vec<(IndexSlice, u32)> = Vec::new();
        if !no_other_bags {
            for (i, child) in children.iter().enumerate() {
                let start;
                let end;
                if let Some(cs) = store.find(child) {
                    start = cs;
                    end = start + child.len();
                } else {
                    start = store.len();
                    store.push_str(child);
                    end = store.len();
                    store.push(';');
                }
                c_contents.push((
                    (start, end),
                    *amounts.get(i).expect("Amount not found!")));
            }
        }
        // If the parent already exists in map, get a pointer to the pre-existing
        // value. Otherwise, insert a new Bag an use its pointer.
        let parent_bag = map.entry((p_start, p_end)).or_insert(Bag::default());

        // Push child bags to parent's 'content' vector.
        for (slice, amount) in c_contents.iter() {
            parent_bag.contents.push((*slice, *amount));
        }

        // Push the parent bag to each child's `holders` vector.
        for (c_slice, _) in c_contents.iter() {
            let c_bag = map.entry((c_slice.0, c_slice.1)).or_insert(Bag::default());
            c_bag.holders.push((p_start, p_end));
        }

    } // end 'for line in lines'

    // Declare a HashSet to track which 'unique' parents have already
    // been tracked by the `count_unique_ancestors` function.
    let mut tracker: HashSet<IndexSlice> = HashSet::new();

    let bag_of_interest = get_color_from_store(CHOSEN_BAG, &store);
    println!("{} has {} unique ancestors.",
             CHOSEN_BAG,
             count_unique_ancestors(bag_of_interest, &map, &mut tracker));

    println!("Each {} bag contains {} other bags in total.",
             CHOSEN_BAG,
             count_total_descendants(bag_of_interest, &map));

    Ok(())
}