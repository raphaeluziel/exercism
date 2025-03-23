use std::collections::{BTreeMap, HashMap, HashSet};

pub fn permutations(n: usize, permutation: &mut [u8; 10], bm: &mut BTreeMap<char, u8>) {
    if n == 1 {
        //bm.iter_mut().enumerate().map(|(i, (k, v))| *v = 9).into_iter().collect::<BTreeMap<char, u8>>();
        for x in bm.iter_mut().enumerate() {
            *x.1.1 = permutation[x.0];
        }
        return;
    } else {
        permutations(n - 1, permutation, bm);

        for i in 0..(n - 1) {
            if i % 2 == 0 {
                permutation.swap(i, n - 1);
            } else {
                permutation.swap(0, n - 1);
            }
            permutations(n - 1, permutation, bm);
        }
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input = input.replace(" ", "").replace("==", "=");
    let mut permutation = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let coded_nums: Vec<&str> = input.split(['+', '=']).collect();
    let first_letters: HashSet<char> = coded_nums.iter().map(|x| x.chars().next().unwrap()).collect();
    let mut decoded: Vec<u64> = Vec::new();

    let mut bm:BTreeMap<char, u8> = input.chars().filter(|&x| x != '+' && x != '=').map(|k| (k, 2u8)).collect();
    //println!("BM1 = {:?}", bm);
    permutations(bm.len(), &mut permutation, &mut bm);
    //println!("BM2 = {:?}", bm);
    

    Some(bm.clone().into_iter().map(|(k, v)| (k, v)).collect())
}
