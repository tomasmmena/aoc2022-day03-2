use std::fs;
use std::env;
use std::io::{self, BufRead};

use std::collections::HashSet;


fn find_error(value: &[String]) -> usize {
    let overlap = value.into_iter().map(| rucksack | {
        let mut set = HashSet::new();
        for c in rucksack.chars() {
            set.insert(c);
        }
        set
    }).reduce(|first, second| {
        &first & &second
    }).unwrap_or(HashSet::new());

    let mut accumulator: usize = 0;

    let priorities = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for c in overlap.into_iter() {
        accumulator += priorities.chars().position(|p| p == c).unwrap();
    }
    accumulator
}


fn main() {
    let path = env::args().nth(1).expect("No route was provided.");

    let data = Vec::from_iter(
        io::BufReader::new(
            fs::File::open(path)
                .expect("Could not read file"))
            .lines()
            .map(|line| line.expect("msg")
        )
    );

    let mut accumulator: usize = 0;
    for i in (0..data.len()).step_by(3) {
        accumulator += find_error(&data[i..i + 3])
    }

    println!("Sum of errors: {}", accumulator);
}


#[test]
fn test_find_error(){
    let data = vec![
        String::from("aaBBCAAbbC"), 
        String::from("CC"), 
        String::from("aabbCC")
    ];
    assert!(find_error(&data[0..2]) == 29);
}