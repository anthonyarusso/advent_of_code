fn main() {
    let mut store = String::new();
    let mut slices: Vec<(usize, usize)> = Vec::new();
    
    for i in 0..5 {
        let s;
        match i {
            0 => { s = "zero".to_string(); },
            1 => { s = "one".to_string(); },
            2 => { s = "two".to_string(); },
            3 => { s = "three".to_string(); },
            4 => { s = "four".to_string(); },
            _ => { s = "wtf".to_string(); },
        }
        if i != 0 {
            store.push(';');
        }
        let start = store.len();
        store.push_str(&s);
        let end = store.len();

        let range = (start, end);
        slices.push(range);
    }
    println!("store: {}", store);
    for slice in slices {
        println!("{}", &store[slice.0..slice.1]);
    }
}
