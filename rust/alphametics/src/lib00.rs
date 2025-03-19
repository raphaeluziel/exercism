use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // "HE + SEES + THE == LIGHT"

    let mut hm: HashMap<char, u8> = HashMap::new();
    // for x in input.chars().filter(|&x| x != '+' && x != '=' && x != ' ') {
    //     hm.insert(x, 1);
    // }
    println!("hash map = {:#?}", hm);

    let input = input.replace(" ", "").replace("==", "=");

    let first_letters: HashSet<char> = input
        .split(['+', '='])
        .map(|x| x.chars().next().unwrap_or_default())
        .collect();
    println!("First letters = {:?}", first_letters);

    let mut input: Vec<Vec<char>> = input
        .split(['+', '='])
        .map(|x| x.chars().rev().collect::<Vec<char>>())
        .collect();

    let longest = input.iter().map(|x| x.len()).max().unwrap_or_default();
    println!("longest = {longest}");

    for v in &mut input {
        v.resize(longest, '.');
    }

    println!("Modified input as vector: {:?}", input);

    let digits: Vec<u8> = (0..10).collect();
    println!("Digits = {:?}", digits);

    let mut carry = 0;

    Some(hm)
}
