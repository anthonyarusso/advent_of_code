use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn count_parents(child: &str, map: &HashMap<String, String>, p_map: &mut HashMap<String, bool>) -> usize {
    let mut parent_count = 0;
    // Create a separate (permanent) variable to store the parent string
    // that parents' slices will borrow from.

    // Child is NOT an outermost bag, i.e. has parent bags
    if let Some(p) = map.get(child) {
        let parent_string: String = p.to_string();
        // Parse the string of parents into a Vector of Strings
        let parents: Vec<&str> = parent_string
            .split(';')
            .collect();

        for parent in parents.iter() {
            // If this is the first time this particular parent
            // has been accounted for, then increment.
            if p_map.insert(parent.to_string(), true).is_none() {
                parent_count += 1;
                parent_count += count_parents(parent, map, p_map);
            }
        }
    }

    parent_count
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    // <Key, Value> : <Bag Color, List of 'Parent' Bag Colors>
    let mut bags_map: HashMap<String, String> = HashMap::new();
    // <Key, Value> : <Bag Color, has_been_accounted>
    let mut accounted_parents: HashMap<String, bool> = HashMap::new();

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

       let children: Vec<String> = sundae.unwrap().1
           .trim_end()
           .replace('.', "")
           .split(", ")
           .map(|e| e.to_string())
           .collect();

       for child in children.iter() {
           // If inserting the first parent for this element, convert `parent`
           // to String and insert directly. Otherwise append ", " + parent.
           let value = bags_map.entry(child.trim_start().to_string()).or_insert(parent.to_string());
           if !(*value).contains(parent) {
               (*value).push(';');
               (*value).push_str(parent);
           }
       }
    }
    // bags_map now contains a list of every parent per child key
    // in the form: "child", "parent1;parent2;parent3"
    
    accounted_parents.reserve(bags_map.len());
    println!("shiny gold bags can be found in {} different colored bags",
             count_parents("shiny gold", &bags_map, &mut accounted_parents));

    Ok(())
}
