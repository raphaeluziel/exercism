use rand::Rng;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input = "HE + SEES + THE == LIGHT";
    let xx = input.replace(" ", "").replace("==", "=");
    let mut hm: HashMap<char, u8> = HashMap::new();

    let mut hs = HashSet::new();
    let mut r = 0;
    println!("R = {r}");
    for x in xx.char_indices().filter(|x| x.1 != '+' && x.1 != '=') {
        r = rand::rng().random_range(0..10);
        println!("R = {r}");
        while !hs.contains(&r) {
            println!("R = {r}");
            hm.insert(x.1, r);
            r = rand::rng().random_range(0..10);
            
        }
        hs.insert(r);
    }

    hm.insert(input.chars().next().unwrap_or_default(), 1);

    println!("Hash = {:#?}", hm);
    println!("Hash = {:#?}", hs);

    todo!("Solve the alphametic {input:?}")
}
