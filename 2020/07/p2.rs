use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn count_children(parent: &str, map: &HashMap<String, String>, c_map: &mut HashMap<String, bool>) -> usize {
    let mut child_count = 0;
    // Create a separate (permanent) variable to store the parent string
    // that parents' slices will borrow from.

    // Child is NOT an outermost bag, i.e. has parent bags
    if let Some(p) = map.get(parent) {
        let child_string: String = p.to_string();
        // Parse the string of parents into a Vector of Strings
        let children: Vec<&str> = child_string
            .split(';')
            .collect();

        for child in children.iter() {
            // If this is the first time this particular parent
            // has been accounted for, then increment.
            if c_map.insert(child.to_string(), true).is_none() {
                child_count += 1;
                child_count += count_children(child, map, c_map);
            }
        }
    }

    child_count
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    // <Key, Value> : <Bag Color, List of 'Parent' Bag Colors>
    let mut bags_map: HashMap<String, String> = HashMap::new();
    // <Key, Value> : <Bag Color, has_been_accounted>
    let mut accounted_children: HashMap<String, bool> = HashMap::new();

    for line in f.lines() {
       let line = line
           .unwrap()
           .to_string()
           .replace(" bags", "")
           .replace(" bag", "")
           .replace(char::is_numeric, "")
           .replace("  ", " ");

       let sundae = line.split_once("contain");
       let parent = sundae.unwrap().0.trim_end();

       let children: String = sundae.unwrap().1
           .trim_end()
           .replace('.', "")
           .replace(", ", ";");

       bags_map.insert(parent.to_string(), children);
    }

    // bags_map now contains a list of every parent per child key
    // in the form: "child", "parent1;parent2;parent3"
    
    accounted_children.reserve(bags_map.len());
    println!("shiny gold bags contain {} other bags",
             count_children("shiny gold", &bags_map, &mut accounted_children));

    Ok(())
}
