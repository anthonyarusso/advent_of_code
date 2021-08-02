use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn count_parents(child: &str, map: &HashMap<String, String>) -> usize {
    let mut parent_count = 0;
    // Parse the string of parents into a Vector of Strings
    let parents: Vec<String> = map
        .get(child)
        .unwrap()
        .to_string()
        .split(';')
        .map(|e| e.to_string())
        .collect();

    parent_count += parents.len();

    for parent in parents.iter() {
        parent_count += count_parents(parent, map);
    }
    parent_count
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    // <Key, Value> : <Bag Color, List of 'Parent' Bag Colors>
    let mut bags_map: HashMap<String, String> = HashMap::new();

    for line in f.lines() {
       let line = line
           .unwrap()
           .to_string()
           .replace(" bags", "")
           .replace(" bag", "")
           .replace(char::is_numeric, "")
           .replace("  ", " ");

       // let mut children: Vec<String> = Vec::new();

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

    Ok(())
}
