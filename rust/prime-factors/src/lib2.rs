mod prime;
use prime::*;

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut prime_count: u64 = 0;
    let mut prime_factor;
    let mut left = n;

    while !is_prime(left) && left != 1 {
        prime_factor = nth(prime_count);
        while left % prime_factor == 0 {
            factors.push(prime_factor);
            left /= prime_factor;
        }
        prime_count += 1;
        
    }
    if left != 1 { factors.push(left); }

    factors
}