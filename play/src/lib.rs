use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input = "HE + SEES + THE == LIGHT";
    let xx = input.replace(" ", "").replace("==", "=");
    let mut hm: HashMap<char, u8> = HashMap::new();
    let mut vs:Vec<u8> = HashSet::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).into_iter().collect();

    println!("VECSET = {:?}", vs);

    for x in xx.chars().filter(|&x| x != '+' && x != '=') {
        hm.insert(x, 1);
    }

    for (jjj, val) in hm.iter_mut() {
        println!("KIH {:?}", jjj);
        *val = vs.pop().unwrap_or_default();
    }


    

    println!("Hash = {:#?}", hm);
    println!("VECSET = {:?}", vs);

    todo!("Solve the alphametic {input:?}")
}
