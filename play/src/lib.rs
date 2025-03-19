use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // "HE + SEES + THE == LIGHT"

    let mut hm: HashMap<char, u8> = HashMap::new();
    // for x in input.chars().filter(|&x| x != '+' && x != '=' && x != ' ') {
    //     hm.insert(x, 1);
    // }
    // println!("hash map = {:#?}", hm);

    let input = input.replace(" ", "").replace("==", "=");

    let first_letters: HashSet<char> = input
        .split(['+', '='])
        .map(|x| x.chars().next().unwrap_or_default())
        .collect();
    //println!("First letters = {:?}", first_letters);

    let mut input: Vec<Vec<char>> = input
        .split(['+', '='])
        .map(|x| x.chars().rev().collect::<Vec<char>>())
        .collect();

    let longest = input.iter().map(|x| x.len()).max().unwrap_or_default();
    //println!("longest = {longest}");

    for v in &mut input {
        v.resize(longest, '.');
    }

    let summ = input.pop().unwrap_or_default();
    //println!("summ = {:?}", summ);

    //println!("Modified input as vector: {:?}", input);

    let mut digits: Vec<u8> = (0..10).collect();
    println!("Digits = {:?}", digits);

    let mut carry = 0;
    let mut col = 0;

    //println!("Input length = {:?}", input.len());

    for col in 0..longest {
        for row in 0..input.len() {
            println!("({row}, {col}) ==>{:?}", input[row][col]);
            if !hm.contains_key(&input[row][col]) && input[row][col] != '.' {
                if first_letters.contains(&input[row][col]) && digits[0] == 0 {
                    hm.insert(input[row][col], digits.remove(1));
                    println!("A {:?}", hm);
                    println!("digits = {:?}", digits);
                } else {
                    hm.insert(input[row][col], digits.remove(0));
                    println!("A {:?}", hm);
                    println!("digits = {:?}", digits);
                }
            }
        }
        println!("summ = {:?}\n", summ[col]);
    }

    println!("hash map = {:#?}", hm);
    println!("digits = {:?}", digits);

    Some(hm)
}
