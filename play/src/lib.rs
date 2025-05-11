// Cut execution time in half by only checking numbers up to and including
// half of the given number

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 { return None; }
    if num == 1 { return Some(Classification::Deficient); }
    
    // let mut factors: Vec<u64> = vec![];
    // factors.push(1);

    // let mut i = 2;
    // while i <= num / 2 {
    //     if num % i == 0 { 
    //         factors.push(i);
    //     }
    //     i += 1;
    // }

    // let sum: u64 = factors.iter().sum();

    // match sum.cmp(&num) {
    //     std::cmp::Ordering::Equal => Some(Classification::Perfect),
    //     std::cmp::Ordering::Greater => Some(Classification::Abundant),
    //     std::cmp::Ordering::Less => Some(Classification::Deficient),
    // }
    
    const PRIMES: [u64; 8] = [2, 3, 5, 7, 13, 17, 19, 31];

    pub fn is_prime(num: u64) -> bool {
        if num < 2 { return false; }
        let mut x = 2;
        let limit = (num as f64).sqrt() as u64;
        while x <= limit {
            if num % x == 0 { return false; }
            x += 1;
        }
        true
    }

    // let mut p: u32 = 2;
    // let mut mersenne: u64 = 2u64.pow(p) - 1;

    let mut p = 2u32;

    while p < 70 {
        let mersenne = 2u64.pow(p) - 1;
        if is_prime(mersenne) { println!("p = {p} is prime, and makes mersenne = {mersenne}"); }
        p += 1;
        if num == mersenne * (mersenne + 1) / 2 { break; }
    }

    if p == 70 { return  None; }
    else { return Some(Classification::Perfect); }

    //todo!()
}


// https://en.wikipedia.org/wiki/List_of_Mersenne_primes_and_perfect_numbers