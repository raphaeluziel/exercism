use std::{collections::{HashMap, HashSet}, iter::Inspect};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let xx = input.replace(" ", "").replace("==", "=");
    let mut hm: HashMap<char, u8> = HashMap::new();
    let mut vs: Vec<u8> = Vec::new();
    let mut decoded: Vec<u32> = Vec::new();

    for x in xx.chars().filter(|&x| x != '+' && x != '=') {
        hm.insert(x, 1);
    }

    let mut iterations = 0;
    loop {
        decoded.clear();
         
        vs = HashSet::from([0, 1, 9])
            .into_iter()
            .collect();
    
        for (_, val) in hm.iter_mut() {
            *val = vs.pop().unwrap_or_default();
        }
    
        let coded_nums: Vec<&str> = xx.split(['+', '=']).collect();
    
        for alph in coded_nums {
            let rrr: u32 = alph
                .chars().rev()
                .enumerate()
                .map(|x| (*hm.get(&x.1).unwrap() as u32) * 10u32.pow(x.0 as u32))
                .sum();
            decoded.push(rrr);
        }

        let dede = decoded.pop().unwrap();
        if dede == decoded.iter().sum() { println!("FOUND!"); break; }
        
        iterations += 1;
        if iterations > 1000000 { return None; }
    }

    println!("\n-------------------------\nDecoded = {:?}", decoded);
    println!("Iterations = {iterations}");
    println!("HashMao = {:?}", hm);

    Some(hm)

    //todo!("Solve the alphametic {input:?}")
}
