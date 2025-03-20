use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input = input.replace(" ", "").replace("==", "=");
    let mut hm: HashMap<char, u8> = HashMap::new();
    let mut decoded: Vec<u64> = Vec::new();
    // let mut trials: Vec<HashMap<char, u8>> = Vec::new();

    for x in input.chars().filter(|&x| x != '+' && x != '=') {
        hm.insert(x, 1);
    }

    let mut iterations = 0;
    let coded_nums: Vec<&str> = input.split(['+', '=']).collect();
    let first_letters: HashSet<char> = coded_nums
        .iter()
        .map(|x| x.chars().next().unwrap())
        .collect();

    'outer: loop {
        decoded.clear();

        let mut vs: Vec<u8> = HashSet::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
            .into_iter()
            .collect();

        for (key, val) in hm.iter_mut() {
            let n = vs.pop().unwrap_or_default();
            if n == 0 && first_letters.contains(key) {
                continue 'outer;
            }
            *val = n;
        }

        for letter in &coded_nums {
            decoded.push(
                letter
                    .chars()
                    .rev()
                    .enumerate()
                    .map(|x| (*hm.get(&x.1).unwrap() as u64) * 10u64.pow(x.0 as u32))
                    .sum(),
            );
        }

        let dede = decoded.pop().unwrap();
        if dede == decoded.iter().sum() {
            break;
        }

        iterations += 1;
    }
    println!("iterations = {iterations}");

    Some(hm)
}
