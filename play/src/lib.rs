use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {

    let mut hm: HashMap<char, u8> = HashMap::new();
    for x in input.chars().filter(|&x| x != '+' && x != '=' && x != ' ') {
        hm.insert(x, 1);
    }
    println!("hash map = {:#?}", hm);

    let input = input.replace(" ", "").replace("==", "=");
    let input: Vec<_> = input
        .split(['+', '='])
        .map(|x| x.chars().rev().collect::<String>())
        .collect();
    println!("Modified input as vector: {:?}", input);

    //println!("first letters = {:?}", first_letters);

    Some(hm)
}
