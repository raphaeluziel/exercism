mod prime;
use prime::*;

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut prime_count: u64 = 0;
    let mut prime_factor;
    let mut left = n;

    while !is_prime(left) && left != 1 {
        prime_factor = nth(prime_count);
        println!("LEFT {}, PRIME_COUNT {} PRIME_FACTOR {}", left, prime_count, prime_factor);
        println!("BBB {}", prime_factor);
        while left % prime_factor == 0 {
            factors.push(prime_factor);
            left = left / prime_factor;
        }
        prime_count += 1;
        factors.push(left);
    }

    factors
}
