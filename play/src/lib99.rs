use std::collections::{HashMap, HashSet};

pub fn permutations(n: usize, permutation: &mut [u8; 10], hm: &mut HashMap<char, u8>) {
    if n == 1 {
        //     for (key, val) in hm.iter_mut() {
        //         let n = vs.pop().unwrap_or_default();
        //         if n == 0 && first_letters.contains(key) {
        //             continue 'outer;
        //         }
        //         *val = n;
        //     }
        for x in hm.iter_mut().enumerate() {
            //println!("JKJKJKJK = {:?}", x);
            *x.1.1 = permutation[x.0];
        }
        return;
    } else {
        permutations(n - 1, permutation, hm);

        for i in 0..(n - 1) {
            if i % 2 == 0 {
                let temp = permutation[i];
                permutation[i] = permutation[n - 1];
                permutation[n - 1] = temp;
            } else {
                let temp = permutation[0];
                permutation[0] = permutation[n - 1];
                permutation[n - 1] = temp;
            }
            permutations(n - 1, permutation, hm);
        }
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut permutation = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("PP = {:?}", permutation);

    let input = input.replace(" ", "").replace("==", "=");
    let mut hm: HashMap<char, u8> = HashMap::new();
    let mut decoded: Vec<u64> = Vec::new();
    //let mut vs: Vec<u8> = (0..10).collect();

    for x in input.chars().filter(|&x| x != '+' && x != '=') {
        hm.insert(x, 1);
    }

    let mut iterations = 0;
    let coded_nums: Vec<&str> = input.split(['+', '=']).collect();
    let first_letters: HashSet<char> = coded_nums
        .iter()
        .map(|x| x.chars().next().unwrap())
        .collect();

    // println!("original hashmap = {:?}", hm);
    // permutations(permutation.len(), &mut permutation, &mut hm);
    // println!("Now the  hashmap = {:?}", hm);
    // loop {
    //     let gh = hm.iter().find(|x| first_letters.contains(x.0) && *x.1 == 0);
    //     match gh {
    //         None => break,
    //         _ => permutations(permutation.len(), &mut permutation, &mut hm)
    //     }
    // }
    println!("Finall   hashmap = {:?}", hm);

    'outer: loop {
        decoded.clear();

        // for (key, val) in hm.iter_mut() {
        //     let n = vs.pop().unwrap_or_default();
        //     if n == 0 && first_letters.contains(key) {
        //         continue 'outer;
        //     }
        //     *val = n;
        // }

        permutations(permutation.len(), &mut permutation, &mut hm);
        loop {
            let gh = hm.iter().find(|x| first_letters.contains(x.0) && *x.1 == 0);
            println!("gh = {:?}", gh);
            match gh {
                None => break,
                _ => continue 'outer,
            }
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

    //Some(hm)
    todo!("hey")
}
